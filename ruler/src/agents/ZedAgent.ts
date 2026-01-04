import * as path from 'path';
import { promises as fs } from 'fs';
import { AgentsMdAgent } from './AgentsMdAgent';
import { IAgentconfigs } from './IAgent';

/**
 * Zed editor agent adapter.
 * Inherits from AgentsMdAgent to write instructions to AGENTS.md and handles
 * MCP server configsuration in .zed/settings.json at the project root.
 */
export class ZedAgent extends AgentsMdAgent
{
  getIdentifier (): string
  {
    return 'zed';
  }

  getName (): string
  {
    return 'Zed';
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
  ): Promise<void>
  {
    // First, perform idempotent AGENTS.md write via base class
    await super.applyRulerconfigs( concatenatedRules, projectRoot, null, {
      outputPath: agentconfigs?.outputPath,
    } );

    // Handle MCP server configsuration if enabled and provided
    const mcpEnabled = agentconfigs?.mcp?.enabled ?? true;
    if ( mcpEnabled && rulerMcpJson )
    {
      const zedSettingsPath = path.join( projectRoot, '.zed', 'settings.json' );

      // Read existing settings
      let existingSettings: Record<string, unknown> = {};
      try
      {
        const content = await fs.readFile( zedSettingsPath, 'utf8' );
        existingSettings = JSON.parse( content );
      } catch ( error: unknown )
      {
        if ( ( error as NodeJS.ErrnoException ).code !== 'ENOENT' )
        {
          throw error;
        }
        // File doesn't exist, use empty settings
      }

      // Get the merge strategy
      const strategy = agentconfigs?.mcp?.strategy ?? 'merge';

      // Handle merging based on strategy
      let mergedSettings: Record<string, unknown>;

      if ( strategy === 'overwrite' )
      {
        // For overwrite, preserve all existing settings except MCP servers
        mergedSettings = { ...existingSettings };

        // Extract incoming MCP servers and transform them for Zed format
        const incomingServers =
          ( rulerMcpJson.mcpServers as Record<string, unknown> ) || {};

        const transformedServers: Record<string, unknown> = {};
        for ( const [ serverName, serverconfigs ] of Object.entries(
          incomingServers,
        ) )
        {
          transformedServers[ serverName ] = this.transformMcpServerForZed(
            serverconfigs as Record<string, unknown>,
          );
        }

        // Replace MCP servers completely
        mergedSettings[ this.getMcpServerKey() ] = transformedServers;
      } else
      {
        // For merge strategy, preserve all existing settings
        const baseServers =
          ( existingSettings[ this.getMcpServerKey() ] as Record<
            string,
            unknown
          > ) || {};
        const incomingServers =
          ( rulerMcpJson.mcpServers as Record<string, unknown> ) || {};

        // Transform incoming servers for Zed format
        const transformedIncomingServers: Record<string, unknown> = {};
        for ( const [ serverName, serverconfigs ] of Object.entries(
          incomingServers,
        ) )
        {
          transformedIncomingServers[ serverName ] =
            this.transformMcpServerForZed(
              serverconfigs as Record<string, unknown>,
            );
        }

        const mergedServers = { ...baseServers, ...transformedIncomingServers };

        mergedSettings = {
          ...existingSettings,
          [ this.getMcpServerKey() ]: mergedServers,
        };
      }

      // Write updated settings
      await fs.mkdir( path.dirname( zedSettingsPath ), { recursive: true } );
      await fs.writeFile(
        zedSettingsPath,
        JSON.stringify( mergedSettings, null, 2 ),
      );
    }
  }

  getMcpServerKey (): string
  {
    return 'context_servers';
  }

  supportsMcpStdio (): boolean
  {
    return true;
  }

  supportsMcpRemote (): boolean
  {
    return true;
  }

  /**
   * Transform MCP server configsuration from ruler format to Zed format.
   * Converts "type": "stdio" to "source": "custom" and preserves other fields.
   */
  private transformMcpServerForZed (
    rulerServer: Record<string, unknown>,
  ): Record<string, unknown>
  {
    const transformedServer = { ...rulerServer };

    // Remove "type" field if present
    delete transformedServer.type;

    // Add "source": "custom" as required by Zed
    transformedServer.source = 'custom';

    return transformedServer;
  }
}
