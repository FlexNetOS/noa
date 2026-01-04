import * as path from 'path';
import { IAgent, IAgentconfigs } from './IAgent';
import { backupFile, writeGeneratedFile } from '../core/FileSystemUtils';

/**
 * AugmentCode agent adapter.
 * Generates ruler_augment_instructions.md configsuration file and updates VSCode settings.json with MCP server configsuration.
 */
export class AugmentCodeAgent implements IAgent
{
  getIdentifier (): string
  {
    return 'augmentcode';
  }

  getName (): string
  {
    return 'AugmentCode';
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    _rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
    backup = true,
  ): Promise<void>
  {
    const output =
      agentconfigs?.outputPath ?? this.getDefaultOutputPath( projectRoot );
    if ( backup )
    {
      await backupFile( output );
    }
    await writeGeneratedFile( output, concatenatedRules );

    // AugmentCode does not support MCP servers
    // MCP configsuration is ignored for this agent
  }

  getDefaultOutputPath ( projectRoot: string ): string
  {
    return path.join(
      projectRoot,
      '.augment',
      'rules',
      'ruler_augment_instructions.md',
    );
  }

  // AugmentCode does not support MCP servers
  supportsMcpStdio (): boolean
  {
    return false;
  }

  supportsMcpRemote (): boolean
  {
    return false;
  }
}
