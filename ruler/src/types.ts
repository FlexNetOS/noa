/**
 * Types for Model Context Protocol (MCP) server configsuration.
 */
export type McpStrategy = 'merge' | 'overwrite';

/** MCP configsuration for an agent or global. */
export interface Mcpconfigs
{
  /** Enable or disable MCP propagation (merge or overwrite). */
  enabled?: boolean;
  /** Merge strategy: 'merge' to merge servers, 'overwrite' to replace configs. */
  strategy?: McpStrategy;
}

/** Global MCP configsuration section (same as agent-specific configs). */
export type GlobalMcpconfigs = Mcpconfigs;

/** Gitignore configsuration for automatic .gitignore file updates. */
export interface Gitignoreconfigs
{
  /** Enable or disable automatic .gitignore updates. */
  enabled?: boolean;
}

/** Skills configsuration for automatic skills distribution. */
export interface Skillsconfigs
{
  /** Enable or disable skills support. */
  enabled?: boolean;
}

/** Information about a discovered skill. */
export interface SkillInfo
{
  /** Name of the skill (directory name). */
  name: string;
  /** Absolute path to the skill directory. */
  path: string;
  /** Whether the directory contains a SKILL.md file. */
  hasSkillMd: boolean;
  /** Whether this is a valid skill. */
  valid: boolean;
  /** Error message if invalid. */
  error?: string;
}
