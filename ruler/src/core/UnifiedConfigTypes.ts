import
  {
    Mcpconfigs,
    Gitignoreconfigs,
    Skillsconfigs,
    McpStrategy,
  } from '../types';

export interface RulerUnifiedconfigs
{
  meta: configsMeta;
  toml: Tomlconfigs;
  rules: RulesBundle;
  mcp: McpBundle | null;
  agents: Record<string, EffectiveAgentconfigs>;
  diagnostics: configsDiagnostic[];
  hash: string;
}

export interface configsMeta
{
  projectRoot: string;
  rulerDir: string;
  configsFile?: string;
  mcpFile?: string;
  loadedAt: Date;
  version: string;
}

export interface Tomlconfigs
{
  raw: unknown;
  schemaVersion: number;
  defaultAgents?: string[];
  agents: Record<string, AgentTomlconfigs>;
  mcp?: McpToggleconfigs;
  mcpServers?: Record<string, McpServerDef>;
  gitignore?: Gitignoreconfigs;
  skills?: Skillsconfigs;
  nested?: boolean;
}

export type McpToggleconfigs = Mcpconfigs;

export interface AgentTomlconfigs
{
  enabled?: boolean;
  outputPath?: string;
  outputPathInstructions?: string;
  outputPathconfigs?: string;
  mcp?: Mcpconfigs;
  source: AgentconfigsSourceMeta;
}

export interface AgentconfigsSourceMeta
{
  sectionPath: string;
}

export interface RulesBundle
{
  files: RuleFile[];
  concatenated: string;
  concatenatedHash: string;
}

export interface RuleFile
{
  path: string;
  relativePath: string;
  content: string;
  contentHash: string;
  mtimeMs: number;
  size: number;
  order: number;
  primary: boolean;
}

export interface McpBundle
{
  servers: Record<string, McpServerDef>;
  raw: Record<string, unknown>;
  hash: string;
}

export interface McpServerDef
{
  type?: 'stdio' | 'local' | 'remote';
  command?: string;
  args?: string[];
  env?: Record<string, string>;
  url?: string;
  headers?: Record<string, string>;
}

export interface EffectiveAgentconfigs
{
  identifier: string;
  enabled: boolean;
  output: AgentOutputPaths;
  mcp: EffectiveMcpconfigs;
  toml?: AgentTomlconfigs;
}

export interface AgentOutputPaths
{
  instructions?: string;
  configs?: string;
  generic?: string;
}

export interface EffectiveMcpconfigs
{
  enabled: boolean;
  strategy: McpStrategy;
}

export type DiagnosticSeverity = 'info' | 'warning' | 'error';

export interface configsDiagnostic
{
  severity: DiagnosticSeverity;
  code: string;
  message: string;
  file?: string;
  detail?: string;
}
