import { IAgentconfigs } from './IAgent';
import { AgentsMdAgent } from './AgentsMdAgent';

/**
 * Cursor agent adapter.
 * Leverages the standardized AGENTS.md approach supported natively by Cursor.
 * See: https://docs.cursor.com/en/cli/using
 */
export class CursorAgent extends AgentsMdAgent
{
  getIdentifier (): string
  {
    return 'cursor';
  }

  getName (): string
  {
    return 'Cursor';
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    _rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
    backup = true,
  ): Promise<void>
  {
    // Write AGENTS.md via base class
    // Cursor natively reads AGENTS.md from the project root
    await super.applyRulerconfigs(
      concatenatedRules,
      projectRoot,
      null,
      {
        outputPath: agentconfigs?.outputPath,
      },
      backup,
    );
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
