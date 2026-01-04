import { promises as fs } from 'fs';
import * as path from 'path';
import * as os from 'os';
import { CodexCliAgent } from '../../../src/agents/CodexCliAgent';

describe( 'CodexCliAgent - MCP configs Path Tracking', () =>
{
  let tmpDir: string;

  beforeEach( async () =>
  {
    tmpDir = await fs.mkdtemp( path.join( os.tmpdir(), 'ruler-codex-test-' ) );
  } );

  afterEach( async () =>
  {
    await fs.rm( tmpDir, { recursive: true, force: true } );
  } );

  it( 'should return both instructions and configs paths from getDefaultOutputPath', () =>
  {
    const agent = new CodexCliAgent();
    const result = agent.getDefaultOutputPath( tmpDir );

    expect( result ).toEqual( {
      instructions: path.join( tmpDir, 'AGENTS.md' ),
      configs: path.join( tmpDir, '.codex', 'configs.toml' ),
    } );
  } );

  it( 'should create both AGENTS.md and .codex/configs.toml when MCP is enabled', async () =>
  {
    const agent = new CodexCliAgent();
    const rulerMcpJson = {
      mcpServers: {
        filesystem: {
          command: 'npx',
          args: [ '-y', '@modelcontextprotocol/server-filesystem', '/path/to/files' ]
        }
      }
    };

    await agent.applyRulerconfigs(
      '# Test Rules\nThis is a test configsuration.',
      tmpDir,
      rulerMcpJson
    );

    // Check that both files were created
    const agentsPath = path.join( tmpDir, 'AGENTS.md' );
    const configsPath = path.join( tmpDir, '.codex', 'configs.toml' );

    expect( await fs.access( agentsPath ).then( () => true ).catch( () => false ) ).toBe( true );
    expect( await fs.access( configsPath ).then( () => true ).catch( () => false ) ).toBe( true );

    // Verify content
    const agentsContent = await fs.readFile( agentsPath, 'utf8' );
    expect( agentsContent ).toContain( 'Test Rules' );

    const configsContent = await fs.readFile( configsPath, 'utf8' );
    expect( configsContent ).toContain( '[mcp_servers.filesystem]' );
    expect( configsContent ).toContain( 'command = "npx"' );
  } );

  it( 'should respect outputPathconfigs override', async () =>
  {
    const agent = new CodexCliAgent();
    const customconfigsPath = path.join( tmpDir, 'custom', 'codex.toml' );
    const rulerMcpJson = {
      mcpServers: {
        filesystem: {
          command: 'npx',
          args: [ '-y', '@modelcontextprotocol/server-filesystem', '/path/to/files' ]
        }
      }
    };

    await agent.applyRulerconfigs(
      '# Test Rules',
      tmpDir,
      rulerMcpJson,
      { outputPathconfigs: customconfigsPath }
    );

    // Should create configs at custom path
    expect( await fs.access( customconfigsPath ).then( () => true ).catch( () => false ) ).toBe( true );

    // Should still create AGENTS.md at default location
    const agentsPath = path.join( tmpDir, 'AGENTS.md' );
    expect( await fs.access( agentsPath ).then( () => true ).catch( () => false ) ).toBe( true );
  } );
} );