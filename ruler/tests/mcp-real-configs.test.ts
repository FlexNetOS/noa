// Comprehensive test to validate MCP propagation uses actual configss, not examples
import * as fs from 'fs/promises';
import * as path from 'path';
import os from 'os';
import { applyAllAgentconfigss } from '../src/lib';

describe( 'MCP Propagation Integration - Real vs Example configss', () =>
{
  let tmpDir: string;

  beforeEach( async () =>
  {
    tmpDir = await fs.mkdtemp( path.join( os.tmpdir(), 'mcp-real-test-' ) );
  } );

  afterEach( async () =>
  {
    await fs.rm( tmpDir, { recursive: true, force: true } );
  } );

  it( 'should propagate actual TOML configsurations, never example configss', async () =>
  {
    // Create .ruler directory with custom MCP servers
    const rulerDir = path.join( tmpDir, '.ruler' );
    await fs.mkdir( rulerDir );

    // Create AGENTS.md 
    await fs.writeFile( path.join( rulerDir, 'AGENTS.md' ), '# Test instructions' );

    // Create ruler.toml with REAL, user-defined MCP servers (not examples)
    const tomlContent = `
[mcp_servers.user_real_filesystem]
command = "npx"
args = ["-y", "real-filesystem-server", "--custom-flag"]

[mcp_servers.user_real_api]  
command = "uvx"
args = ["real-api-server"]
env = { API_TOKEN = "real-token-12345" }
`;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), tomlContent );

    // Apply ruler configsuration for OpenHands
    await applyAllAgentconfigss(
      tmpDir,
      [ 'openhands' ],
      undefined,
      true, // mcp enabled
      undefined,
      undefined,
      false, // verbose
      false, // dry run
      true   // local only
    );

    // Verify OpenHands configs exists and contains ONLY the user-defined servers
    const openHandsconfigsPath = path.join( tmpDir, 'configs.toml' );
    const configsExists = await fs.access( openHandsconfigsPath ).then( () => true ).catch( () => false );
    expect( configsExists ).toBe( true );

    const configsContent = await fs.readFile( openHandsconfigsPath, 'utf8' );

    // Should contain the REAL user-defined servers
    expect( configsContent ).toContain( 'user_real_filesystem' );
    expect( configsContent ).toContain( 'user_real_api' );
    expect( configsContent ).toContain( 'real-filesystem-server' );
    expect( configsContent ).toContain( 'real-api-server' );
    expect( configsContent ).toContain( 'API_TOKEN = "real-token-12345"' );

    // Should NOT contain any example server names or commands
    expect( configsContent ).not.toContain( 'example_stdio' );
    expect( configsContent ).not.toContain( 'example_remote' );
    expect( configsContent ).not.toContain( 'filesystem_server' ); // from integration test
    expect( configsContent ).not.toContain( 'remote_api' ); // from integration test
    expect( configsContent ).not.toContain( 'scripts/your-mcp-server.js' );
    expect( configsContent ).not.toContain( 'api.example.com' );
  } );

  it( 'should merge TOML and JSON sources correctly, not use examples', async () =>
  {
    // Create .ruler directory
    const rulerDir = path.join( tmpDir, '.ruler' );
    await fs.mkdir( rulerDir );

    await fs.writeFile( path.join( rulerDir, 'AGENTS.md' ), '# Test instructions' );

    // Create ruler.toml with one server
    const tomlContent = `
[mcp_servers.toml_server]
command = "npx"
args = ["toml-mcp-tool"]
`;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), tomlContent );

    // Create legacy mcp.json with another server
    const mcpJson = {
      mcpServers: {
        json_server: {
          command: "uvx",
          args: [ "json-mcp-tool" ]
        }
      }
    };
    await fs.writeFile( path.join( rulerDir, 'mcp.json' ), JSON.stringify( mcpJson ) );

    // Apply ruler configsuration for OpenHands
    await applyAllAgentconfigss(
      tmpDir,
      [ 'openhands' ],
      undefined,
      true, // mcp enabled
      undefined,
      undefined,
      false, // verbose
      false, // dry run
      true   // local only
    );

    const openHandsconfigsPath = path.join( tmpDir, 'configs.toml' );
    const configsContent = await fs.readFile( openHandsconfigsPath, 'utf8' );

    // Should contain BOTH user-defined servers from merged sources
    expect( configsContent ).toContain( 'toml_server' );
    expect( configsContent ).toContain( 'json_server' );
    expect( configsContent ).toContain( 'toml-mcp-tool' );
    expect( configsContent ).toContain( 'json-mcp-tool' );

    // Should NOT contain example configss
    expect( configsContent ).not.toContain( 'example' );
    expect( configsContent ).not.toContain( 'filesystem_server' );
    expect( configsContent ).not.toContain( 'remote_api' );
  } );

  it( 'should create no OpenHands configs when no MCP servers are defined', async () =>
  {
    // Create .ruler directory with NO MCP servers
    const rulerDir = path.join( tmpDir, '.ruler' );
    await fs.mkdir( rulerDir );

    await fs.writeFile( path.join( rulerDir, 'AGENTS.md' ), '# Test instructions' );

    // ruler.toml with NO mcp_servers section
    const tomlContent = `
# No MCP servers defined at all
default_agents = ["openhands"]
`;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), tomlContent );

    // Apply ruler configsuration for OpenHands
    await applyAllAgentconfigss(
      tmpDir,
      [ 'openhands' ],
      undefined,
      true, // mcp enabled
      undefined,
      undefined,
      false, // verbose
      false, // dry run
      true   // local only
    );

    // Should NOT create OpenHands configs file when no servers are defined
    const openHandsconfigsPath = path.join( tmpDir, 'configs.toml' );
    const configsExists = await fs.access( openHandsconfigsPath ).then( () => true ).catch( () => false );
    expect( configsExists ).toBe( false );
  } );
} );