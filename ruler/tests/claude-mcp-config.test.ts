import * as fs from 'fs/promises';
import * as path from 'path';
import { setupTestProject, teardownTestProject, runRulerWithInheritedStdio } from './harness';

describe( 'claude-mcp-configs', () =>
{
  let testProject: { projectRoot: string; };

  beforeEach( async () =>
  {
    // Create ruler MCP configs
    const rulerMcp = { mcpServers: { ruler_server: { url: 'http://ruler.com' } } };

    // Create Claude MCP configs - initially empty or non-existent
    const claudeNative = { mcpServers: { native_claude_server: { url: 'http://claude.com' } } };

    testProject = await setupTestProject( {
      '.ruler/mcp.json': JSON.stringify( rulerMcp, null, 2 ) + '\n',
      '.mcp.json': JSON.stringify( claudeNative, null, 2 ) + '\n'
    } );
  } );

  afterEach( async () =>
  {
    await teardownTestProject( testProject.projectRoot );
  } );

  it( 'should use "mcpServers" key for Claude Code', async () =>
  {
    const { projectRoot } = testProject;

    runRulerWithInheritedStdio( 'apply --agents claude', projectRoot );

    // Verify Claude MCP configs uses 'mcpServers' key
    const claudeResultText = await fs.readFile(
      path.join( projectRoot, '.mcp.json' ),
      'utf8',
    );
    const claudeResult = JSON.parse( claudeResultText );

    // Should have 'mcpServers' key, not empty string or other
    expect( claudeResult.mcpServers ).toBeDefined();
    expect( claudeResult[ "" ] ).toBeUndefined(); // Should not have empty string key

    // Should contain both native and ruler servers
    expect( Object.keys( claudeResult.mcpServers ).sort() ).toEqual( [ 'native_claude_server', 'ruler_server' ] );
  } );

  it( 'should use "mcpServers" key for Claude Code with overwrite strategy', async () =>
  {
    const { projectRoot } = testProject;

    runRulerWithInheritedStdio( 'apply --agents claude --mcp-overwrite', projectRoot );

    // Verify Claude MCP configs was overwritten and uses 'mcpServers' key
    const claudeResultText = await fs.readFile(
      path.join( projectRoot, '.mcp.json' ),
      'utf8',
    );
    const claudeResult = JSON.parse( claudeResultText );

    // Should have 'mcpServers' key, not empty string or other
    expect( claudeResult.mcpServers ).toBeDefined();
    expect( claudeResult[ "" ] ).toBeUndefined(); // Should not have empty string key

    // Should contain only ruler server (overwrite should remove native)
    expect( Object.keys( claudeResult.mcpServers ) ).toEqual( [ 'ruler_server' ] );
  } );
} );