import * as path from 'path';
import { promises as fs } from 'fs';
import * as FileSystemUtils from './core/FileSystemUtils';
import { loadconfigs } from './core/configsLoader';
import { IAgent } from './agents/IAgent';
import { allAgents } from './agents';
import { createRulerError, logVerbose, actionPrefix } from './constants';
import
  {
    revertAgentconfigsuration,
    cleanUpAuxiliaryFiles,
  } from './core/revert-engine';
import { resolveSelectedAgents } from './core/agent-selection';
import { mapRawAgentconfigss } from './core/configs-utils';

const agents: IAgent[] = allAgents;

export { allAgents };

/**
 * Reverts ruler configsurations for selected AI agents.
 */
export async function revertAllAgentconfigss (
  projectRoot: string,
  includedAgents?: string[],
  configsPath?: string,
  keepBackups = false,
  verbose = false,
  dryRun = false,
  localOnly = false,
): Promise<void>
{
  logVerbose(
    `Loading configsuration for revert from project root: ${ projectRoot }`,
    verbose,
  );

  const configs = await loadconfigs( {
    projectRoot,
    cliAgents: includedAgents,
    configsPath,
  } );

  const rulerDir = await FileSystemUtils.findRulerDir( projectRoot, !localOnly );
  if ( !rulerDir )
  {
    throw createRulerError(
      `.ruler directory not found`,
      `Searched from: ${ projectRoot }`,
    );
  }
  logVerbose( `Found .ruler directory at: ${ rulerDir }`, verbose );

  // Normalize per-agent configs keys to agent identifiers
  configs.agentconfigss = mapRawAgentconfigss( configs.agentconfigss, agents );

  // Select agents to revert (same logic as apply, but with backward compatibility for invalid agents)
  let selected: IAgent[];
  try
  {
    selected = resolveSelectedAgents( configs, agents );
  } catch ( error )
  {
    // For backward compatibility, revert continues with available agents if some are invalid
    // This preserves the original behavior where invalid agents were silently ignored
    if (
      error instanceof Error &&
      error.message.includes( 'Invalid agent specified' )
    )
    {
      logVerbose(
        `Warning: ${ error.message } - continuing with valid agents only`,
        verbose,
      );

      // Fall back to the old logic without validation
      if ( configs.cliAgents && configs.cliAgents.length > 0 )
      {
        const filters = configs.cliAgents.map( ( n ) => n.toLowerCase() );
        selected = agents.filter( ( agent ) =>
          filters.some(
            ( f ) =>
              agent.getIdentifier() === f ||
              agent.getName().toLowerCase().includes( f ),
          ),
        );
      } else if ( configs.defaultAgents && configs.defaultAgents.length > 0 )
      {
        const defaults = configs.defaultAgents.map( ( n ) => n.toLowerCase() );
        selected = agents.filter( ( agent ) =>
        {
          const identifier = agent.getIdentifier();
          const override = configs.agentconfigss[ identifier ]?.enabled;
          if ( override !== undefined )
          {
            return override;
          }
          return defaults.some(
            ( d ) =>
              identifier === d || agent.getName().toLowerCase().includes( d ),
          );
        } );
      } else
      {
        selected = agents.filter(
          ( agent ) =>
            configs.agentconfigss[ agent.getIdentifier() ]?.enabled !== false,
        );
      }
    } else
    {
      throw error;
    }
  }

  logVerbose(
    `Selected agents: ${ selected.map( ( a ) => a.getName() ).join( ', ' ) }`,
    verbose,
  );

  // Revert configsurations for each agent
  let totalFilesProcessed = 0;
  let totalFilesRestored = 0;
  let totalFilesRemoved = 0;
  let totalBackupsRemoved = 0;

  for ( const agent of selected )
  {
    const prefix = actionPrefix( dryRun );
    console.log( `${ prefix } Reverting ${ agent.getName() }...` );

    const agentconfigs = configs.agentconfigss[ agent.getIdentifier() ];
    const result = await revertAgentconfigsuration(
      agent,
      projectRoot,
      agentconfigs,
      keepBackups,
      verbose,
      dryRun,
    );

    totalFilesProcessed += result.restored + result.removed;
    totalFilesRestored += result.restored;
    totalFilesRemoved += result.removed;
    totalBackupsRemoved += result.backupsRemoved;
  }

  // Clean up auxiliary files and directories
  const cleanupResult = await cleanUpAuxiliaryFiles(
    projectRoot,
    verbose,
    dryRun,
  );
  totalFilesRemoved += cleanupResult.additionalFilesRemoved;

  // Clean .gitignore if reverting all agents
  const gitignoreCleaned =
    !configs.cliAgents || configs.cliAgents.length === 0
      ? await cleanGitignore( projectRoot, verbose, dryRun )
      : false;

  // Display summary
  const prefix = actionPrefix( dryRun );

  if ( dryRun )
  {
    console.log( `${ prefix } Revert summary (dry run):` );
  } else
  {
    console.log( `${ prefix } Revert completed successfully.` );
  }

  console.log( `  Files processed: ${ totalFilesProcessed }` );
  console.log( `  Files restored from backup: ${ totalFilesRestored }` );
  console.log( `  Generated files removed: ${ totalFilesRemoved }` );
  if ( !keepBackups )
  {
    console.log( `  Backup files removed: ${ totalBackupsRemoved }` );
  }
  if ( cleanupResult.directoriesRemoved > 0 )
  {
    console.log(
      `  Empty directories removed: ${ cleanupResult.directoriesRemoved }`,
    );
  }
  if ( gitignoreCleaned )
  {
    console.log( `  .gitignore cleaned: yes` );
  }
}

/**
 * Removes the ruler-managed block from .gitignore file.
 */
async function cleanGitignore (
  projectRoot: string,
  verbose: boolean,
  dryRun: boolean,
): Promise<boolean>
{
  const gitignorePath = path.join( projectRoot, '.gitignore' );

  try
  {
    await fs.access( gitignorePath );
  } catch
  {
    logVerbose( 'No .gitignore file found', verbose );
    return false;
  }

  const content = await fs.readFile( gitignorePath, 'utf8' );
  const startMarker = '# START Ruler Generated Files';
  const endMarker = '# END Ruler Generated Files';

  const startIndex = content.indexOf( startMarker );
  const endIndex = content.indexOf( endMarker );

  if ( startIndex === -1 || endIndex === -1 )
  {
    logVerbose( 'No ruler-managed block found in .gitignore', verbose );
    return false;
  }

  const prefix = actionPrefix( dryRun );

  if ( dryRun )
  {
    logVerbose( `${ prefix } Would remove ruler block from .gitignore`, verbose );
  } else
  {
    const beforeBlock = content.substring( 0, startIndex );
    const afterBlock = content.substring( endIndex + endMarker.length );

    let newContent = beforeBlock + afterBlock;
    newContent = newContent.replace( /\n{3,}/g, '\n\n' ); // Replace 3+ newlines with 2

    if ( newContent.trim() === '' )
    {
      await fs.unlink( gitignorePath );
      logVerbose( `${ prefix } Removed empty .gitignore file`, verbose );
    } else
    {
      await fs.writeFile( gitignorePath, newContent );
      logVerbose( `${ prefix } Removed ruler block from .gitignore`, verbose );
    }
  }

  return true;
}
