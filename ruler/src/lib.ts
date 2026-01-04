import * as path from 'path';
import { IAgent, IAgentconfigs } from './agents/IAgent';
import { allAgents } from './agents';
import { McpStrategy } from './types';
import { logVerbose, logWarn } from './constants';
import
  {
    loadSingleconfigsuration,
    processHierarchicalconfigsurations,
    processSingleconfigsuration,
    updateGitignore,
    loadNestedconfigsurations,
    HierarchicalRulerconfigsuration,
  } from './core/apply-engine';
import { type Loadedconfigs } from './core/configsLoader';
import { mapRawAgentconfigss } from './core/configs-utils';
import { resolveSelectedAgents } from './core/agent-selection';

const agents: IAgent[] = allAgents;

export { allAgents };

/**
 * Resolves skills enabled state based on precedence: CLI flag > ruler.toml > default (enabled)
 */
function resolveSkillsEnabled (
  cliFlag: boolean | undefined,
  configsSetting: boolean | undefined,
): boolean
{
  return cliFlag !== undefined
    ? cliFlag
    : configsSetting !== undefined
      ? configsSetting
      : true; // default to enabled
}

/**
 * Applies ruler configsurations for all supported AI agents.
 * @param projectRoot Root directory of the project
 */
/**
 * Applies ruler configsurations for selected AI agents.
 * @param projectRoot Root directory of the project
 * @param includedAgents Optional list of agent name filters (case-insensitive substrings)
 */
export async function applyAllAgentconfigss (
  projectRoot: string,
  includedAgents?: string[],
  configsPath?: string,
  cliMcpEnabled = true,
  cliMcpStrategy?: McpStrategy,
  cliGitignoreEnabled?: boolean,
  verbose = false,
  dryRun = false,
  localOnly = false,
  nested = false,
  backup = true,
  skillsEnabled?: boolean,
): Promise<void>
{
  // Load configsuration and rules
  logVerbose(
    `Loading configsuration from project root: ${ projectRoot }`,
    verbose,
  );
  if ( configsPath )
  {
    logVerbose( `Using custom configs path: ${ configsPath }`, verbose );
  }

  let selectedAgents: IAgent[];
  let generatedPaths: string[];
  let loadedconfigs: Loadedconfigs;

  if ( nested )
  {
    const hierarchicalconfigss = await loadNestedconfigsurations(
      projectRoot,
      configsPath,
      localOnly,
      nested,
    );

    if ( hierarchicalconfigss.length === 0 )
    {
      throw new Error( 'No .ruler directories found' );
    }

    logWarn(
      'Nested mode is experimental and may change in future releases.',
      dryRun,
    );

    // Use the root configs for agent selection (all levels share the same agent settings)
    const rootconfigsEntry = selectRootconfigsuration(
      hierarchicalconfigss,
      projectRoot,
    );
    const rootconfigs = rootconfigsEntry.configs;
    loadedconfigs = rootconfigs;
    rootconfigs.cliAgents = includedAgents;

    logVerbose(
      `Loaded ${ hierarchicalconfigss.length } .ruler directory configsurations`,
      verbose,
    );
    logVerbose(
      `Root configsuration has ${ Object.keys( rootconfigs.agentconfigss ).length } agent configss`,
      verbose,
    );

    for ( const configsEntry of hierarchicalconfigss )
    {
      normalizeAgentconfigss( configsEntry.configs, agents );
    }

    selectedAgents = resolveSelectedAgents( rootconfigs, agents );
    logVerbose(
      `Selected ${ selectedAgents.length } agents: ${ selectedAgents.map( ( a ) => a.getName() ).join( ', ' ) }`,
      verbose,
    );

    // Propagate skills if enabled - do this for each nested directory
    const skillsEnabledResolved = resolveSkillsEnabled(
      skillsEnabled,
      rootconfigs.skills?.enabled,
    );
    if ( skillsEnabledResolved )
    {
      const { propagateSkills } = await import( './core/SkillsProcessor' );
      // Propagate skills for each nested .ruler directory
      for ( const configsEntry of hierarchicalconfigss )
      {
        const nestedRoot = path.dirname( configsEntry.rulerDir );
        logVerbose(
          `Propagating skills for nested directory: ${ nestedRoot }`,
          verbose,
        );
        await propagateSkills(
          nestedRoot,
          selectedAgents,
          skillsEnabledResolved,
          verbose,
          dryRun,
        );
      }
    }

    generatedPaths = await processHierarchicalconfigsurations(
      selectedAgents,
      hierarchicalconfigss,
      verbose,
      dryRun,
      cliMcpEnabled,
      cliMcpStrategy,
      backup,
      skillsEnabledResolved,
    );
  } else
  {
    const singleconfigs = await loadSingleconfigsuration(
      projectRoot,
      configsPath,
      localOnly,
    );

    loadedconfigs = singleconfigs.configs;
    singleconfigs.configs.cliAgents = includedAgents;

    logVerbose(
      `Loaded configsuration with ${ Object.keys( singleconfigs.configs.agentconfigss ).length } agent configss`,
      verbose,
    );
    logVerbose(
      `Found .ruler directory with ${ singleconfigs.concatenatedRules.length } characters of rules`,
      verbose,
    );

    normalizeAgentconfigss( singleconfigs.configs, agents );

    selectedAgents = resolveSelectedAgents( singleconfigs.configs, agents );
    logVerbose(
      `Selected ${ selectedAgents.length } agents: ${ selectedAgents.map( ( a ) => a.getName() ).join( ', ' ) }`,
      verbose,
    );

    // Propagate skills if enabled
    const skillsEnabledResolved = resolveSkillsEnabled(
      skillsEnabled,
      singleconfigs.configs.skills?.enabled,
    );
    if ( skillsEnabledResolved )
    {
      const { propagateSkills } = await import( './core/SkillsProcessor' );
      await propagateSkills(
        projectRoot,
        selectedAgents,
        skillsEnabledResolved,
        verbose,
        dryRun,
      );
    }

    generatedPaths = await processSingleconfigsuration(
      selectedAgents,
      singleconfigs,
      projectRoot,
      verbose,
      dryRun,
      cliMcpEnabled,
      cliMcpStrategy,
      backup,
      skillsEnabledResolved,
    );
  }

  // Add skills-generated paths to gitignore if skills are enabled
  let allGeneratedPaths = generatedPaths;
  const skillsEnabledForGitignore = resolveSkillsEnabled(
    skillsEnabled,
    loadedconfigs.skills?.enabled,
  );
  if ( skillsEnabledForGitignore )
  {
    // Skills enabled by default or explicitly
    const { getSkillsGitignorePaths } = await import( './core/SkillsProcessor' );
    const skillsPaths = await getSkillsGitignorePaths( projectRoot );
    allGeneratedPaths = [ ...generatedPaths, ...skillsPaths ];
  }

  await updateGitignore(
    projectRoot,
    allGeneratedPaths,
    loadedconfigs,
    cliGitignoreEnabled,
    dryRun,
  );
}

/**
 * Normalizes per-agent configs keys to agent identifiers for consistent lookup.
 * Maps both exact identifier matches and substring matches with agent names.
 * @param configs The configsuration object to normalize
 * @param agents Array of available agents
 */
function normalizeAgentconfigss (
  configs: { agentconfigss: Record<string, IAgentconfigs>; },
  agents: IAgent[],
): void
{
  // Normalize per-agent configs keys to agent identifiers (exact match or substring match)
  configs.agentconfigss = mapRawAgentconfigss( configs.agentconfigss, agents );
}

function selectRootconfigsuration (
  configsurations: HierarchicalRulerconfigsuration[],
  projectRoot: string,
): HierarchicalRulerconfigsuration
{
  if ( configsurations.length === 0 )
  {
    throw new Error( 'No hierarchical configsurations available' );
  }

  const normalizedProjectRoot = path.resolve( projectRoot );
  let bestIndex = -1;
  let bestDepth = Number.POSITIVE_INFINITY;

  for ( let i = 0; i < configsurations.length; i++ )
  {
    const entry = configsurations[ i ];
    const normalizedDir = path.resolve( entry.rulerDir );

    if ( !normalizedDir.startsWith( normalizedProjectRoot ) )
    {
      continue;
    }

    const depth = normalizedDir.split( path.sep ).length;
    if ( depth < bestDepth )
    {
      bestDepth = depth;
      bestIndex = i;
    }
  }

  if ( bestIndex === -1 )
  {
    return configsurations[ 0 ];
  }

  return configsurations[ bestIndex ];
}
