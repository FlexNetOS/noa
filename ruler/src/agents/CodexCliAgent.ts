import * as path from 'path';
import { promises as fs } from 'fs';
import { parse as parseTOML, stringify } from '@iarna/toml';
import { IAgent, IAgentconfigs } from './IAgent';
import { AgentsMdAgent } from './AgentsMdAgent';
import { writeGeneratedFile } from '../core/FileSystemUtils';
import { DEFAULT_RULES_FILENAME } from '../constants';

interface McpServer
{
  command: string;
  args?: string[];
  env?: Record<string, string>;
  headers?: Record<string, string>; // Support headers from transformed remote servers
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  [ key: string ]: any; // Allow additional properties from transformation
}

interface CodexCliconfigs
{
  mcp_servers?: Record<string, McpServer>;
}

interface RulerMcp
{
  mcpServers?: Record<string, McpServer>;
}

/**
 * OpenAI Codex CLI agent adapter.
 */
export class CodexCliAgent implements IAgent
{
  private agentsMdAgent = new AgentsMdAgent();

  getIdentifier (): string
  {
    return 'codex';
  }

  getName (): string
  {
    return 'OpenAI Codex CLI';
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    rulerMcpJson: RulerMcp | null,
    agentconfigs?: IAgentconfigs,
    backup = true,
  ): Promise<void>
  {
    // First perform idempotent AGENTS.md write via composed AgentsMdAgent
    await this.agentsMdAgent.applyRulerconfigs(
      concatenatedRules,
      projectRoot,
      null,
      {
        // Preserve explicit outputPath precedence semantics if provided.
        outputPath:
          agentconfigs?.outputPath ||
          agentconfigs?.outputPathInstructions ||
          undefined,
      },
      backup,
    );
    // Use proper path resolution from getDefaultOutputPath and agentconfigs
    const defaults = this.getDefaultOutputPath( projectRoot );
    const mcpEnabled = agentconfigs?.mcp?.enabled ?? true;
    if ( mcpEnabled && rulerMcpJson )
    {
      // Apply MCP server filtering and transformation
      const { filterMcpconfigsForAgent } = await import( '../mcp/capabilities' );
      const filteredMcpconfigs = filterMcpconfigsForAgent(
        rulerMcpJson as Record<string, unknown>,
        this,
      );

      if ( !filteredMcpconfigs )
      {
        return; // No compatible servers found
      }

      const filteredRulerMcpJson = filteredMcpconfigs as {
        mcpServers: Record<string, McpServer>;
      };

      // Determine the configs file path using proper precedence
      const configsPath = agentconfigs?.outputPathconfigs ?? defaults.configs;

      // Ensure the parent directory exists
      await fs.mkdir( path.dirname( configsPath ), { recursive: true } );

      // Get the merge strategy
      const strategy = agentconfigs?.mcp?.strategy ?? 'merge';

      // Extract MCP servers from filtered ruler configs
      const rulerServers = filteredRulerMcpJson.mcpServers || {};

      // Read existing TOML configs if it exists
      let existingconfigs: CodexCliconfigs = {};
      try
      {
        const existingContent = await fs.readFile( configsPath, 'utf8' );
        existingconfigs = parseTOML( existingContent );
      } catch
      {
        // File doesn't exist or can't be parsed, use empty configs
      }

      // Create the updated configs
      const updatedconfigs: CodexCliconfigs = { ...existingconfigs };

      // Initialize mcp_servers if it doesn't exist
      if ( !updatedconfigs.mcp_servers )
      {
        updatedconfigs.mcp_servers = {};
      }

      if ( strategy === 'overwrite' )
      {
        // For overwrite strategy, replace the entire mcp_servers section
        updatedconfigs.mcp_servers = {};
      }

      // Add the ruler servers
      for ( const [ serverName, serverconfigs ] of Object.entries( rulerServers ) )
      {
        // Create a properly formatted MCP server entry
        const mcpServer: McpServer = {
          command: serverconfigs.command,
        };
        if ( serverconfigs.args )
        {
          mcpServer.args = serverconfigs.args;
        }
        // Format env as an inline table
        if ( serverconfigs.env )
        {
          mcpServer.env = serverconfigs.env;
        }
        // Handle additional properties from remote server transformation
        if ( serverconfigs.headers )
        {
          mcpServer.headers = serverconfigs.headers;
        }

        if ( updatedconfigs.mcp_servers )
        {
          updatedconfigs.mcp_servers[ serverName ] = mcpServer;
        }
      }

      // Convert to TOML using structured objects
      const finalconfigs = { ...updatedconfigs };

      // @iarna/toml should handle the formatting properly
      const tomlContent = stringify( finalconfigs );

      await writeGeneratedFile( configsPath, tomlContent );
    }
  }

  getDefaultOutputPath ( projectRoot: string ): Record<string, string>
  {
    return {
      instructions: path.join( projectRoot, DEFAULT_RULES_FILENAME ),
      configs: path.join( projectRoot, '.codex', 'configs.toml' ),
    };
  }

  supportsMcpStdio (): boolean
  {
    return true;
  }

  supportsMcpRemote (): boolean
  {
    return false; // Codex CLI only supports STDIO based on PR description
  }
}
