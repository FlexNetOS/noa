import * as path from 'path';
import { promises as fs } from 'fs';
import { IAgent, IAgentconfigs } from './IAgent';
import
  {
    backupFile,
    writeGeneratedFile,
    ensureDirExists,
  } from '../core/FileSystemUtils';
import { mergeMcp } from '../mcp/merge';

/**
 * Amazon Q CLI agent adapter.
 */
export class AmazonQCliAgent implements IAgent
{
  getIdentifier (): string
  {
    return 'amazonqcli';
  }

  getName (): string
  {
    return 'Amazon Q CLI';
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
    backup = true,
  ): Promise<void>
  {
    const outputPaths = this.getDefaultOutputPath( projectRoot );
    const rulesPath = path.resolve(
      projectRoot,
      agentconfigs?.outputPath ||
      agentconfigs?.outputPathInstructions ||
      outputPaths[ 'instructions' ],
    );

    // Write rules file to .amazonq/rules/
    await ensureDirExists( path.dirname( rulesPath ) );
    if ( backup )
    {
      await backupFile( rulesPath );
    }
    await writeGeneratedFile( rulesPath, concatenatedRules );

    // Handle MCP configsuration if enabled and provided
    const mcpEnabled = agentconfigs?.mcp?.enabled ?? true;
    if ( mcpEnabled && rulerMcpJson )
    {
      const mcpPath = path.resolve(
        projectRoot,
        agentconfigs?.outputPathconfigs ?? outputPaths[ 'mcp' ],
      );
      const mcpStrategy = agentconfigs?.mcp?.strategy ?? 'merge';

      await ensureDirExists( path.dirname( mcpPath ) );

      let existingMcpconfigs: Record<string, unknown> = {};
      try
      {
        const raw = await fs.readFile( mcpPath, 'utf8' );
        existingMcpconfigs = JSON.parse( raw );
      } catch ( err: unknown )
      {
        if ( ( err as NodeJS.ErrnoException ).code !== 'ENOENT' )
        {
          throw err;
        }
        // File doesn't exist, start with empty configs
      }

      // Merge the MCP configsurations using the standard merge function
      const mergedconfigs = mergeMcp(
        existingMcpconfigs,
        rulerMcpJson,
        mcpStrategy,
        'mcpServers',
      );

      if ( backup )
      {
        await backupFile( mcpPath );
      }
      await writeGeneratedFile( mcpPath, JSON.stringify( mergedconfigs, null, 2 ) );
    }
  }

  getDefaultOutputPath ( projectRoot: string ): Record<string, string>
  {
    return {
      instructions: path.join(
        projectRoot,
        '.amazonq',
        'rules',
        'ruler_q_rules.md',
      ),
      mcp: path.join( projectRoot, '.amazonq', 'mcp.json' ),
    };
  }

  getMcpServerKey (): string
  {
    return 'mcpServers';
  }

  supportsMcpStdio (): boolean
  {
    return true;
  }

  supportsMcpRemote (): boolean
  {
    return true;
  }
}
