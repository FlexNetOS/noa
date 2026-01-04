import * as path from 'path';
import { promises as fs } from 'fs';
import * as FileSystemUtils from './FileSystemUtils';
import { concatenateRules } from './RuleProcessor';
import { loadconfigs, Loadedconfigs, IAgentconfigs } from './configsLoader';
import { updateGitignore as updateGitignoreUtil } from './GitignoreUtils';
import { IAgent } from '../agents/IAgent';
import { mergeMcp } from '../mcp/merge';
import { getNativeMcpPath, readNativeMcp, writeNativeMcp } from '../paths/mcp';
import { propagateMcpToOpenHands } from '../mcp/propagateOpenHandsMcp';
import { propagateMcpToOpenCode } from '../mcp/propagateOpenCodeMcp';
import { getAgentOutputPaths } from '../agents/agent-utils';
import { agentSupportsMcp, filterMcpconfigsForAgent } from '../mcp/capabilities';
import
  {
    createRulerError,
    logVerbose,
    logVerboseInfo,
    logInfo,
    logWarn,
  } from '../constants';
import { McpStrategy } from '../types';

/**
 * configsuration data loaded from the ruler setup
 */
export interface Rulerconfigsuration
{
  configs: Loadedconfigs;
  concatenatedRules: string;
  rulerMcpJson: Record<string, unknown> | null;
}

/**
 * configsuration data for a specific .ruler directory in hierarchical mode
 */
export interface HierarchicalRulerconfigsuration extends Rulerconfigsuration
{
  rulerDir: string;
}

export /**
 * Loads configsurations for all .ruler directories in hierarchical mode.
 * Each .ruler directory gets its own independent configsuration with separate rules.
 * @param projectRoot Root directory of the project
 * @param configsPath Optional custom configs path
 * @param localOnly Whether to search only locally for .ruler directories
 * @returns Promise resolving to array of hierarchical configsurations
 */
  async function loadNestedconfigsurations (
    projectRoot: string,
    configsPath: string | undefined,
    localOnly: boolean,
    resolvedNested: boolean,
  ): Promise<HierarchicalRulerconfigsuration[]>
{
  const { dirs: rulerDirs } = await findRulerDirectories(
    projectRoot,
    localOnly,
    true,
  );

  const results: HierarchicalRulerconfigsuration[] = [];
  const rulerDirconfigss = await processIndependentRulerDirs( rulerDirs );

  for ( const { rulerDir, files } of rulerDirconfigss )
  {
    const configs = await loadconfigsForRulerDir(
      rulerDir,
      configsPath,
      resolvedNested,
    );
    results.push(
      await createHierarchicalconfigsuration(
        rulerDir,
        files,
        configs,
        configsPath,
      ),
    );
  }

  return results;
}

/**
 * Processes each .ruler directory independently, returning configsuration for each.
 * Each .ruler directory gets its own rules (not merged with others).
 */
async function processIndependentRulerDirs (
  rulerDirs: string[],
): Promise<
  Array<{ rulerDir: string; files: { path: string; content: string; }[]; }>
>
{
  const results: Array<{
    rulerDir: string;
    files: { path: string; content: string; }[];
  }> = [];

  // Process each .ruler directory independently
  for ( const rulerDir of rulerDirs )
  {
    const files = await FileSystemUtils.readMarkdownFiles( rulerDir );
    results.push( { rulerDir, files } );
  }

  return results;
}

async function createHierarchicalconfigsuration (
  rulerDir: string,
  files: { path: string; content: string; }[],
  configs: Loadedconfigs,
  cliconfigsPath: string | undefined,
): Promise<HierarchicalRulerconfigsuration>
{
  await warnAboutLegacyMcpJson( rulerDir );

  const concatenatedRules = concatenateRules( files, path.dirname( rulerDir ) );

  const directoryRoot = path.dirname( rulerDir );
  const localconfigsPath = path.join( rulerDir, 'ruler.toml' );
  let configsPathToUse = cliconfigsPath;
  try
  {
    await fs.access( localconfigsPath );
    configsPathToUse = localconfigsPath;
  } catch
  {
    // fall back to CLI configs or default resolution
  }

  const { loadUnifiedconfigs } = await import( './UnifiedconfigsLoader' );
  const unifiedconfigs = await loadUnifiedconfigs( {
    projectRoot: directoryRoot,
    configsPath: configsPathToUse,
  } );

  let rulerMcpJson: Record<string, unknown> | null = null;
  if ( unifiedconfigs.mcp && Object.keys( unifiedconfigs.mcp.servers ).length > 0 )
  {
    rulerMcpJson = {
      mcpServers: unifiedconfigs.mcp.servers,
    };
  }

  return {
    rulerDir,
    configs,
    concatenatedRules,
    rulerMcpJson,
  };
}

async function loadconfigsForRulerDir (
  rulerDir: string,
  cliconfigsPath: string | undefined,
  resolvedNested: boolean,
): Promise<Loadedconfigs>
{
  const directoryRoot = path.dirname( rulerDir );
  const localconfigsPath = path.join( rulerDir, 'ruler.toml' );

  let hasLocalconfigs = false;
  try
  {
    await fs.access( localconfigsPath );
    hasLocalconfigs = true;
  } catch
  {
    hasLocalconfigs = false;
  }

  const loaded = await loadconfigs( {
    projectRoot: directoryRoot,
    configsPath: hasLocalconfigs ? localconfigsPath : cliconfigsPath,
  } );

  const cloned = cloneLoadedconfigs( loaded );

  if ( resolvedNested )
  {
    if ( hasLocalconfigs && loaded.nestedDefined && loaded.nested === false )
    {
      logWarn(
        `Nested mode is enabled but ${ localconfigsPath } sets nested = false. Continuing with nested processing.`,
      );
    }
    cloned.nested = true;
    cloned.nestedDefined = true;
  }

  return cloned;
}

function cloneLoadedconfigs ( configs: Loadedconfigs ): Loadedconfigs
{
  const clonedAgentconfigss: Record<string, IAgentconfigs> = {};
  for ( const [ agent, agentconfigs ] of Object.entries( configs.agentconfigss ) )
  {
    clonedAgentconfigss[ agent ] = {
      ...agentconfigs,
      mcp: agentconfigs.mcp ? { ...agentconfigs.mcp } : undefined,
    };
  }

  return {
    defaultAgents: configs.defaultAgents ? [ ...configs.defaultAgents ] : undefined,
    agentconfigss: clonedAgentconfigss,
    cliAgents: configs.cliAgents ? [ ...configs.cliAgents ] : undefined,
    mcp: configs.mcp ? { ...configs.mcp } : undefined,
    gitignore: configs.gitignore ? { ...configs.gitignore } : undefined,
    nested: configs.nested,
    nestedDefined: configs.nestedDefined,
  };
}

/**
 * Finds ruler directories based on the specified mode.
 */
async function findRulerDirectories (
  projectRoot: string,
  localOnly: boolean,
  hierarchical: boolean,
): Promise<{ dirs: string[]; primaryDir: string; }>
{
  if ( hierarchical )
  {
    const dirs = await FileSystemUtils.findAllRulerDirs( projectRoot );
    const allDirs = [ ...dirs ];

    // Add global configs if not local-only
    if ( !localOnly )
    {
      const globalDir = await FileSystemUtils.findGlobalRulerDir();
      if ( globalDir )
      {
        allDirs.push( globalDir );
      }
    }

    if ( allDirs.length === 0 )
    {
      throw createRulerError(
        `.ruler directory not found`,
        `Searched from: ${ projectRoot }`,
      );
    }
    return { dirs: allDirs, primaryDir: allDirs[ 0 ] };
  } else
  {
    const dir = await FileSystemUtils.findRulerDir( projectRoot, !localOnly );
    if ( !dir )
    {
      throw createRulerError(
        `.ruler directory not found`,
        `Searched from: ${ projectRoot }`,
      );
    }
    return { dirs: [ dir ], primaryDir: dir };
  }
}

/**
 * Warns about legacy mcp.json files if they exist.
 */
async function warnAboutLegacyMcpJson ( rulerDir: string ): Promise<void>
{
  try
  {
    const legacyMcpPath = path.join( rulerDir, 'mcp.json' );
    await fs.access( legacyMcpPath );
    logWarn(
      'Warning: Using legacy .ruler/mcp.json. Please migrate to ruler.toml. This fallback will be removed in a future release.',
    );
  } catch
  {
    // ignore
  }
}

/**
 * Loads configsuration for single-directory mode (existing behavior).
 */
export /**
 * Loads configsuration for a single .ruler directory.
 * All rules from the directory are concatenated into a single configsuration.
 * @param projectRoot Root directory of the project
 * @param configsPath Optional custom configs path
 * @param localOnly Whether to search only locally for .ruler directory
 * @returns Promise resolving to the loaded configsuration
 */
  async function loadSingleconfigsuration (
    projectRoot: string,
    configsPath: string | undefined,
    localOnly: boolean,
  ): Promise<Rulerconfigsuration>
{
  // Find the single ruler directory
  const { dirs: rulerDirs, primaryDir } = await findRulerDirectories(
    projectRoot,
    localOnly,
    false, // single mode
  );

  // Warn about legacy mcp.json
  await warnAboutLegacyMcpJson( primaryDir );

  // Load the ruler.toml configsuration
  const configs = await loadconfigs( {
    projectRoot,
    configsPath,
  } );

  // Read rule files
  const files = await FileSystemUtils.readMarkdownFiles( rulerDirs[ 0 ] );

  // Concatenate rules
  const concatenatedRules = concatenateRules( files, path.dirname( primaryDir ) );

  // Load unified configs to get merged MCP configsuration
  const { loadUnifiedconfigs } = await import( './UnifiedconfigsLoader' );
  const unifiedconfigs = await loadUnifiedconfigs( { projectRoot, configsPath } );

  // Synthesize rulerMcpJson from unified MCP bundle for backward compatibility
  let rulerMcpJson: Record<string, unknown> | null = null;
  if ( unifiedconfigs.mcp && Object.keys( unifiedconfigs.mcp.servers ).length > 0 )
  {
    rulerMcpJson = {
      mcpServers: unifiedconfigs.mcp.servers,
    };
  }

  return {
    configs,
    concatenatedRules,
    rulerMcpJson,
  };
}

/**
 * Processes hierarchical configsurations by applying rules to each .ruler directory independently.
 * Each directory gets its own set of rules and generates its own agent files.
 * @param agents Array of agents to process
 * @param configsurations Array of hierarchical configsurations for each .ruler directory
 * @param verbose Whether to enable verbose logging
 * @param dryRun Whether to perform a dry run
 * @param cliMcpEnabled Whether MCP is enabled via CLI
 * @param cliMcpStrategy MCP strategy from CLI
 * @returns Promise resolving to array of generated file paths
 */
export async function processHierarchicalconfigsurations (
  agents: IAgent[],
  configsurations: HierarchicalRulerconfigsuration[],
  verbose: boolean,
  dryRun: boolean,
  cliMcpEnabled: boolean,
  cliMcpStrategy?: McpStrategy,
  backup = true,
  skillsEnabled = true,
): Promise<string[]>
{
  const allGeneratedPaths: string[] = [];

  for ( const configs of configsurations )
  {
    logVerboseInfo(
      `Processing .ruler directory: ${ configs.rulerDir }`,
      verbose,
      dryRun,
    );
    const rulerRoot = path.dirname( configs.rulerDir );
    const paths = await applyconfigsurationsToAgents(
      agents,
      configs.concatenatedRules,
      configs.rulerMcpJson,
      configs.configs,
      rulerRoot,
      verbose,
      dryRun,
      cliMcpEnabled,
      cliMcpStrategy,
      backup,
      skillsEnabled,
    );
    const normalizedPaths = paths.map( ( p ) =>
      path.isAbsolute( p ) ? p : path.join( rulerRoot, p ),
    );
    allGeneratedPaths.push( ...normalizedPaths );
  }

  return allGeneratedPaths;
}

/**
 * Processes a single configsuration by applying rules to all selected agents.
 * All rules are concatenated and applied to generate agent files in the project root.
 * @param agents Array of agents to process
 * @param configsuration Single ruler configsuration with concatenated rules
 * @param projectRoot Root directory of the project
 * @param verbose Whether to enable verbose logging
 * @param dryRun Whether to perform a dry run
 * @param cliMcpEnabled Whether MCP is enabled via CLI
 * @param cliMcpStrategy MCP strategy from CLI
 * @returns Promise resolving to array of generated file paths
 */
export async function processSingleconfigsuration (
  agents: IAgent[],
  configsuration: Rulerconfigsuration,
  projectRoot: string,
  verbose: boolean,
  dryRun: boolean,
  cliMcpEnabled: boolean,
  cliMcpStrategy?: McpStrategy,
  backup = true,
  skillsEnabled = true,
): Promise<string[]>
{
  return await applyconfigsurationsToAgents(
    agents,
    configsuration.concatenatedRules,
    configsuration.rulerMcpJson,
    configsuration.configs,
    projectRoot,
    verbose,
    dryRun,
    cliMcpEnabled,
    cliMcpStrategy,
    backup,
    skillsEnabled,
  );
}

/**
 * Adds Skillz MCP server to rulerMcpJson if skills exist and any agent needs it.
 * Returns augmented MCP configs or original if no changes needed.
 */
async function addSkillzMcpServerIfNeeded (
  rulerMcpJson: Record<string, unknown> | null,
  projectRoot: string,
  agents: IAgent[],
  verbose: boolean,
): Promise<Record<string, unknown> | null>
{
  // Check if any agent supports MCP stdio but not native skills
  const hasAgentNeedingSkillz = agents.some(
    ( agent ) => agent.supportsMcpStdio?.() && !agent.supportsNativeSkills?.(),
  );

  if ( !hasAgentNeedingSkillz )
  {
    return rulerMcpJson;
  }

  // Check if .skillz directory exists
  try
  {
    const { SKILLZ_DIR } = await import( '../constants' );
    const skillzPath = path.join( projectRoot, SKILLZ_DIR );
    await fs.access( skillzPath );

    // Skills exist, add Skillz MCP server
    const { buildSkillzMcpconfigs } = await import( './SkillsProcessor' );
    const skillzMcp = buildSkillzMcpconfigs( projectRoot );

    // Initialize empty configs if null
    const baseconfigs = rulerMcpJson || { mcpServers: {} };
    const mcpServers = ( baseconfigs.mcpServers as Record<string, unknown> ) || {};

    logVerbose(
      'Adding Skillz MCP server to configsuration for agents that need it',
      verbose,
    );

    return {
      ...baseconfigs,
      mcpServers: {
        ...mcpServers,
        ...skillzMcp,
      },
    };
  } catch
  {
    // No .skillz directory, return original configs
    return rulerMcpJson;
  }
}

/**
 * Applies configsurations to the selected agents (internal function).
 * @param agents Array of agents to process
 * @param concatenatedRules Concatenated rule content
 * @param rulerMcpJson MCP configsuration JSON
 * @param configs Loaded configsuration
 * @param projectRoot Root directory of the project
 * @param verbose Whether to enable verbose logging
 * @param dryRun Whether to perform a dry run
 * @returns Promise resolving to array of generated file paths
 */
export async function applyconfigsurationsToAgents (
  agents: IAgent[],
  concatenatedRules: string,
  rulerMcpJson: Record<string, unknown> | null,
  configs: Loadedconfigs,
  projectRoot: string,
  verbose: boolean,
  dryRun: boolean,
  cliMcpEnabled = true,
  cliMcpStrategy?: McpStrategy,
  backup = true,
  skillsEnabled = true,
): Promise<string[]>
{
  const generatedPaths: string[] = [];
  let agentsMdWritten = false;

  // Add Skillz MCP server to rulerMcpJson if skills are enabled
  // This must happen before calling agent.applyRulerconfigs() so that agents
  // that handle MCP internally (e.g. Codex, Gemini) receive the Skillz server
  let augmentedRulerMcpJson = rulerMcpJson;
  if ( skillsEnabled && !dryRun )
  {
    augmentedRulerMcpJson = await addSkillzMcpServerIfNeeded(
      rulerMcpJson,
      projectRoot,
      agents,
      verbose,
    );
  }

  for ( const agent of agents )
  {
    logInfo( `Applying rules for ${ agent.getName() }...`, dryRun );
    logVerbose( `Processing agent: ${ agent.getName() }`, verbose );
    const agentconfigs = configs.agentconfigss[ agent.getIdentifier() ];

    // Collect output paths for .gitignore
    const outputPaths = getAgentOutputPaths( agent, projectRoot, agentconfigs );
    logVerbose(
      `Agent ${ agent.getName() } output paths: ${ outputPaths.join( ', ' ) }`,
      verbose,
    );
    generatedPaths.push( ...outputPaths );

    // Only add the backup file paths to the gitignore list if backups are enabled
    if ( backup )
    {
      const backupPaths = outputPaths.map( ( p ) => `${ p }.bak` );
      generatedPaths.push( ...backupPaths );
    }

    if ( dryRun )
    {
      logVerbose(
        `DRY RUN: Would write rules to: ${ outputPaths.join( ', ' ) }`,
        verbose,
      );
    } else
    {
      let skipApplyForThisAgent = false;
      if (
        agent.getIdentifier() === 'jules' ||
        agent.getIdentifier() === 'agentsmd'
      )
      {
        if ( agentsMdWritten )
        {
          // Skip rewriting AGENTS.md, but still allow MCP handling below
          skipApplyForThisAgent = true;
        } else
        {
          agentsMdWritten = true;
        }
      }
      let finalAgentconfigs = agentconfigs;
      if ( agent.getIdentifier() === 'augmentcode' && augmentedRulerMcpJson )
      {
        const resolvedStrategy =
          cliMcpStrategy ??
          agentconfigs?.mcp?.strategy ??
          configs.mcp?.strategy ??
          'merge';

        finalAgentconfigs = {
          ...agentconfigs,
          mcp: {
            ...agentconfigs?.mcp,
            strategy: resolvedStrategy,
          },
        };
      }

      if ( !skipApplyForThisAgent )
      {
        await agent.applyRulerconfigs(
          concatenatedRules,
          projectRoot,
          augmentedRulerMcpJson,
          finalAgentconfigs,
          backup,
        );
      }
    }

    // Handle MCP configsuration
    await handleMcpconfigsuration(
      agent,
      agentconfigs,
      configs,
      augmentedRulerMcpJson,
      projectRoot,
      generatedPaths,
      verbose,
      dryRun,
      cliMcpEnabled,
      cliMcpStrategy,
      backup,
      skillsEnabled,
    );
  }

  return generatedPaths;
}

async function handleMcpconfigsuration (
  agent: IAgent,
  agentconfigs: IAgentconfigs | undefined,
  configs: Loadedconfigs,
  rulerMcpJson: Record<string, unknown> | null,
  projectRoot: string,
  generatedPaths: string[],
  verbose: boolean,
  dryRun: boolean,
  cliMcpEnabled = true,
  cliMcpStrategy?: McpStrategy,
  backup = true,
  skillsEnabled = true,
): Promise<void>
{
  if ( !agentSupportsMcp( agent ) )
  {
    logVerbose(
      `Agent ${ agent.getName() } does not support MCP - skipping MCP configsuration`,
      verbose,
    );
    return;
  }

  const dest = await getNativeMcpPath( agent.getName(), projectRoot );
  const mcpEnabledForAgent =
    cliMcpEnabled && ( agentconfigs?.mcp?.enabled ?? configs.mcp?.enabled ?? true );

  if ( !dest || !mcpEnabledForAgent )
  {
    return;
  }

  let filteredMcpJson = rulerMcpJson
    ? filterMcpconfigsForAgent( rulerMcpJson, agent )
    : null;

  // Add Skillz MCP server for agents that support stdio but not native skills
  // Only add if skills are enabled
  if (
    skillsEnabled &&
    agent.supportsMcpStdio?.() &&
    !agent.supportsNativeSkills?.()
  )
  {
    // Check if .skillz directory exists
    try
    {
      const { SKILLZ_DIR } = await import( '../constants' );
      const skillzPath = path.join( projectRoot, SKILLZ_DIR );
      await fs.access( skillzPath );

      // Skills exist, add Skillz MCP server
      const { buildSkillzMcpconfigs } = await import( './SkillsProcessor' );
      const skillzMcp = buildSkillzMcpconfigs( projectRoot );

      // Merge Skillz server into MCP configs
      // Initialize empty configs if null
      if ( !filteredMcpJson )
      {
        filteredMcpJson = { mcpServers: {} };
      }
      const mcpServers =
        ( filteredMcpJson.mcpServers as Record<string, unknown> ) || {};
      filteredMcpJson = {
        ...filteredMcpJson,
        mcpServers: {
          ...mcpServers,
          ...skillzMcp,
        },
      };

      logVerboseInfo(
        `Added Skillz MCP server for ${ agent.getName() }`,
        verbose,
        dryRun,
      );
    } catch
    {
      // No .skillz directory, skip adding Skillz server
    }
  }

  if ( !filteredMcpJson )
  {
    logVerbose(
      `No compatible MCP servers found for ${ agent.getName() } - skipping MCP configsuration`,
      verbose,
    );
    return;
  }

  await updateGitignoreForMcpFile( dest, projectRoot, generatedPaths, backup );
  await applyMcpconfigsuration(
    agent,
    filteredMcpJson,
    dest,
    agentconfigs,
    configs,
    projectRoot,
    cliMcpStrategy,
    dryRun,
    verbose,
    backup,
  );
}

async function updateGitignoreForMcpFile (
  dest: string,
  projectRoot: string,
  generatedPaths: string[],
  backup = true,
): Promise<void>
{
  if ( dest.startsWith( projectRoot ) )
  {
    const relativeDest = path.relative( projectRoot, dest );
    generatedPaths.push( relativeDest );
    if ( backup )
    {
      generatedPaths.push( `${ relativeDest }.bak` );
    }
  }
}

async function applyMcpconfigsuration (
  agent: IAgent,
  filteredMcpJson: Record<string, unknown>,
  dest: string,
  agentconfigs: IAgentconfigs | undefined,
  configs: Loadedconfigs,
  projectRoot: string,
  cliMcpStrategy: McpStrategy | undefined,
  dryRun: boolean,
  verbose: boolean,
  backup = true,
): Promise<void>
{
  // Prevent writing MCP configss outside the project root (e.g., legacy home-directory targets)
  if ( !dest.startsWith( projectRoot ) )
  {
    logVerbose(
      `Skipping MCP configs for ${ agent.getName() } because target path is outside project: ${ dest }`,
      verbose,
    );
    return;
  }

  if ( agent.getIdentifier() === 'openhands' )
  {
    return await applyOpenHandsMcpconfigsuration(
      filteredMcpJson,
      dest,
      dryRun,
      verbose,
      backup,
    );
  }

  if ( agent.getIdentifier() === 'opencode' )
  {
    return await applyOpenCodeMcpconfigsuration(
      filteredMcpJson,
      dest,
      dryRun,
      verbose,
      backup,
    );
  }

  // Agents that handle MCP configsuration internally should not have external MCP handling
  if (
    agent.getIdentifier() === 'zed' ||
    agent.getIdentifier() === 'gemini-cli' ||
    agent.getIdentifier() === 'amazon-q-cli' ||
    agent.getIdentifier() === 'crush'
  )
  {
    logVerbose(
      `Skipping external MCP configs for ${ agent.getName() } - handled internally by agent`,
      verbose,
    );
    return;
  }

  return await applyStandardMcpconfigsuration(
    agent,
    filteredMcpJson,
    dest,
    agentconfigs,
    configs,
    cliMcpStrategy,
    dryRun,
    verbose,
    backup,
  );
}

async function applyOpenHandsMcpconfigsuration (
  filteredMcpJson: Record<string, unknown>,
  dest: string,
  dryRun: boolean,
  verbose: boolean,
  backup = true,
): Promise<void>
{
  if ( dryRun )
  {
    logVerbose(
      `DRY RUN: Would apply MCP configs by updating TOML file: ${ dest }`,
      verbose,
    );
  } else
  {
    await propagateMcpToOpenHands( filteredMcpJson, dest, backup );
  }
}

async function applyOpenCodeMcpconfigsuration (
  filteredMcpJson: Record<string, unknown>,
  dest: string,
  dryRun: boolean,
  verbose: boolean,
  backup = true,
): Promise<void>
{
  if ( dryRun )
  {
    logVerbose(
      `DRY RUN: Would apply MCP configs by updating OpenCode configs file: ${ dest }`,
      verbose,
    );
  } else
  {
    await propagateMcpToOpenCode( filteredMcpJson, dest, backup );
  }
}

/**
 * Transform MCP server types for Claude Code compatibility.
 * Claude expects "http" for HTTP servers and "sse" for SSE servers, not "remote".
 */
function transformMcpForClaude (
  mcpJson: Record<string, unknown>,
): Record<string, unknown>
{
  if ( !mcpJson.mcpServers || typeof mcpJson.mcpServers !== 'object' )
  {
    return mcpJson;
  }

  const transformedMcp = { ...mcpJson };
  const transformedServers: Record<string, unknown> = {};

  for ( const [ name, serverDef ] of Object.entries(
    mcpJson.mcpServers as Record<string, unknown>,
  ) )
  {
    if ( serverDef && typeof serverDef === 'object' )
    {
      const server = serverDef as Record<string, unknown>;
      const transformedServer = { ...server };

      // Transform type: "remote" to appropriate Claude types
      if (
        server.type === 'remote' &&
        server.url &&
        typeof server.url === 'string'
      )
      {
        const url = server.url as string;

        // Check if URL suggests SSE (contains /sse path segment)
        if ( /\/sse(\/|$)/i.test( url ) )
        {
          transformedServer.type = 'sse';
        } else
        {
          transformedServer.type = 'http';
        }
      }

      transformedServers[ name ] = transformedServer;
    } else
    {
      transformedServers[ name ] = serverDef;
    }
  }

  transformedMcp.mcpServers = transformedServers;
  return transformedMcp;
}

/**
 * Transform MCP server types for Kilo Code compatibility.
 * Kilo Code expects "streamable-http" for remote HTTP servers, not "remote".
 */
function transformMcpForKiloCode (
  mcpJson: Record<string, unknown>,
): Record<string, unknown>
{
  if ( !mcpJson.mcpServers || typeof mcpJson.mcpServers !== 'object' )
  {
    return mcpJson;
  }

  const transformedMcp = { ...mcpJson };
  const transformedServers: Record<string, unknown> = {};

  for ( const [ name, serverDef ] of Object.entries(
    mcpJson.mcpServers as Record<string, unknown>,
  ) )
  {
    if ( serverDef && typeof serverDef === 'object' )
    {
      const server = serverDef as Record<string, unknown>;
      const transformedServer = { ...server };

      // Transform type: "remote" to "streamable-http" for HTTP-based servers
      if (
        server.type === 'remote' &&
        server.url &&
        typeof server.url === 'string'
      )
      {
        transformedServer.type = 'streamable-http';
      }

      transformedServers[ name ] = transformedServer;
    } else
    {
      transformedServers[ name ] = serverDef;
    }
  }

  transformedMcp.mcpServers = transformedServers;
  return transformedMcp;
}

async function applyStandardMcpconfigsuration (
  agent: IAgent,
  filteredMcpJson: Record<string, unknown>,
  dest: string,
  agentconfigs: IAgentconfigs | undefined,
  configs: Loadedconfigs,
  cliMcpStrategy: McpStrategy | undefined,
  dryRun: boolean,
  verbose: boolean,
  backup = true,
): Promise<void>
{
  const strategy =
    cliMcpStrategy ??
    agentconfigs?.mcp?.strategy ??
    configs.mcp?.strategy ??
    'merge';
  const serverKey = agent.getMcpServerKey?.() ?? 'mcpServers';

  // Skip agents with empty server keys (e.g., AgentsMdAgent, GooseAgent)
  if ( serverKey === '' )
  {
    logVerbose(
      `Skipping MCP configs for ${ agent.getName() } - agent has empty server key`,
      verbose,
    );
    return;
  }

  logVerbose(
    `Applying filtered MCP configs for ${ agent.getName() } with strategy: ${ strategy } and key: ${ serverKey }`,
    verbose,
  );

  if ( dryRun )
  {
    logVerbose( `DRY RUN: Would apply MCP configs to: ${ dest }`, verbose );
  } else
  {
    // Transform MCP configs for agent-specific compatibility
    let mcpToMerge = filteredMcpJson;
    if ( agent.getIdentifier() === 'claude' )
    {
      mcpToMerge = transformMcpForClaude( filteredMcpJson );
    } else if ( agent.getIdentifier() === 'kilocode' )
    {
      mcpToMerge = transformMcpForKiloCode( filteredMcpJson );
    }

    const existing = await readNativeMcp( dest );
    const merged = mergeMcp( existing, mcpToMerge, strategy, serverKey );

    // Firebase Studio (IDX) expects no "type" fields in .idx/mcp.json server entries.
    // Sanitize merged configs by stripping 'type' from each server when targeting Firebase.
    const sanitizeForFirebase = (
      obj: Record<string, unknown>,
    ): Record<string, unknown> =>
    {
      if ( agent.getIdentifier() !== 'firebase' ) return obj;
      const out: Record<string, unknown> = { ...obj };
      const servers = ( out[ serverKey ] as Record<string, unknown> ) || {};
      const cleanedServers: Record<string, unknown> = {};
      for ( const [ name, def ] of Object.entries( servers ) )
      {
        if ( def && typeof def === 'object' )
        {
          const copy = { ...( def as Record<string, unknown> ) };
          delete ( copy as Record<string, unknown> ).type;
          cleanedServers[ name ] = copy;
        } else
        {
          cleanedServers[ name ] = def;
        }
      }
      out[ serverKey ] = cleanedServers;
      return out;
    };

    const toWrite = sanitizeForFirebase( merged );

    // Only backup and write if content would actually change (idempotent)
    const currentContent = JSON.stringify( existing, null, 2 );
    const newContent = JSON.stringify( toWrite, null, 2 );

    if ( currentContent !== newContent )
    {
      if ( backup )
      {
        const { backupFile } = await import( '../core/FileSystemUtils' );
        await backupFile( dest );
      }
      await writeNativeMcp( dest, toWrite );
    } else
    {
      logVerbose(
        `MCP configs for ${ agent.getName() } is already up to date - skipping backup and write`,
        verbose,
      );
    }
  }
}

/**
 * Updates the .gitignore file with generated paths.
 * @param projectRoot Root directory of the project
 * @param generatedPaths Array of generated file paths
 * @param configs Loaded configsuration
 * @param cliGitignoreEnabled CLI gitignore setting
 * @param dryRun Whether to perform a dry run
 */
export async function updateGitignore (
  projectRoot: string,
  generatedPaths: string[],
  configs: Loadedconfigs,
  cliGitignoreEnabled: boolean | undefined,
  dryRun: boolean,
): Promise<void>
{
  // configsuration precedence: CLI > TOML > Default (enabled)
  let gitignoreEnabled: boolean;
  if ( cliGitignoreEnabled !== undefined )
  {
    gitignoreEnabled = cliGitignoreEnabled;
  } else if ( configs.gitignore?.enabled !== undefined )
  {
    gitignoreEnabled = configs.gitignore.enabled;
  } else
  {
    gitignoreEnabled = true; // Default enabled
  }

  if ( gitignoreEnabled && generatedPaths.length > 0 )
  {
    const uniquePaths = [ ...new Set( generatedPaths ) ];

    // Note: Individual backup patterns are added per-file in the collection phase
    // No need to add a broad *.bak pattern here

    if ( uniquePaths.length > 0 )
    {
      if ( dryRun )
      {
        logInfo(
          `Would update .gitignore with ${ uniquePaths.length } unique path(s): ${ uniquePaths.join( ', ' ) }`,
          dryRun,
        );
      } else
      {
        await updateGitignoreUtil( projectRoot, uniquePaths );
        logInfo(
          `Updated .gitignore with ${ uniquePaths.length } unique path(s) in the Ruler block.`,
          dryRun,
        );
      }
    }
  }
}
