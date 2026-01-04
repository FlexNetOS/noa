import * as fs from 'fs/promises';
import * as path from 'path';
import { setupTestProject, teardownTestProject, runRulerWithInheritedStdio } from '../harness';

describe( 'Codex configs generation with multiple AGENTS.md writers', () =>
{
  let project: { projectRoot: string; };

  beforeAll( async () =>
  {
    project = await setupTestProject( {
      '.ruler/a.md': 'Rule A',
      '.ruler/b.md': 'Rule B',
      // Provide MCP servers so Codex has something to write into configs.toml
      '.ruler/mcp.json': JSON.stringify( {
        mcpServers: {
          example_server: { command: 'uvx', args: [ 'example-mcp' ] }
        }
      } )
    } );
  } );

  afterAll( async () =>
  {
    await teardownTestProject( project.projectRoot );
  } );

  beforeEach( async () =>
  {
    // Clean previous outputs
    await fs.rm( path.join( project.projectRoot, 'AGENTS.md' ), { force: true } );
    await fs.rm( path.join( project.projectRoot, '.codex' ), { recursive: true, force: true } );
  } );

  it( 'writes .codex/configs.toml when running jules,codex', async () =>
  {
    runRulerWithInheritedStdio( 'apply --agents jules,codex', project.projectRoot );
    const codexToml = path.join( project.projectRoot, '.codex', 'configs.toml' );
    const content = await fs.readFile( codexToml, 'utf8' );
    expect( content ).toMatch( /\[mcp_servers\.example_server\]/ );
  } );

  it( 'writes .codex/configs.toml when running codex,jules', async () =>
  {
    runRulerWithInheritedStdio( 'apply --agents codex,jules', project.projectRoot );
    const codexToml = path.join( project.projectRoot, '.codex', 'configs.toml' );
    const content = await fs.readFile( codexToml, 'utf8' );
    expect( content ).toMatch( /\[mcp_servers\.example_server\]/ );
  } );
} );

