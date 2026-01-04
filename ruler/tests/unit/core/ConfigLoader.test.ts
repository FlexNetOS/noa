import * as fs from 'fs/promises';
import * as path from 'path';
import os from 'os';

import { loadconfigs, Loadedconfigs } from '../../../src/core/configsLoader';

describe( 'configsLoader', () =>
{
  let tmpDir: string;
  let rulerDir: string;

  beforeEach( async () =>
  {
    tmpDir = await fs.mkdtemp( path.join( os.tmpdir(), 'ruler-configs-' ) );
    rulerDir = path.join( tmpDir, '.ruler' );
    await fs.mkdir( rulerDir, { recursive: true } );
  } );

  afterEach( async () =>
  {
    await fs.rm( tmpDir, { recursive: true, force: true } );
  } );

  it( 'returns empty configs when file does not exist', async () =>
  {
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.defaultAgents ).toBeUndefined();
    expect( configs.agentconfigss ).toEqual( {} );
    expect( configs.cliAgents ).toBeUndefined();
  } );

  it( 'returns empty configs when file is empty', async () =>
  {
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), '' );
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.defaultAgents ).toBeUndefined();
    expect( configs.agentconfigss ).toEqual( {} );
  } );

  it( 'parses default_agents', async () =>
  {
    const content = `default_agents = ["A", "B"]`;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.defaultAgents ).toEqual( [ 'A', 'B' ] );
  } );

  it( 'parses nested configsuration option', async () =>
  {
    const content = `nested = true`;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.nested ).toBe( true );
  } );

  it( 'defaults nested to undefined when not specified', async () =>
  {
    const content = `default_agents = ["A"]`;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.nested ).toBe( false );
  } );

  it( 'parses agent enabled overrides', async () =>
  {
    const content = `
      [agents.A]
      enabled = false
      [agents.B]
      enabled = true
    `;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.agentconfigss.A.enabled ).toBe( false );
    expect( configs.agentconfigss.B.enabled ).toBe( true );
  } );

  it( 'parses agent output_path and resolves to projectRoot', async () =>
  {
    const content = `
      [agents.A]
      output_path = "foo/bar.md"
    `;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.agentconfigss.A.outputPath ).toBe(
      path.resolve( tmpDir, 'foo/bar.md' ),
    );
  } );

  it( 'parses agent output_path_instructions and resolves to projectRoot', async () =>
  {
    const content = `
    [agents.A]
    output_path_instructions = "foo/instructions.md"
  `;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.agentconfigss.A.outputPathInstructions ).toBe(
      path.resolve( tmpDir, 'foo/instructions.md' ),
    );
  } );

  it( 'parses agent output_path_configs and resolves to projectRoot', async () =>
  {
    const content = `
    [agents.A]
    output_path_configs = "foo/configs.toml"
  `;
    await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
    const configs = await loadconfigs( { projectRoot: tmpDir } );
    expect( configs.agentconfigss.A.outputPathconfigs ).toBe(
      path.resolve( tmpDir, 'foo/configs.toml' ),
    );
  } );

  it( 'loads configs from custom path via configsPath option', async () =>
  {
    const altDir = path.join( tmpDir, 'alt' );
    await fs.mkdir( altDir, { recursive: true } );
    const altPath = path.join( altDir, 'myconfigs.toml' );
    await fs.writeFile( altPath, `default_agents = ["X"]` );
    const configs = await loadconfigs( {
      projectRoot: tmpDir,
      configsPath: altPath,
    } );
    expect( configs.defaultAgents ).toEqual( [ 'X' ] );
  } );

  it( 'captures CLI agents override', async () =>
  {
    const overrides = [ 'C', 'D' ];
    const configs = await loadconfigs( {
      projectRoot: tmpDir,
      cliAgents: overrides,
    } );
    expect( configs.cliAgents ).toEqual( overrides );
  } );

  describe( 'gitignore configsuration', () =>
  {
    it( 'parses [gitignore] section with enabled = true', async () =>
    {
      const content = `
        [gitignore]
        enabled = true
      `;
      await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
      const configs = await loadconfigs( { projectRoot: tmpDir } );
      expect( configs.gitignore ).toBeDefined();
      expect( configs.gitignore?.enabled ).toBe( true );
    } );

    it( 'parses [gitignore] section with enabled = false', async () =>
    {
      const content = `
        [gitignore]
        enabled = false
      `;
      await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
      const configs = await loadconfigs( { projectRoot: tmpDir } );
      expect( configs.gitignore ).toBeDefined();
      expect( configs.gitignore?.enabled ).toBe( false );
    } );

    it( 'parses [gitignore] section with missing enabled key', async () =>
    {
      const content = `
        [gitignore]
        # enabled key not specified
      `;
      await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
      const configs = await loadconfigs( { projectRoot: tmpDir } );
      expect( configs.gitignore ).toBeDefined();
      expect( configs.gitignore?.enabled ).toBeUndefined();
    } );

    it( 'handles missing [gitignore] section', async () =>
    {
      const content = `
        default_agents = ["A"]
      `;
      await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), content );
      const configs = await loadconfigs( { projectRoot: tmpDir } );
      expect( configs.gitignore ).toBeDefined();
      expect( configs.gitignore?.enabled ).toBeUndefined();
    } );

    it( 'handles empty configs file for gitignore', async () =>
    {
      await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), '' );
      const configs = await loadconfigs( { projectRoot: tmpDir } );
      expect( configs.gitignore ).toBeDefined();
      expect( configs.gitignore?.enabled ).toBeUndefined();
    } );
  } );
} );
