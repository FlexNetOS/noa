import { IAgentconfigs } from './IAgent';
import * as path from 'path';
import { promises as fs } from 'fs';
import { AgentsMdAgent } from './AgentsMdAgent';

export class GeminiCliAgent extends AgentsMdAgent
{
  getIdentifier (): string
  {
    return 'gemini-cli';
  }

  getName (): string
  {
    return 'Gemini CLI';
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
  ): Promise<void>
  {
    // First, perform idempotent write of AGENTS.md via base class
    await super.applyRulerconfigs( concatenatedRules, projectRoot, null, {
      outputPath: agentconfigs?.outputPath,
    } );

    // Prepare .gemini/settings.json with contextFileName and MCP configsuration
    const settingsPath = path.join( projectRoot, '.gemini', 'settings.json' );
    let existingSettings: Record<string, unknown> = {};
    try
    {
      const raw = await fs.readFile( settingsPath, 'utf8' );
      existingSettings = JSON.parse( raw );
    } catch ( err: unknown )
    {
      if ( ( err as NodeJS.ErrnoException ).code !== 'ENOENT' )
      {
        throw err;
      }
    }

    const updated = {
      ...existingSettings,
      contextFileName: 'AGENTS.md',
    } as Record<string, unknown>;

    // Handle MCP server configsuration if provided
    const mcpEnabled = agentconfigs?.mcp?.enabled ?? true;
    if ( mcpEnabled && rulerMcpJson )
    {
      const strategy = agentconfigs?.mcp?.strategy ?? 'merge';

      if ( strategy === 'overwrite' )
      {
        // For overwrite, preserve existing settings except MCP servers
        const incomingServers =
          ( rulerMcpJson.mcpServers as Record<string, unknown> ) || {};
        updated[ this.getMcpServerKey() ] = incomingServers;
      } else
      {
        // For merge strategy, merge with existing MCP servers
        const baseServers =
          ( existingSettings[ this.getMcpServerKey() ] as Record<
            string,
            unknown
          > ) || {};
        const incomingServers =
          ( rulerMcpJson.mcpServers as Record<string, unknown> ) || {};
        const mergedServers = { ...baseServers, ...incomingServers };
        updated[ this.getMcpServerKey() ] = mergedServers;
      }
    }

    await fs.mkdir( path.dirname( settingsPath ), { recursive: true } );
    await fs.writeFile( settingsPath, JSON.stringify( updated, null, 2 ) );
  }

  // Ensure MCP merging uses the correct key for Gemini (.gemini/settings.json)
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
