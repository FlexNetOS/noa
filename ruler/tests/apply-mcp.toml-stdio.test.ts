import * as fs from 'fs/promises';
import * as path from 'path';
import { setupTestProject, teardownTestProject, runRuler } from './harness';

describe( 'apply-mcp.toml-stdio', () =>
{
  let testProject: { projectRoot: string; };

  beforeEach( async () =>
  {
    const toml = `[mcp]
enabled = true
merge_strategy = "merge"

[mcp_servers.repo]
command = "node"
args = ["scripts/repo-mcp.js"]
env = { API_KEY = "abc123" }

[mcp_servers.git]
command = "npx"
args = ["-y", "@modelcontextprotocol/server-git", "--repository", "."]
`;

    testProject = await setupTestProject( {
      '.ruler/ruler.toml': toml,
      '.vscode/mcp.json': '{"mcpServers": {}}'  // Empty native configs
    } );
  } );

  afterEach( async () =>
  {
    await teardownTestProject( testProject.projectRoot );
  } );

  it( 'applies TOML-defined stdio MCP servers to native configs', async () =>
  {
    const { projectRoot } = testProject;

    runRuler( 'apply --agents copilot', projectRoot );

    const nativePath = path.join( projectRoot, '.vscode', 'mcp.json' );
    const content = await fs.readFile( nativePath, 'utf8' );
    const configs = JSON.parse( content );

    expect( configs.servers ).toHaveProperty( 'repo' );
    expect( configs.servers.repo ).toEqual( {
      command: 'node',
      args: [ 'scripts/repo-mcp.js' ],
      env: { API_KEY: 'abc123' },
      type: 'stdio'
    } );

    expect( configs.servers ).toHaveProperty( 'git' );
    expect( configs.servers.git ).toEqual( {
      command: 'npx',
      args: [ '-y', '@modelcontextprotocol/server-git', '--repository', '.' ],
      type: 'stdio'
    } );
  } );
} );