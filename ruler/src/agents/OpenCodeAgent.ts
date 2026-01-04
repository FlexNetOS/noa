import { IAgent, IAgentconfigs } from './IAgent';
import * as fs from 'fs/promises';
import * as path from 'path';

export class OpenCodeAgent implements IAgent
{
  getIdentifier (): string
  {
    return 'opencode';
  }

  getName (): string
  {
    return 'OpenCode';
  }

  getDefaultOutputPath ( projectRoot: string ): Record<string, string>
  {
    return {
      instructions: path.join( projectRoot, 'AGENTS.md' ),
      mcp: path.join( projectRoot, 'opencode.json' ),
    };
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
  ): Promise<void>
  {
    const outputPaths = this.getDefaultOutputPath( projectRoot );
    const instructionsPath = path.resolve(
      projectRoot,
      agentconfigs?.outputPathInstructions ?? outputPaths[ 'instructions' ],
    );
    const mcpPath = path.resolve(
      projectRoot,
      agentconfigs?.outputPathconfigs ?? outputPaths[ 'mcp' ],
    );

    await fs.writeFile( instructionsPath, concatenatedRules );

    // Create OpenCode configs with schema and MCP configsuration
    let finalMcpconfigs: { $schema: string; mcp: Record<string, unknown>; } = {
      $schema: 'https://opencode.ai/configs.json',
      mcp: {},
    };

    try
    {
      const existingMcpconfigs = JSON.parse( await fs.readFile( mcpPath, 'utf-8' ) );
      if ( existingMcpconfigs && typeof existingMcpconfigs === 'object' )
      {
        finalMcpconfigs = {
          $schema: 'https://opencode.ai/configs.json',
          ...existingMcpconfigs,
          mcp: {
            ...( existingMcpconfigs.mcp || {} ),
            ...( ( rulerMcpJson?.mcpServers ?? {} ) as Record<string, unknown> ),
          },
        };
      } else if ( rulerMcpJson )
      {
        finalMcpconfigs = {
          $schema: 'https://opencode.ai/configs.json',
          mcp: ( rulerMcpJson?.mcpServers ?? {} ) as Record<string, unknown>,
        };
      }
    } catch
    {
      if ( rulerMcpJson )
      {
        finalMcpconfigs = {
          $schema: 'https://opencode.ai/configs.json',
          mcp: ( rulerMcpJson?.mcpServers ?? {} ) as Record<string, unknown>,
        };
      }
    }

    // Always write the configs file, even if MCP is empty
    await fs.writeFile( mcpPath, JSON.stringify( finalMcpconfigs, null, 2 ) );
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
