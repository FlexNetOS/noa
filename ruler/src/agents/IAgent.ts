/**
 * Interface defining an AI agent configsuration adapter.
 */
import { Mcpconfigs } from '../types';
/**
 * configsuration overrides for a specific agent.
 */
export interface IAgentconfigs
{
  /** Explicit enable/disable agent */
  enabled?: boolean;
  /** Override for primary output path */
  outputPath?: string;
  /** Override for Aider instruction file path */
  outputPathInstructions?: string;
  /** Override for Aider configs file path */
  outputPathconfigs?: string;
  /** MCP propagation configs for this agent. */
  mcp?: Mcpconfigs;
}

export interface IAgent
{
  /**
   * Returns the lowercase identifier of the agent (e.g., "copilot", "claude", "aider").
   */
  getIdentifier (): string;

  /**
   * Returns the display name of the agent.
   */
  getName (): string;

  /**
   * Applies the concatenated ruler rules to the agent's configsuration.
   * @param concatenatedRules The combined rules text
   * @param projectRoot The root directory of the project
   */
  applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
    backup?: boolean,
  ): Promise<void>;

  /**
   * Returns the default output path(s) for this agent given the project root.
   */
  getDefaultOutputPath ( projectRoot: string ): string | Record<string, string>;

  /**
   * Returns the specific key to be used for the server object in MCP JSON.
   * Defaults to 'mcpServers' if not implemented.
   */
  getMcpServerKey?(): string;

  /**
   * Returns whether this agent supports MCP STDIO servers.
   * Defaults to false if not implemented.
   */
  supportsMcpStdio?(): boolean;

  /**
   * Returns whether this agent supports MCP remote servers.
   * Defaults to false if not implemented.
   */
  supportsMcpRemote?(): boolean;

  /**
   * Returns whether this agent has native skills support (like Claude Code).
   * When true, skills are copied directly to the agent's skills directory.
   * Defaults to false if not implemented.
   */
  supportsNativeSkills?(): boolean;
}
