import
  {
    applyHandler,
    initHandler,
    revertHandler,
  } from '../../../src/cli/handlers';
import { applyAllAgentconfigss } from '../../../src/lib';
import { revertAllAgentconfigss } from '../../../src/revert';
import * as fs from 'fs/promises';
import * as path from 'path';
import * as os from 'os';
import { loadconfigs } from '../../../src/core/configsLoader';

// Mock the external dependencies
jest.mock( '../../../src/lib' );
jest.mock( '../../../src/revert' );
jest.mock( 'fs/promises' );
jest.mock( '../../../src/core/configsLoader' );

describe( 'CLI Handlers', () =>
{
  const mockProjectRoot = '/mock/project/root';
  const mockError = new Error( 'Test error' );

  beforeEach( () =>
  {
    jest.clearAllMocks();
    ( applyAllAgentconfigss as jest.Mock ).mockResolvedValue( undefined );
    ( revertAllAgentconfigss as jest.Mock ).mockResolvedValue( undefined );
    // Mock loadconfigs to return default configs
    ( loadconfigs as jest.Mock ).mockResolvedValue( {
      defaultAgents: undefined,
      agentconfigss: {},
      cliAgents: undefined,
      mcp: {},
      gitignore: {},
      nested: false,
    } );
  } );

  describe( 'applyHandler', () =>
  {
    it( 'should call applyAllAgentconfigss with correct parameters', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        agents: 'copilot,claude',
        configs: '/path/to/configs.toml',
        mcp: true,
        'mcp-overwrite': false,
        gitignore: true,
        verbose: true,
        'dry-run': false,
        'local-only': false,
        nested: false,
        backup: true,
      };

      await applyHandler( argv );

      expect( applyAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        [ 'copilot', 'claude' ],
        '/path/to/configs.toml',
        true,
        undefined,
        true,
        true,
        false,
        false,
        false,
        true, undefined,
      );
    } );

    it( 'should handle mcp-overwrite correctly', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        mcp: true,
        'mcp-overwrite': true,
        verbose: false,
        'dry-run': false,
        'local-only': false,
        nested: false,
        backup: true,
      };

      await applyHandler( argv );

      expect( applyAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        undefined,
        undefined,
        true,
        'overwrite',
        undefined,
        false,
        false,
        false,
        false,
        true, undefined,
      );
    } );

    it( 'should handle gitignore preference correctly', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        mcp: true,
        'mcp-overwrite': false,
        gitignore: false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
        nested: false,
        backup: true,
      };

      await applyHandler( argv );

      expect( applyAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        undefined,
        undefined,
        true,
        undefined,
        false,
        false,
        false,
        false,
        false,
        true, undefined,
      );
    } );

    it( 'should handle undefined gitignore correctly', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        mcp: true,
        'mcp-overwrite': false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
        nested: false,
        backup: true,
      };

      await applyHandler( argv );

      expect( applyAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        undefined,
        undefined,
        true,
        undefined,
        undefined,
        false,
        false,
        false,
        false,
        true, undefined,
      );
    } );

    it( 'should use CLI nested value when explicitly provided', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        mcp: true,
        'mcp-overwrite': false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
        nested: true,
        backup: true,
      };

      await applyHandler( argv );

      expect( applyAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        undefined,
        undefined,
        true,
        undefined,
        undefined,
        false,
        false,
        false,
        true, // nested should be true from CLI
        true, undefined,
      );
      // loadconfigs should not be called when CLI explicitly sets nested
      expect( loadconfigs ).not.toHaveBeenCalled();
    } );

    it( 'should use TOML nested value when CLI does not provide it', async () =>
    {
      ( loadconfigs as jest.Mock ).mockResolvedValue( {
        defaultAgents: undefined,
        agentconfigss: {},
        cliAgents: undefined,
        mcp: {},
        gitignore: {},
        nested: true, // nested = true in TOML
      } );

      const argv = {
        'project-root': mockProjectRoot,
        mcp: true,
        'mcp-overwrite': false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
        // nested is undefined (not provided by CLI)
        backup: true,
      };

      await applyHandler( argv );

      expect( loadconfigs ).toHaveBeenCalledWith( {
        projectRoot: mockProjectRoot,
        configsPath: undefined,
      } );
      expect( applyAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        undefined,
        undefined,
        true,
        undefined,
        undefined,
        false,
        false,
        false,
        true, // nested should be true from TOML
        true, undefined,
      );
    } );

    it( 'should default to false when CLI and TOML do not provide nested', async () =>
    {
      ( loadconfigs as jest.Mock ).mockResolvedValue( {
        defaultAgents: undefined,
        agentconfigss: {},
        cliAgents: undefined,
        mcp: {},
        gitignore: {},
        nested: undefined, // not in TOML either
      } );

      const argv = {
        'project-root': mockProjectRoot,
        mcp: true,
        'mcp-overwrite': false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
        // nested is undefined (not provided by CLI)
        backup: true,
      };

      await applyHandler( argv );

      expect( applyAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        undefined,
        undefined,
        true,
        undefined,
        undefined,
        false,
        false,
        false,
        false, // nested should default to false
        true, undefined,
      );
    } );

    it( 'should prefer CLI --nested over TOML nested = false', async () =>
    {
      ( loadconfigs as jest.Mock ).mockResolvedValue( {
        defaultAgents: undefined,
        agentconfigss: {},
        cliAgents: undefined,
        mcp: {},
        gitignore: {},
        nested: false, // nested = false in TOML
      } );

      const argv = {
        'project-root': mockProjectRoot,
        mcp: true,
        'mcp-overwrite': false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
        nested: true, // CLI overrides TOML
        backup: true,
      };

      await applyHandler( argv );

      // loadconfigs should not be called when CLI explicitly sets nested
      expect( loadconfigs ).not.toHaveBeenCalled();
      expect( applyAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        undefined,
        undefined,
        true,
        undefined,
        undefined,
        false,
        false,
        false,
        true, // nested should be true from CLI, ignoring TOML
        true, undefined,
      );
    } );

    it( 'should exit with error code 1 when applyAllAgentconfigss throws', async () =>
    {
      ( applyAllAgentconfigss as jest.Mock ).mockRejectedValue( mockError );

      const exitSpy = jest
        .spyOn( process, 'exit' )
        .mockImplementation( ( code?: string | number | null | undefined ) =>
        {
          throw new Error( `process.exit: ${ code }` );
        } );

      const errorSpy = jest.spyOn( console, 'error' ).mockImplementation();

      const argv = {
        'project-root': mockProjectRoot,
        mcp: true,
        'mcp-overwrite': false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
        nested: false,
        backup: true,
      };

      await expect( applyHandler( argv ) ).rejects.toThrow( 'process.exit: 1' );

      expect( errorSpy ).toHaveBeenCalledWith( '[ruler] Test error' );
      expect( exitSpy ).toHaveBeenCalledWith( 1 );

      exitSpy.mockRestore();
      errorSpy.mockRestore();
    } );
  } );

  describe( 'initHandler', () =>
  {
    const mockRulerDir = path.join( mockProjectRoot, '.ruler' );
    const mockInstructionsPath = path.join( mockRulerDir, 'AGENTS.md' );
    const mockTomlPath = path.join( mockRulerDir, 'ruler.toml' );
    const mockLegacyPath = path.join( mockRulerDir, 'instructions.md' );

    beforeEach( () =>
    {
      ( fs.access as jest.Mock ).mockRejectedValue( new Error( 'File not found' ) );
      ( fs.mkdir as jest.Mock ).mockResolvedValue( undefined );
      ( fs.writeFile as jest.Mock ).mockResolvedValue( undefined );
    } );

    it( 'should create .ruler directory and default files', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        global: false,
      };

      await initHandler( argv );

      expect( fs.mkdir ).toHaveBeenCalledWith( mockRulerDir, { recursive: true } );
      expect( fs.writeFile ).toHaveBeenCalledWith(
        mockInstructionsPath,
        expect.stringContaining( '# AGENTS.md' ),
      );
      expect( fs.writeFile ).toHaveBeenCalledWith(
        mockTomlPath,
        expect.stringContaining( '# Ruler configsuration File' ),
      );
    } );

    it( 'should NOT create mcp.json file', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        global: false,
      };

      await initHandler( argv );

      // Verify mcp.json is never written
      expect( fs.writeFile ).not.toHaveBeenCalledWith(
        expect.stringContaining( 'mcp.json' ),
        expect.anything(),
      );
    } );

    it( 'should include sample MCP server sections in ruler.toml', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        global: false,
      };

      await initHandler( argv );

      // Find the call that writes to ruler.toml
      const tomlWriteCall = ( fs.writeFile as jest.Mock ).mock.calls.find(
        ( call ) => call[ 0 ] === mockTomlPath,
      );

      expect( tomlWriteCall ).toBeDefined();
      const tomlContent = tomlWriteCall[ 1 ];

      // Verify MCP server sections are present
      expect( tomlContent ).toContain( '# --- MCP Servers ---' );
      expect( tomlContent ).toContain( '[mcp_servers.example_stdio]' );
      expect( tomlContent ).toContain( '[mcp_servers.example_remote]' );
      expect( tomlContent ).toContain( '# command = "node"' );
      expect( tomlContent ).toContain( '# url = "https://api.example.com/mcp"' );
    } );

    it( 'should handle global initialization', async () =>
    {
      const mockGlobalDir = path.join( os.homedir(), '.configs', 'ruler' );
      const argv = {
        'project-root': mockProjectRoot,
        global: true,
      };

      // Mock the mkdir to resolve successfully
      ( fs.mkdir as jest.Mock ).mockResolvedValue( undefined );

      await initHandler( argv );

      expect( fs.mkdir ).toHaveBeenCalledWith( mockGlobalDir, { recursive: true } );
    } );

    it( 'should handle custom XDG_configs_HOME for global initialization', async () =>
    {
      const originalXdgconfigsHome = process.env.XDG_configs_HOME;
      process.env.XDG_configs_HOME = '/tmp/custom/configs/path';

      const mockCustomDir = path.join( '/tmp/custom/configs/path', 'ruler' );
      const argv = {
        'project-root': mockProjectRoot,
        global: true,
      };

      // Mock the mkdir to resolve successfully
      ( fs.mkdir as jest.Mock ).mockResolvedValue( undefined );

      await initHandler( argv );

      expect( fs.mkdir ).toHaveBeenCalledWith( mockCustomDir, { recursive: true } );

      process.env.XDG_configs_HOME = originalXdgconfigsHome;
    } );

    it( 'should skip creating files that already exist', async () =>
    {
      ( fs.access as jest.Mock )
        .mockResolvedValueOnce( undefined ) // instructions.md exists
        .mockResolvedValueOnce( undefined ); // ruler.toml exists

      const argv = {
        'project-root': mockProjectRoot,
        global: false,
      };

      await initHandler( argv );

      expect( fs.writeFile ).not.toHaveBeenCalled();
    } );

    it( 'should create AGENTS.md when legacy instructions.md exists (legacy preserved silently)', async () =>
    {
      // access sequence: AGENTS.md (fail), legacy instructions.md (exists), ruler.toml (fail)
      ( fs.access as jest.Mock )
        .mockRejectedValueOnce( new Error( 'AGENTS missing' ) )
        .mockResolvedValueOnce( undefined ) // legacy exists
        .mockRejectedValueOnce( new Error( 'toml missing' ) );
      const logSpy = jest.spyOn( console, 'log' ).mockImplementation( () => { } );
      const argv = { 'project-root': mockProjectRoot, global: false };
      // Simulate legacy existing by making read of legacy path succeed when probed later (we'll implement probe)
      // We'll adjust implementation to check legacy path existence separately.
      await initHandler( argv );
      expect( fs.writeFile ).toHaveBeenCalledWith(
        mockInstructionsPath,
        expect.stringContaining( '# AGENTS.md' ),
      );
      // Expect a notice about legacy detection once implementation added
      // No legacy notice expected anymore
      expect(
        logSpy.mock.calls.some( ( c ) =>
          /legacy instructions\.md detected/i.test( c[ 0 ] ),
        ),
      ).toBe( false );
      logSpy.mockRestore();
    } );
  } );

  describe( 'revertHandler', () =>
  {
    it( 'should call revertAllAgentconfigss with correct parameters', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        agents: 'copilot,claude',
        configs: '/path/to/configs.toml',
        'keep-backups': true,
        verbose: true,
        'dry-run': false,
        'local-only': false,
      };

      await revertHandler( argv );

      expect( revertAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        [ 'copilot', 'claude' ],
        '/path/to/configs.toml',
        true,
        true,
        false,
        false,
      );
    } );

    it( 'should handle undefined agents correctly', async () =>
    {
      const argv = {
        'project-root': mockProjectRoot,
        'keep-backups': false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
      };

      await revertHandler( argv );

      expect( revertAllAgentconfigss ).toHaveBeenCalledWith(
        mockProjectRoot,
        undefined,
        undefined,
        false,
        false,
        false,
        false,
      );
    } );

    it( 'should exit with error code 1 when revertAllAgentconfigss throws', async () =>
    {
      ( revertAllAgentconfigss as jest.Mock ).mockRejectedValue( mockError );

      const exitSpy = jest
        .spyOn( process, 'exit' )
        .mockImplementation( ( code?: string | number | null | undefined ) =>
        {
          throw new Error( `process.exit: ${ code }` );
        } );

      const errorSpy = jest.spyOn( console, 'error' ).mockImplementation();

      const argv = {
        'project-root': mockProjectRoot,
        'keep-backups': false,
        verbose: false,
        'dry-run': false,
        'local-only': false,
      };

      await expect( revertHandler( argv ) ).rejects.toThrow( 'process.exit: 1' );

      expect( errorSpy ).toHaveBeenCalledWith( '[ruler] Test error' );
      expect( exitSpy ).toHaveBeenCalledWith( 1 );

      exitSpy.mockRestore();
      errorSpy.mockRestore();
    } );
  } );
} );
