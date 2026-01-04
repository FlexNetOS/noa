import { IAgentconfigs } from './IAgent';
import * as path from 'path';
import { promises as fs } from 'fs';
import { AgentsMdAgent } from './AgentsMdAgent';

export class QwenCodeAgent extends AgentsMdAgent
{
  getIdentifier (): string
  {
    return 'qwen';
  }

  getName (): string
  {
    return 'Qwen Code';
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    _rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
  ): Promise<void>
  {
    // First, perform idempotent write of AGENTS.md via base class
    await super.applyRulerconfigs( concatenatedRules, projectRoot, null, {
      outputPath: agentconfigs?.outputPath,
    } );

    // Ensure .qwen/settings.json has contextFileName set to AGENTS.md
    const settingsPath = path.join( projectRoot, '.qwen', 'settings.json' );
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

    await fs.mkdir( path.dirname( settingsPath ), { recursive: true } );
    await fs.writeFile( settingsPath, JSON.stringify( updated, null, 2 ) );
  }

  // Ensure MCP merging uses the correct key for Qwen Code (.qwen/settings.json)
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
