import * as fs from 'fs/promises';
import * as path from 'path';
import * as os from 'os';
import { CodexCliAgent } from '../../src/agents/CodexCliAgent';

describe( 'TOML Edge Cases', () =>
{
  let tmpDir: string;
  let agent: CodexCliAgent;

  beforeEach( async () =>
  {
    tmpDir = await fs.mkdtemp( path.join( os.tmpdir(), 'ruler-toml-edge-test-' ) );
    agent = new CodexCliAgent();
  } );

  afterEach( async () =>
  {
    await fs.rm( tmpDir, { recursive: true, force: true } );
  } );

  it( 'should handle special characters in MCP server values', async () =>
  {
    // Test data with edge case values that could break manual TOML serialization
    const rulerMcp = {
      mcpServers: {
        'special-chars': {
          command: 'echo "quoted command"',
          args: [ 'arg with spaces', 'arg"with"quotes', 'arg,with,commas', 'unicode: 🚀' ],
          env: {
            'VAR_WITH_QUOTES': 'value "with" quotes',
            'VAR_WITH_COMMA': 'value,with,comma',
            'VAR_WITH_UNICODE': '🌟 unicode value',
            'VAR_WITH_NEWLINE': 'line1\nline2',
          }
        },
        'empty-arrays': {
          command: 'test-empty',
          args: [],
          env: {}
        },
        'complex-nested': {
          command: 'complex',
          args: [ 'path/to/file.txt', '--flag=value' ],
          env: {
            'NESTED_JSON': '{"key": "value", "count": 42}',
            'PATH_WITH_SPACES': '/path with spaces/to/file',
          },
          headers: {
            'Authorization': 'Bearer token-with-special-chars!@#$%',
            'Content-Type': 'application/json; charset=utf-8',
            'X-Custom-Header': 'value with "quotes" and \'single quotes\''
          }
        }
      }
    };

    // Apply the configsuration
    await agent.applyRulerconfigs(
      '# Test rules',
      tmpDir,
      rulerMcp,
      { mcp: { strategy: 'merge' } },
      false
    );

    const configsPath = path.join( tmpDir, '.codex', 'configs.toml' );
    const configsContent = await fs.readFile( configsPath, 'utf8' );

    console.log( 'Generated TOML:' );
    console.log( configsContent );
    console.log( '---' );

    // Verify the file was created
    expect( configsContent ).toBeDefined();
    expect( configsContent.length ).toBeGreaterThan( 0 );

    // Verify it contains our server sections
    expect( configsContent ).toContain( '[mcp_servers.special-chars]' );
    expect( configsContent ).toContain( '[mcp_servers.empty-arrays]' );
    expect( configsContent ).toContain( '[mcp_servers.complex-nested]' );

    // Verify command values are properly escaped
    expect( configsContent ).toContain( 'echo "quoted command"' );

    // Most importantly: verify we can parse it back without errors
    const { parse } = require( '@iarna/toml' );
    let parsedconfigs: any;
    expect( () =>
    {
      parsedconfigs = parse( configsContent );
    } ).not.toThrow();

    // Verify the parsed configs has the expected structure
    expect( parsedconfigs ).toHaveProperty( 'mcp_servers' );
    expect( parsedconfigs.mcp_servers ).toHaveProperty( 'special-chars' );
    expect( parsedconfigs.mcp_servers ).toHaveProperty( 'empty-arrays' );
    expect( parsedconfigs.mcp_servers ).toHaveProperty( 'complex-nested' );

    // Verify specific edge case values were preserved correctly
    const specialCharsServer = parsedconfigs.mcp_servers[ 'special-chars' ];
    expect( specialCharsServer.command ).toBe( 'echo "quoted command"' );
    expect( specialCharsServer.args ).toEqual( [
      'arg with spaces',
      'arg"with"quotes',
      'arg,with,commas',
      'unicode: 🚀'
    ] );
    expect( specialCharsServer.env[ 'VAR_WITH_QUOTES' ] ).toBe( 'value "with" quotes' );
    expect( specialCharsServer.env[ 'VAR_WITH_UNICODE' ] ).toBe( '🌟 unicode value' );

    const complexServer = parsedconfigs.mcp_servers[ 'complex-nested' ];
    expect( complexServer.headers[ 'Authorization' ] ).toBe( 'Bearer token-with-special-chars!@#$%' );
    expect( complexServer.headers[ 'X-Custom-Header' ] ).toBe( 'value with "quotes" and \'single quotes\'' );
    expect( complexServer.env[ 'NESTED_JSON' ] ).toBe( '{"key": "value", "count": 42}' );
  } );

  it( 'should handle empty and minimal configsurations', async () =>
  {
    const rulerMcp = {
      mcpServers: {
        'minimal': {
          command: 'minimal-cmd'
          // No args, env, or headers
        }
      }
    };

    await agent.applyRulerconfigs(
      '# Minimal test',
      tmpDir,
      rulerMcp,
      { mcp: { strategy: 'merge' } },
      false
    );

    const configsPath = path.join( tmpDir, '.codex', 'configs.toml' );
    const configsContent = await fs.readFile( configsPath, 'utf8' );

    // Should parse without errors
    const { parse } = require( '@iarna/toml' );
    const parsedconfigs = parse( configsContent );

    expect( parsedconfigs.mcp_servers.minimal.command ).toBe( 'minimal-cmd' );
    expect( parsedconfigs.mcp_servers.minimal.args ).toBeUndefined();
    expect( parsedconfigs.mcp_servers.minimal.env ).toBeUndefined();
  } );

  it( 'should handle server names with special characters', async () =>
  {
    const rulerMcp = {
      mcpServers: {
        'server-with-dashes': {
          command: 'cmd1'
        },
        'server_with_underscores': {
          command: 'cmd2'
        },
        'server.with.dots': {
          command: 'cmd3'
        }
      }
    };

    await agent.applyRulerconfigs(
      '# Special name test',
      tmpDir,
      rulerMcp,
      { mcp: { strategy: 'merge' } },
      false
    );

    const configsPath = path.join( tmpDir, '.codex', 'configs.toml' );
    const configsContent = await fs.readFile( configsPath, 'utf8' );

    // Should parse without errors
    const { parse } = require( '@iarna/toml' );
    const parsedconfigs = parse( configsContent );

    expect( parsedconfigs.mcp_servers ).toHaveProperty( 'server-with-dashes' );
    expect( parsedconfigs.mcp_servers ).toHaveProperty( 'server_with_underscores' );
    expect( parsedconfigs.mcp_servers[ 'server.with.dots' ] ).toBeDefined();
    expect( parsedconfigs.mcp_servers[ 'server.with.dots' ].command ).toBe( 'cmd3' );
  } );
} );