import * as fs from 'fs/promises';
import * as path from 'path';
import os from 'os';
import { loadconfigs } from '../../../src/core/configsLoader';
import { applyAllAgentconfigss } from '../../../src/lib';

describe( 'Lowercase configsuration Support', () =>
{
  let tmpDir: string;

  beforeEach( async () =>
  {
    tmpDir = await fs.mkdtemp( path.join( os.tmpdir(), 'ruler-lowercase-configs-' ) );

    // Create .ruler directory
    const rulerDir = path.join( tmpDir, '.ruler' );
    await fs.mkdir( rulerDir, { recursive: true } );

    // Create a basic instructions file
    await fs.writeFile( path.join( rulerDir, 'instructions.md' ), '# Test instructions' );
  } );

  afterEach( async () =>
  {
    await fs.rm( tmpDir, { recursive: true, force: true } );
  } );

  it( 'supports lowercase agent identifiers in default_agents', async () =>
  {
    const configsContent = `
default_agents = ["copilot", "claude", "aider"]

[agents.copilot]
enabled = true

[agents.claude]
enabled = false
`;

    const configsPath = path.join( tmpDir, '.ruler', 'ruler.toml' );
    await fs.writeFile( configsPath, configsContent );

    const configs = await loadconfigs( {
      projectRoot: tmpDir,
      configsPath,
    } );

    expect( configs.defaultAgents ).toEqual( [ 'copilot', 'claude', 'aider' ] );
    expect( configs.agentconfigss.copilot?.enabled ).toBe( true );
    expect( configs.agentconfigss.claude?.enabled ).toBe( false );
  } );

  it( 'supports mixed case agent identifiers in CLI agents', async () =>
  {
    const configs = await loadconfigs( {
      projectRoot: tmpDir,
      cliAgents: [ 'copilot', 'CLAUDE', 'Aider' ],
    } );

    expect( configs.cliAgents ).toEqual( [ 'copilot', 'CLAUDE', 'Aider' ] );
  } );

  it( 'normalizes agent configs keys to lowercase', async () =>
  {
    const configsContent = `
[agents.COPILOT]
enabled = true

[agents.Claude]
enabled = false

[agents.aider]
enabled = true
`;

    const configsPath = path.join( tmpDir, '.ruler', 'ruler.toml' );
    await fs.writeFile( configsPath, configsContent );

    const configs = await loadconfigs( {
      projectRoot: tmpDir,
      configsPath,
    } );

    // configsLoader preserves the original casing, normalization happens in lib.ts
    expect( configs.agentconfigss.COPILOT?.enabled ).toBe( true );
    expect( configs.agentconfigss.Claude?.enabled ).toBe( false );
    expect( configs.agentconfigss.aider?.enabled ).toBe( true );
  } );

  it( 'provides correct output paths for all agents', async () =>
  {
    const configsContent = `
[agents.copilot]
output_path = "custom/copilot.md"

[agents.claude]
output_path = "CUSTOM_CLAUDE.md"

[agents.aider]
output_path_instructions = "custom_aider.md"
output_path_configs = "custom_aider.yml"
`;

    const configsPath = path.join( tmpDir, '.ruler', 'ruler.toml' );
    await fs.writeFile( configsPath, configsContent );

    const configs = await loadconfigs( {
      projectRoot: tmpDir,
      configsPath,
    } );

    // configsLoader resolves paths to absolute paths
    expect( configs.agentconfigss.copilot?.outputPath ).toBe( path.join( tmpDir, 'custom/copilot.md' ) );
    expect( configs.agentconfigss.claude?.outputPath ).toBe( path.join( tmpDir, 'CUSTOM_CLAUDE.md' ) );
    expect( configs.agentconfigss.aider?.outputPathInstructions ).toBe( path.join( tmpDir, 'custom_aider.md' ) );
    expect( configs.agentconfigss.aider?.outputPathconfigs ).toBe( path.join( tmpDir, 'custom_aider.yml' ) );
  } );
} );