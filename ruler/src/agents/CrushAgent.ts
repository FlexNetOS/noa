import { IAgent, IAgentconfigs } from './IAgent';
import * as fs from 'fs/promises';
import * as path from 'path';

export class CrushAgent implements IAgent
{
  getIdentifier (): string
  {
    return 'crush';
  }

  getName (): string
  {
    return 'Crush';
  }

  getDefaultOutputPath ( projectRoot: string ): Record<string, string>
  {
    return {
      instructions: path.join( projectRoot, 'CRUSH.md' ),
      mcp: path.join( projectRoot, '.crush.json' ),
    };
  }

  /**
   * Transform MCP server types for Crush compatibility.
   * Crush expects "http" for HTTP servers and "sse" for SSE servers, not "remote".
   */
  private transformMcpServersForCrush (
    mcpServers: Record<string, unknown>,
  ): Record<string, unknown>
  {
    const transformedServers: Record<string, unknown> = {};

    for ( const [ name, serverDef ] of Object.entries( mcpServers ) )
    {
      if ( serverDef && typeof serverDef === 'object' )
      {
        const server = serverDef as Record<string, unknown>;
        const transformedServer = { ...server };

        // Transform type: "remote" to appropriate Crush types
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

    return transformedServers;
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
  ): Promise<void>
  {
    const outputPaths = this.getDefaultOutputPath( projectRoot );
    const instructionsPath =
      agentconfigs?.outputPathInstructions ?? outputPaths[ 'instructions' ];
    const mcpPath = agentconfigs?.outputPathconfigs ?? outputPaths[ 'mcp' ];

    await fs.writeFile( instructionsPath, concatenatedRules );

    // Always transform from mcpServers ({ mcpServers: ... }) to { mcp: ... } for Crush
    let finalMcpconfigs: { mcp: Record<string, unknown>; } = { mcp: {} };

    try
    {
      const existingMcpconfigs = JSON.parse( await fs.readFile( mcpPath, 'utf-8' ) );
      if ( existingMcpconfigs && typeof existingMcpconfigs === 'object' )
      {
        const transformedServers = this.transformMcpServersForCrush(
          ( rulerMcpJson?.mcpServers ?? {} ) as Record<string, unknown>,
        );
        finalMcpconfigs = {
          ...existingMcpconfigs,
          mcp: {
            ...( existingMcpconfigs.mcp || {} ),
            ...transformedServers,
          },
        };
      } else if ( rulerMcpJson )
      {
        const transformedServers = this.transformMcpServersForCrush(
          ( rulerMcpJson?.mcpServers ?? {} ) as Record<string, unknown>,
        );
        finalMcpconfigs = {
          mcp: transformedServers,
        };
      }
    } catch
    {
      if ( rulerMcpJson )
      {
        const transformedServers = this.transformMcpServersForCrush(
          ( rulerMcpJson?.mcpServers ?? {} ) as Record<string, unknown>,
        );
        finalMcpconfigs = {
          mcp: transformedServers,
        };
      }
    }

    if ( Object.keys( finalMcpconfigs.mcp ).length > 0 )
    {
      await fs.writeFile( mcpPath, JSON.stringify( finalMcpconfigs, null, 2 ) );
    }
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
