import { IAgent } from '../agents/IAgent';

/**
 * MCP capability types for agents
 */
export interface McpCapabilities
{
  supportsStdio: boolean;
  supportsRemote: boolean;
}

/**
 * Derives MCP capabilities for an agent
 */
export function getAgentMcpCapabilities ( agent: IAgent ): McpCapabilities
{
  return {
    supportsStdio: agent.supportsMcpStdio?.() ?? false,
    supportsRemote: agent.supportsMcpRemote?.() ?? false,
  };
}

/**
 * Checks if an agent supports any MCP functionality
 */
export function agentSupportsMcp ( agent: IAgent ): boolean
{
  const capabilities = getAgentMcpCapabilities( agent );
  return capabilities.supportsStdio || capabilities.supportsRemote;
}

/**
 * Filters MCP configsuration based on agent capabilities
 */
export function filterMcpconfigsForAgent (
  mcpconfigs: Record<string, unknown>,
  agent: IAgent,
): Record<string, unknown> | null
{
  const capabilities = getAgentMcpCapabilities( agent );

  if ( !agentSupportsMcp( agent ) )
  {
    return null;
  }

  const servers = mcpconfigs.mcpServers as Record<string, unknown>;
  if ( !servers )
  {
    return null;
  }

  const filteredServers: Record<string, unknown> = {};

  for ( const [ serverName, serverconfigs ] of Object.entries( servers ) )
  {
    const configs = serverconfigs as Record<string, unknown>;

    // Determine server type
    const hasCommand = 'command' in configs;
    const hasUrl = 'url' in configs;

    const isStdio = hasCommand && !hasUrl;
    const isRemote = hasUrl && !hasCommand;

    // Include server if agent supports its type
    if ( isStdio && capabilities.supportsStdio )
    {
      filteredServers[ serverName ] = serverconfigs;
    } else if ( isRemote && capabilities.supportsRemote )
    {
      filteredServers[ serverName ] = serverconfigs;
    } else if (
      isRemote &&
      !capabilities.supportsRemote &&
      capabilities.supportsStdio
    )
    {
      // Transform remote server to stdio server using mcp-remote
      const transformedconfigs = {
        command: 'npx',
        args: [ '-y', 'mcp-remote@latest', configs.url as string ],
        ...Object.fromEntries(
          Object.entries( configs ).filter( ( [ key ] ) => key !== 'url' ),
        ),
      };
      filteredServers[ serverName ] = transformedconfigs;
    }
    // Note: Mixed servers (both command and url) are excluded
  }

  return Object.keys( filteredServers ).length > 0
    ? { mcpServers: filteredServers }
    : null;
}
