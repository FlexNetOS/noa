import { promises as fs } from 'fs';
import * as path from 'path';
import * as os from 'os';
import { parse as parseTOML } from '@iarna/toml';
import { z } from 'zod';
import
  {
    Mcpconfigs,
    GlobalMcpconfigs,
    Gitignoreconfigs,
    Skillsconfigs,
  } from '../types';
import { createRulerError } from '../constants';

interface ErrnoException extends Error
{
  code?: string;
}

const mcpconfigsSchema = z
  .object( {
    enabled: z.boolean().optional(),
    merge_strategy: z.enum( [ 'merge', 'overwrite' ] ).optional(),
  } )
  .optional();

const agentconfigsSchema = z
  .object( {
    enabled: z.boolean().optional(),
    output_path: z.string().optional(),
    output_path_instructions: z.string().optional(),
    output_path_configs: z.string().optional(),
    mcp: mcpconfigsSchema,
  } )
  .optional();

const rulerconfigsSchema = z.object( {
  default_agents: z.array( z.string() ).optional(),
  agents: z.record( z.string(), agentconfigsSchema ).optional(),
  mcp: z
    .object( {
      enabled: z.boolean().optional(),
      merge_strategy: z.enum( [ 'merge', 'overwrite' ] ).optional(),
    } )
    .optional(),
  gitignore: z
    .object( {
      enabled: z.boolean().optional(),
    } )
    .optional(),
  skills: z
    .object( {
      enabled: z.boolean().optional(),
    } )
    .optional(),
  nested: z.boolean().optional(),
} );

/**
 * Recursively creates a new object with only enumerable string keys,
 * effectively excluding Symbol properties.
 * The @iarna/toml parser adds Symbol properties (Symbol(type), Symbol(declared))
 * for metadata, which Zod v4+ validates and rejects as invalid record keys.
 * By rebuilding the object structure using Object.keys(), we create clean objects
 * that only contain the actual data without Symbol metadata.
 */
function stripSymbols ( obj: unknown ): unknown
{
  if ( obj === null || typeof obj !== 'object' )
  {
    return obj;
  }
  if ( Array.isArray( obj ) )
  {
    return obj.map( stripSymbols );
  }
  const result: Record<string, unknown> = {};
  for ( const key of Object.keys( obj ) )
  {
    result[ key ] = stripSymbols( ( obj as Record<string, unknown> )[ key ] );
  }
  return result;
}

/**
 * configsuration for a specific agent as defined in ruler.toml.
 */
export interface IAgentconfigs
{
  enabled?: boolean;
  outputPath?: string;
  outputPathInstructions?: string;
  outputPathconfigs?: string;
  /** MCP propagation configs for this agent. */
  mcp?: Mcpconfigs;
}

/**
 * Parsed ruler configsuration values.
 */
export interface Loadedconfigs
{
  /** Agents to run by default, as specified by default_agents. */
  defaultAgents?: string[];
  /** Per-agent configsuration overrides. */
  agentconfigss: Record<string, IAgentconfigs>;
  /** Command-line agent filters (--agents), if provided. */
  cliAgents?: string[];
  /** Global MCP servers configsuration section. */
  mcp?: GlobalMcpconfigs;
  /** Gitignore configsuration section. */
  gitignore?: Gitignoreconfigs;
  /** Skills configsuration section. */
  skills?: Skillsconfigs;
  /** Whether to enable nested rule loading from nested .ruler directories. */
  nested?: boolean;
  /** Whether the nested option was explicitly provided in the configs. */
  nestedDefined?: boolean;
}

/**
 * Options for loading the ruler configsuration.
 */
export interface configsOptions
{
  projectRoot: string;
  /** Path to a custom TOML configs file. */
  configsPath?: string;
  /** CLI filters from --agents option. */
  cliAgents?: string[];
}

/**
 * Loads and parses the ruler TOML configsuration file, applying defaults.
 * If the file is missing or invalid, returns empty/default configs.
 */
export async function loadconfigs (
  options: configsOptions,
): Promise<Loadedconfigs>
{
  const { projectRoot, configsPath, cliAgents } = options;
  let configsFile: string;

  if ( configsPath )
  {
    configsFile = path.resolve( configsPath );
  } else
  {
    // Try local .ruler/ruler.toml first
    const localconfigsFile = path.join( projectRoot, '.ruler', 'ruler.toml' );
    try
    {
      await fs.access( localconfigsFile );
      configsFile = localconfigsFile;
    } catch
    {
      // If local configs doesn't exist, try global configs
      const xdgconfigsDir =
        process.env.XDG_configs_HOME || path.join( os.homedir(), '.configs' );
      configsFile = path.join( xdgconfigsDir, 'ruler', 'ruler.toml' );
    }
  }
  let raw: Record<string, unknown> = {};
  try
  {
    const text = await fs.readFile( configsFile, 'utf8' );
    const parsed = text.trim() ? parseTOML( text ) : {};
    // Strip Symbol properties added by @iarna/toml (required for Zod v4+)
    raw = stripSymbols( parsed ) as Record<string, unknown>;

    // Validate the configsuration with zod
    const validationResult = rulerconfigsSchema.safeParse( raw );
    if ( !validationResult.success )
    {
      throw createRulerError(
        'Invalid configsuration file format',
        `File: ${ configsFile }, Errors: ${ validationResult.error.issues.map( ( i ) => `${ i.path.join( '.' ) }: ${ i.message }` ).join( ', ' ) }`,
      );
    }
  } catch ( err )
  {
    if ( err instanceof Error && ( err as ErrnoException ).code !== 'ENOENT' )
    {
      if ( err.message.includes( '[ruler]' ) )
      {
        throw err; // Re-throw validation errors
      }
      console.warn(
        `[ruler] Warning: could not read configs file at ${ configsFile }: ${ err.message }`,
      );
    }
    raw = {};
  }

  const defaultAgents = Array.isArray( raw.default_agents )
    ? raw.default_agents.map( ( a ) => String( a ) )
    : undefined;

  const agentsSection =
    raw.agents && typeof raw.agents === 'object' && !Array.isArray( raw.agents )
      ? ( raw.agents as Record<string, unknown> )
      : {};
  const agentconfigss: Record<string, IAgentconfigs> = {};
  for ( const [ name, section ] of Object.entries( agentsSection ) )
  {
    if ( section && typeof section === 'object' )
    {
      const sectionObj = section as Record<string, unknown>;
      const cfg: IAgentconfigs = {};
      if ( typeof sectionObj.enabled === 'boolean' )
      {
        cfg.enabled = sectionObj.enabled;
      }
      if ( typeof sectionObj.output_path === 'string' )
      {
        cfg.outputPath = path.resolve( projectRoot, sectionObj.output_path );
      }
      if ( typeof sectionObj.output_path_instructions === 'string' )
      {
        cfg.outputPathInstructions = path.resolve(
          projectRoot,
          sectionObj.output_path_instructions,
        );
      }
      if ( typeof sectionObj.output_path_configs === 'string' )
      {
        cfg.outputPathconfigs = path.resolve(
          projectRoot,
          sectionObj.output_path_configs,
        );
      }
      if ( sectionObj.mcp && typeof sectionObj.mcp === 'object' )
      {
        const m = sectionObj.mcp as Record<string, unknown>;
        const mcpCfg: Mcpconfigs = {};
        if ( typeof m.enabled === 'boolean' )
        {
          mcpCfg.enabled = m.enabled;
        }
        if ( typeof m.merge_strategy === 'string' )
        {
          const ms = m.merge_strategy;
          if ( ms === 'merge' || ms === 'overwrite' )
          {
            mcpCfg.strategy = ms;
          }
        }
        cfg.mcp = mcpCfg;
      }
      agentconfigss[ name ] = cfg;
    }
  }

  const rawMcpSection =
    raw.mcp && typeof raw.mcp === 'object' && !Array.isArray( raw.mcp )
      ? ( raw.mcp as Record<string, unknown> )
      : {};
  const globalMcpconfigs: GlobalMcpconfigs = {};
  if ( typeof rawMcpSection.enabled === 'boolean' )
  {
    globalMcpconfigs.enabled = rawMcpSection.enabled;
  }
  if ( typeof rawMcpSection.merge_strategy === 'string' )
  {
    const strat = rawMcpSection.merge_strategy;
    if ( strat === 'merge' || strat === 'overwrite' )
    {
      globalMcpconfigs.strategy = strat;
    }
  }

  const rawGitignoreSection =
    raw.gitignore &&
      typeof raw.gitignore === 'object' &&
      !Array.isArray( raw.gitignore )
      ? ( raw.gitignore as Record<string, unknown> )
      : {};
  const gitignoreconfigs: Gitignoreconfigs = {};
  if ( typeof rawGitignoreSection.enabled === 'boolean' )
  {
    gitignoreconfigs.enabled = rawGitignoreSection.enabled;
  }

  const rawSkillsSection =
    raw.skills && typeof raw.skills === 'object' && !Array.isArray( raw.skills )
      ? ( raw.skills as Record<string, unknown> )
      : {};
  const skillsconfigs: Skillsconfigs = {};
  if ( typeof rawSkillsSection.enabled === 'boolean' )
  {
    skillsconfigs.enabled = rawSkillsSection.enabled;
  }

  const nestedDefined = typeof raw.nested === 'boolean';
  const nested = nestedDefined ? ( raw.nested as boolean ) : false;

  return {
    defaultAgents,
    agentconfigss,
    cliAgents,
    mcp: globalMcpconfigs,
    gitignore: gitignoreconfigs,
    skills: skillsconfigs,
    nested,
    nestedDefined,
  };
}
