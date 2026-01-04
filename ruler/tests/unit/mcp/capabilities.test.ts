import { getAgentMcpCapabilities, agentSupportsMcp, filterMcpconfigsForAgent } from '../../../src/mcp/capabilities';
import { OpenHandsAgent } from '../../../src/agents/OpenHandsAgent';
import { AugmentCodeAgent } from '../../../src/agents/AugmentCodeAgent';
import { CopilotAgent } from '../../../src/agents/CopilotAgent';
import { CodexCliAgent } from '../../../src/agents/CodexCliAgent';

describe( 'MCP Capabilities', () =>
{
  describe( 'getAgentMcpCapabilities', () =>
  {
    it( 'returns correct capabilities for OpenHands (both stdio and remote)', () =>
    {
      const agent = new OpenHandsAgent();
      const capabilities = getAgentMcpCapabilities( agent );

      expect( capabilities.supportsStdio ).toBe( true );
      expect( capabilities.supportsRemote ).toBe( true );
    } );

    it( 'returns correct capabilities for AugmentCode (no MCP support)', () =>
    {
      const agent = new AugmentCodeAgent();
      const capabilities = getAgentMcpCapabilities( agent );

      expect( capabilities.supportsStdio ).toBe( false );
      expect( capabilities.supportsRemote ).toBe( false );
    } );

    it( 'returns correct capabilities for Copilot (both stdio and remote)', () =>
    {
      const agent = new CopilotAgent();
      const capabilities = getAgentMcpCapabilities( agent );

      expect( capabilities.supportsStdio ).toBe( true );
      expect( capabilities.supportsRemote ).toBe( true );
    } );

    it( 'returns correct capabilities for CodexCli (stdio only)', () =>
    {
      const agent = new CodexCliAgent();
      const capabilities = getAgentMcpCapabilities( agent );

      expect( capabilities.supportsStdio ).toBe( true );
      expect( capabilities.supportsRemote ).toBe( false );
    } );
  } );

  describe( 'agentSupportsMcp', () =>
  {
    it( 'returns true for agents that support MCP', () =>
    {
      expect( agentSupportsMcp( new OpenHandsAgent() ) ).toBe( true );
      expect( agentSupportsMcp( new CopilotAgent() ) ).toBe( true );
      expect( agentSupportsMcp( new CodexCliAgent() ) ).toBe( true );
    } );

    it( 'returns false for agents that do not support MCP', () =>
    {
      expect( agentSupportsMcp( new AugmentCodeAgent() ) ).toBe( false );
    } );
  } );

  describe( 'filterMcpconfigsForAgent', () =>
  {
    const testMcpconfigs = {
      mcpServers: {
        stdio_server: {
          command: 'npx',
          args: [ '-y', 'server-filesystem', '/tmp' ]
        },
        remote_server: {
          url: 'https://api.example.com/mcp'
        },
        mixed_server: {
          command: 'npx',
          url: 'https://api.example.com/mcp'  // Invalid, but tests edge case
        }
      }
    };

    it( 'returns all servers for agents that support both stdio and remote', () =>
    {
      const agent = new OpenHandsAgent();
      const filtered = filterMcpconfigsForAgent( testMcpconfigs, agent );

      expect( filtered ).not.toBeNull();
      expect( filtered!.mcpServers ).toEqual( {
        stdio_server: testMcpconfigs.mcpServers.stdio_server,
        remote_server: testMcpconfigs.mcpServers.remote_server
      } );
      // mixed_server should be excluded as it has both command and url
    } );

    it( 'returns null for agents that do not support MCP', () =>
    {
      const agent = new AugmentCodeAgent();
      const filtered = filterMcpconfigsForAgent( testMcpconfigs, agent );

      expect( filtered ).toBeNull();
    } );

    it( 'returns null when no servers are provided', () =>
    {
      const agent = new OpenHandsAgent();
      const emptyconfigs = { mcpServers: {} };
      const filtered = filterMcpconfigsForAgent( emptyconfigs, agent );

      expect( filtered ).toBeNull();
    } );

    it( 'filters servers based on agent capabilities (stdio only)', () =>
    {
      const agent = new CodexCliAgent();
      const filtered = filterMcpconfigsForAgent( testMcpconfigs, agent );

      expect( filtered ).not.toBeNull();
      expect( filtered!.mcpServers ).toEqual( {
        stdio_server: testMcpconfigs.mcpServers.stdio_server,
        // remote_server should be transformed to stdio server using mcp-remote
        remote_server: {
          command: 'npx',
          args: [ '-y', 'mcp-remote@latest', 'https://api.example.com/mcp' ]
        }
        // mixed_server should still be excluded as it has both command and url
      } );
    } );

    it( 'returns null when mcpServers is not present', () =>
    {
      const agent = new OpenHandsAgent();
      const invalidconfigs = { somethingElse: {} };
      const filtered = filterMcpconfigsForAgent( invalidconfigs, agent );

      expect( filtered ).toBeNull();
    } );

    it( 'transforms remote servers to stdio servers for stdio-only agents', () =>
    {
      const agent = new CodexCliAgent();
      const configsWithRemoteServers = {
        mcpServers: {
          simple_remote: {
            url: 'https://example.com/mcp'
          },
          remote_with_headers: {
            url: 'https://api.example.com/mcp',
            headers: {
              'Authorization': 'Bearer token123',
              'Content-Type': 'application/json'
            }
          },
          stdio_server: {
            command: 'node',
            args: [ 'server.js' ]
          }
        }
      };

      const filtered = filterMcpconfigsForAgent( configsWithRemoteServers, agent );

      expect( filtered ).not.toBeNull();
      expect( filtered!.mcpServers ).toEqual( {
        simple_remote: {
          command: 'npx',
          args: [ '-y', 'mcp-remote@latest', 'https://example.com/mcp' ]
        },
        remote_with_headers: {
          command: 'npx',
          args: [ '-y', 'mcp-remote@latest', 'https://api.example.com/mcp' ],
          // Note: headers should be preserved as env variables or similar mechanism
          headers: {
            'Authorization': 'Bearer token123',
            'Content-Type': 'application/json'
          }
        },
        stdio_server: {
          command: 'node',
          args: [ 'server.js' ]
        }
      } );
    } );

    it( 'does not transform remote servers for agents that support both stdio and remote', () =>
    {
      const agent = new OpenHandsAgent();  // Supports both stdio and remote
      const configsWithRemoteServers = {
        mcpServers: {
          remote_server: {
            url: 'https://example.com/mcp'
          },
          stdio_server: {
            command: 'node',
            args: [ 'server.js' ]
          }
        }
      };

      const filtered = filterMcpconfigsForAgent( configsWithRemoteServers, agent );

      expect( filtered ).not.toBeNull();
      expect( filtered!.mcpServers ).toEqual( {
        remote_server: {
          url: 'https://example.com/mcp'
        },
        stdio_server: {
          command: 'node',
          args: [ 'server.js' ]
        }
      } );
    } );
  } );
} );