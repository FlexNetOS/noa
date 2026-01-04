import * as fs from 'fs/promises';
import * as path from 'path';
import os from 'os';

import
  {
    loadSingleconfigsuration,
    loadNestedconfigsurations,
    applyconfigsurationsToAgents,
    updateGitignore,
    processHierarchicalconfigsurations,
    Rulerconfigsuration,
    HierarchicalRulerconfigsuration,
  } from '../../../src/core/apply-engine';
import { IAgent } from '../../../src/agents/IAgent';
import { ClaudeAgent } from '../../../src/agents/ClaudeAgent';
import { CopilotAgent } from '../../../src/agents/CopilotAgent';
import { Loadedconfigs } from '../../../src/core/configsLoader';
import * as FileSystemUtils from '../../../src/core/FileSystemUtils';
import * as Constants from '../../../src/constants';

// Mock agents for testing
class MockAgent implements IAgent
{
  constructor (
    private name: string,
    private identifier: string,
  ) { }

  getName (): string
  {
    return this.name;
  }

  getIdentifier (): string
  {
    return this.identifier;
  }

  async applyRulerconfigs (
    rules: string,
    projectRoot: string,
    mcpJson: Record<string, unknown> | null,
    agentconfigs?: any,
  ): Promise<void>
  {
    // Mock implementation
  }

  getDefaultOutputPath ( projectRoot: string ): string
  {
    return `${ projectRoot }/.${ this.identifier }/configs.json`;
  }

  getMcpServerKey?(): string
  {
    return 'mcpServers';
  }

  supportsMcpStdio?(): boolean
  {
    return true;
  }

  supportsMcpRemote?(): boolean
  {
    return true;
  }
}

describe( 'apply-engine', () =>
{
  let tmpDir: string;
  let rulerDir: string;

  beforeEach( async () =>
  {
    tmpDir = await fs.mkdtemp( path.join( os.tmpdir(), 'ruler-apply-engine-' ) );
    rulerDir = path.join( tmpDir, '.ruler' );
    await fs.mkdir( rulerDir, { recursive: true } );
  } );

  afterEach( async () =>
  {
    await fs.rm( tmpDir, { recursive: true, force: true } );
  } );

  describe( 'loadRulerconfigsuration', () =>
  {
    it( 'should load configsuration with rules and MCP', async () =>
    {
      // Setup test files
      const configsContent = `default_agents = ["claude", "copilot"]`;
      await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), configsContent );

      const rulesContent = '# Test rules\nUse TypeScript for all code.';
      await fs.writeFile( path.join( rulerDir, 'instructions.md' ), rulesContent );

      const mcpContent = JSON.stringify( {
        mcpServers: {
          test: {
            command: 'test-command',
            args: [ '--test' ],
          },
        },
      } );
      await fs.writeFile( path.join( rulerDir, 'mcp.json' ), mcpContent );

      const result = await loadSingleconfigsuration( tmpDir, undefined, false );

      // Since hierarchical=false, result should be Rulerconfigsuration
      expect( result ).toHaveProperty( 'configs' );
      expect( result ).toHaveProperty( 'concatenatedRules' );
      expect( result ).toHaveProperty( 'rulerMcpJson' );

      const configsResult = result as Rulerconfigsuration;
      expect( configsResult.configs.defaultAgents ).toEqual( [ 'claude', 'copilot' ] );
      expect( configsResult.concatenatedRules ).toContain(
        'Use TypeScript for all code.',
      );
      expect( configsResult.rulerMcpJson ).toEqual( {
        mcpServers: {
          test: {
            command: 'test-command',
            args: [ '--test' ],
            type: 'stdio',
          },
        },
      } );
    } );

    it( 'should handle missing MCP file gracefully', async () =>
    {
      const configsContent = `default_agents = ["claude"]`;
      await fs.writeFile( path.join( rulerDir, 'ruler.toml' ), configsContent );

      const rulesContent = '# Test rules';
      await fs.writeFile( path.join( rulerDir, 'instructions.md' ), rulesContent );

      const result = await loadSingleconfigsuration( tmpDir, undefined, false );

      // Since hierarchical=false, result should be Rulerconfigsuration
      expect( result ).toHaveProperty( 'configs' );
      expect( result ).toHaveProperty( 'concatenatedRules' );
      expect( result ).toHaveProperty( 'rulerMcpJson' );

      const configsResult = result as Rulerconfigsuration;
      expect( configsResult.configs.defaultAgents ).toEqual( [ 'claude' ] );
      expect( configsResult.concatenatedRules ).toContain( '# Test rules' );
      expect( configsResult.rulerMcpJson ).toBeNull();
    } );

    it( 'should throw error when .ruler directory not found', async () =>
    {
      const nonExistentDir = path.join( tmpDir, 'nonexistent' );

      jest.spyOn( FileSystemUtils, 'findRulerDir' ).mockResolvedValue( null );

      try
      {
        await expect(
          loadSingleconfigsuration( nonExistentDir, undefined, true ),
        ).rejects.toThrow( '.ruler directory not found' );
      } finally
      {
        ( FileSystemUtils.findRulerDir as jest.Mock ).mockRestore();
      }
    } );
  } );

  describe( 'loadNestedconfigsurations', () =>
  {
    it( 'loads independent configss and forces nested flag through descendants', async () =>
    {
      const moduleDir = path.join( tmpDir, 'module' );
      const submoduleDir = path.join( moduleDir, 'submodule' );

      const rootRulerDir = path.join( tmpDir, '.ruler' );
      const moduleRulerDir = path.join( moduleDir, '.ruler' );
      const submoduleRulerDir = path.join( submoduleDir, '.ruler' );

      await fs.mkdir( rootRulerDir, { recursive: true } );
      await fs.mkdir( moduleRulerDir, { recursive: true } );
      await fs.mkdir( submoduleRulerDir, { recursive: true } );

      await fs.writeFile(
        path.join( rootRulerDir, 'AGENTS.md' ),
        '# Root Instructions',
      );
      await fs.writeFile(
        path.join( moduleRulerDir, 'AGENTS.md' ),
        '# Module Instructions',
      );
      await fs.writeFile(
        path.join( submoduleRulerDir, 'AGENTS.md' ),
        '# Submodule Instructions',
      );

      await fs.writeFile(
        path.join( rootRulerDir, 'ruler.toml' ),
        `default_agents = ["root-agent"]
nested = true

[agents]
[agents.claude]
enabled = true

[mcp]
enabled = true
`,
      );

      await fs.writeFile(
        path.join( moduleRulerDir, 'ruler.toml' ),
        `default_agents = ["module-agent"]

[agents]
[agents.copilot]
enabled = false

[mcp]
enabled = false
`,
      );

      await fs.writeFile(
        path.join( submoduleRulerDir, 'ruler.toml' ),
        `default_agents = ["submodule-agent"]
nested = false

[agents]
[agents.windsurf]
enabled = true

[mcp]
merge_strategy = "overwrite"
`,
      );

      const warnSpy = jest
        .spyOn( Constants, 'logWarn' )
        .mockImplementation( () => { } );

      try
      {
        const configss = await loadNestedconfigsurations(
          tmpDir,
          undefined,
          true, // localOnly: true to avoid picking up global configs
          true, // resolvedNested: true to force nested mode
        );

        expect( configss ).toHaveLength( 3 );

        const rootconfigs = configss.find( ( c ) => c.rulerDir === rootRulerDir );
        const moduleconfigs = configss.find( ( c ) => c.rulerDir === moduleRulerDir );
        const submoduleconfigs = configss.find(
          ( c ) => c.rulerDir === submoduleRulerDir,
        );

        expect( rootconfigs ).toBeDefined();
        expect( moduleconfigs ).toBeDefined();
        expect( submoduleconfigs ).toBeDefined();

        if ( !rootconfigs || !moduleconfigs || !submoduleconfigs )
        {
          throw new Error( 'Expected hierarchical configss for all directories' );
        }

        expect( rootconfigs.configs ).not.toBe( moduleconfigs.configs );
        expect( rootconfigs.configs ).not.toBe( submoduleconfigs.configs );
        expect( moduleconfigs.configs ).not.toBe( submoduleconfigs.configs );

        expect( rootconfigs.configs.defaultAgents ).toEqual( [ 'root-agent' ] );
        expect( moduleconfigs.configs.defaultAgents ).toEqual( [ 'module-agent' ] );
        expect( submoduleconfigs.configs.defaultAgents ).toEqual( [
          'submodule-agent',
        ] );

        expect( Object.keys( rootconfigs.configs.agentconfigss ) ).toEqual( [ 'claude' ] );
        expect( rootconfigs.configs.agentconfigss.claude?.enabled ).toBe( true );

        expect( Object.keys( moduleconfigs.configs.agentconfigss ) ).toEqual( [
          'copilot',
        ] );
        expect( moduleconfigs.configs.agentconfigss.copilot?.enabled ).toBe( false );

        expect( Object.keys( submoduleconfigs.configs.agentconfigss ) ).toEqual( [
          'windsurf',
        ] );
        expect( submoduleconfigs.configs.agentconfigss.windsurf?.enabled ).toBe(
          true,
        );

        expect( rootconfigs.configs.mcp?.enabled ).toBe( true );
        expect( moduleconfigs.configs.mcp?.enabled ).toBe( false );
        expect( submoduleconfigs.configs.mcp?.strategy ).toBe( 'overwrite' );

        expect( rootconfigs.configs.nested ).toBe( true );
        expect( moduleconfigs.configs.nested ).toBe( true );
        expect( submoduleconfigs.configs.nested ).toBe( true );

        expect( warnSpy ).toHaveBeenCalledWith(
          expect.stringContaining( 'nested = false' ),
        );
        expect( warnSpy ).toHaveBeenCalledWith(
          expect.stringContaining( path.join( submoduleRulerDir, 'ruler.toml' ) ),
        );
      } finally
      {
        warnSpy.mockRestore();
      }
    } );

    it( 'propagates unified MCP bundles and preserves agent-level MCP flags per directory', async () =>
    {
      const moduleDir = path.join( tmpDir, 'module' );
      const submoduleDir = path.join( moduleDir, 'submodule' );

      const rootRulerDir = path.join( tmpDir, '.ruler' );
      const moduleRulerDir = path.join( moduleDir, '.ruler' );
      const submoduleRulerDir = path.join( submoduleDir, '.ruler' );

      await fs.mkdir( rootRulerDir, { recursive: true } );
      await fs.mkdir( moduleRulerDir, { recursive: true } );
      await fs.mkdir( submoduleRulerDir, { recursive: true } );

      await fs.writeFile(
        path.join( rootRulerDir, 'AGENTS.md' ),
        '# Root Instructions',
      );
      await fs.writeFile(
        path.join( moduleRulerDir, 'AGENTS.md' ),
        '# Module Instructions',
      );
      await fs.writeFile(
        path.join( submoduleRulerDir, 'AGENTS.md' ),
        '# Submodule Instructions',
      );

      await fs.writeFile(
        path.join( rootRulerDir, 'ruler.toml' ),
        `default_agents = ["claude", "copilot"]

[agents]
[agents.claude]
enabled = true

[agents.claude.mcp]
enabled = true

[agents.copilot]
enabled = true

[agents.copilot.mcp]
enabled = false

[mcp_servers.root-stdio]
command = "root-cmd"
args = ["--root"]
`,
      );

      await fs.writeFile(
        path.join( moduleRulerDir, 'ruler.toml' ),
        `default_agents = ["copilot", "windsurf"]

[agents]
[agents.copilot]
enabled = true

[agents.copilot.mcp]
enabled = true

[agents.windsurf]
enabled = false

[agents.windsurf.mcp]
enabled = false

[mcp_servers.module-remote]
url = "https://module.example"
`,
      );

      await fs.writeFile(
        path.join( submoduleRulerDir, 'ruler.toml' ),
        `default_agents = ["windsurf"]

[agents]
[agents.windsurf]
enabled = true

[agents.windsurf.mcp]
enabled = true

[mcp_servers.sub-stdio]
command = "sub-cmd"
`,
      );

      const configss = await loadNestedconfigsurations(
        tmpDir,
        undefined,
        false,
        true,
      );

      const rootconfigs = configss.find( ( c ) => c.rulerDir === rootRulerDir );
      const moduleconfigs = configss.find( ( c ) => c.rulerDir === moduleRulerDir );
      const submoduleconfigs = configss.find(
        ( c ) => c.rulerDir === submoduleRulerDir,
      );

      expect( rootconfigs?.rulerMcpJson ).toEqual( {
        mcpServers: {
          'root-stdio': expect.objectContaining( {
            command: 'root-cmd',
            args: [ '--root' ],
            type: 'stdio',
          } ),
        },
      } );

      expect( moduleconfigs?.rulerMcpJson ).toEqual( {
        mcpServers: {
          'module-remote': expect.objectContaining( {
            url: 'https://module.example',
            type: 'remote',
          } ),
        },
      } );

      expect( submoduleconfigs?.rulerMcpJson ).toEqual( {
        mcpServers: {
          'sub-stdio': expect.objectContaining( {
            command: 'sub-cmd',
            type: 'stdio',
          } ),
        },
      } );

      expect( rootconfigs?.configs.agentconfigss.claude?.mcp?.enabled ).toBe( true );
      expect( rootconfigs?.configs.agentconfigss.copilot?.mcp?.enabled ).toBe( false );
      expect( moduleconfigs?.configs.agentconfigss.copilot?.mcp?.enabled ).toBe(
        true,
      );
      expect( moduleconfigs?.configs.agentconfigss.windsurf?.mcp?.enabled ).toBe(
        false,
      );
      expect( submoduleconfigs?.configs.agentconfigss.windsurf?.mcp?.enabled ).toBe(
        true,
      );
    } );
  } );

  describe( 'processHierarchicalconfigsurations', () =>
  {
    it( 'passes each directory root and MCP bundle through to agent applications', async () =>
    {
      const rootRulerDir = path.join( tmpDir, '.ruler' );
      const nestedRulerDir = path.join( tmpDir, 'nested', '.ruler' );
      await fs.mkdir( rootRulerDir, { recursive: true } );
      await fs.mkdir( nestedRulerDir, { recursive: true } );

      const records: Array<{
        projectRoot: string;
        mcp: Record<string, unknown> | null;
      }> = [];

      class RecordingAgent extends MockAgent
      {
        async applyRulerconfigs (
          rules: string,
          projectRoot: string,
          mcpJson: Record<string, unknown> | null,
        ): Promise<void>
        {
          records.push( { projectRoot, mcp: mcpJson } );
        }
      }

      const agent = new RecordingAgent( 'Recording Agent', 'recording' );

      const configsurations: HierarchicalRulerconfigsuration[] = [
        {
          rulerDir: rootRulerDir,
          configs: { agentconfigss: { recording: {} } } as Loadedconfigs,
          concatenatedRules: '# Root',
          rulerMcpJson: { mcpServers: { root: { command: 'root' } } },
        },
        {
          rulerDir: nestedRulerDir,
          configs: { agentconfigss: { recording: {} } } as Loadedconfigs,
          concatenatedRules: '# Nested',
          rulerMcpJson: {
            mcpServers: { nested: { url: 'https://nested.example' } },
          },
        },
      ];

      await processHierarchicalconfigsurations(
        [ agent ],
        configsurations,
        false,
        false,
        true,
        undefined,
        false,
      );

      expect( records ).toEqual( [
        {
          projectRoot: path.dirname( rootRulerDir ),
          mcp: { mcpServers: { root: { command: 'root' } } },
        },
        {
          projectRoot: path.dirname( nestedRulerDir ),
          mcp: { mcpServers: { nested: { url: 'https://nested.example' } } },
        },
      ] );
    } );
  } );

  describe( 'applyconfigsurationsToAgents', () =>
  {
    it( 'should apply configsurations to all agents and return generated paths', async () =>
    {
      const mockAgents = [ new MockAgent( 'Claude Code', 'claude' ) ];
      const configs: Loadedconfigs = { agentconfigss: {} };
      const rules = '# Test rules';
      const mcpJson = null;

      const result = await applyconfigsurationsToAgents(
        mockAgents,
        rules,
        mcpJson,
        configs,
        tmpDir,
        false,
        false,
        true,
        undefined,
      );

      expect( result ).toContain( `${ tmpDir }/.claude/configs.json` );
    } );

    it( 'should handle dry run mode', async () =>
    {
      const mockAgents = [ new MockAgent( 'Claude Code', 'claude' ) ];
      const configs: Loadedconfigs = { agentconfigss: {} };
      const rules = '# Test rules';
      const mcpJson = null;

      const result = await applyconfigsurationsToAgents(
        mockAgents,
        rules,
        mcpJson,
        configs,
        tmpDir,
        false,
        true, // dry run
        true,
        undefined,
      );

      expect( result ).toContain( `${ tmpDir }/.claude/configs.json` );
    } );
  } );

  describe( 'updateGitignore', () =>
  {
    it( 'should update gitignore when enabled', async () =>
    {
      const configs: Loadedconfigs = { agentconfigss: {} };
      const generatedPaths = [ '.claude/configs.json', '.copilot/settings.json' ];

      await updateGitignore( tmpDir, generatedPaths, configs, true, false );

      const gitignoreContent = await fs.readFile(
        path.join( tmpDir, '.gitignore' ),
        'utf8',
      );
      expect( gitignoreContent ).toContain( '.claude/configs.json' );
      expect( gitignoreContent ).toContain( '.copilot/settings.json' );
    } );

    it( 'should not update gitignore when disabled', async () =>
    {
      const configs: Loadedconfigs = { agentconfigss: {} };
      const generatedPaths = [ '.claude/configs.json' ];

      await updateGitignore( tmpDir, generatedPaths, configs, false, false );

      const gitignoreExists = await fs
        .access( path.join( tmpDir, '.gitignore' ) )
        .then( () => true )
        .catch( () => false );

      expect( gitignoreExists ).toBe( false );
    } );

    it( 'should handle dry run mode', async () =>
    {
      const configs: Loadedconfigs = { agentconfigss: {} };
      const generatedPaths = [ '.claude/configs.json' ];

      await updateGitignore( tmpDir, generatedPaths, configs, true, true );

      const gitignoreExists = await fs
        .access( path.join( tmpDir, '.gitignore' ) )
        .then( () => true )
        .catch( () => false );

      expect( gitignoreExists ).toBe( false );
    } );
  } );

  describe( 'dry-run logging patterns', () =>
  {
    beforeEach( () =>
    {
      jest.clearAllMocks();
    } );

    it( 'should use [ruler:dry-run] prefix when dryRun is true', async () =>
    {
      const consoleLogSpy = jest.spyOn( console, 'log' ).mockImplementation();
      const mockAgents = [ new MockAgent( 'Claude Code', 'claude' ) ];
      const configs: Loadedconfigs = { agentconfigss: {} };
      const rules = '# Test rules';
      const mcpJson = null;

      await applyconfigsurationsToAgents(
        mockAgents,
        rules,
        mcpJson,
        configs,
        tmpDir,
        false,
        true, // dryRun=true
        true,
        undefined,
      );

      const logCalls = consoleLogSpy.mock.calls.flat();
      const hasRulerDryRunPrefix = logCalls.some(
        ( call ) => typeof call === 'string' && call.includes( '[ruler:dry-run]' ),
      );

      expect( hasRulerDryRunPrefix ).toBe( true );
      consoleLogSpy.mockRestore();
    } );

    it( 'should use [ruler] prefix when dryRun is false', async () =>
    {
      const consoleLogSpy = jest.spyOn( console, 'log' ).mockImplementation();
      const mockAgents = [ new MockAgent( 'Claude Code', 'claude' ) ];
      const configs: Loadedconfigs = { agentconfigss: {} };
      const rules = '# Test rules';
      const mcpJson = null;

      await applyconfigsurationsToAgents(
        mockAgents,
        rules,
        mcpJson,
        configs,
        tmpDir,
        false,
        false, // dryRun=false
        true,
        undefined,
      );

      const logCalls = consoleLogSpy.mock.calls.flat();
      const hasRulerPrefix = logCalls.some(
        ( call ) =>
          typeof call === 'string' &&
          call.includes( '[ruler]' ) &&
          !call.includes( '[ruler:dry-run]' ),
      );

      expect( hasRulerPrefix ).toBe( true );
      consoleLogSpy.mockRestore();
    } );
  } );

  describe( 'MCP type transformations', () =>
  {
    it( 'should transform remote type to streamable-http for Kilo Code', async () =>
    {
      const kilocodeDir = path.join( tmpDir, '.kilocode' );
      await fs.mkdir( kilocodeDir, { recursive: true } );

      const configs: Loadedconfigs = {
        agentconfigss: {
          kilocode: {
            enabled: true,
            mcp: { enabled: true },
          },
        },
      };

      const rules = '# Test rules';
      const mcpJson = {
        mcpServers: {
          'context7': {
            url: 'https://mcp.context7.com/mcp',
            type: 'remote',
            headers: {
              Authorization: 'Bearer CTX123456',
            },
          },
        },
      };

      const kilocodeAgent = new MockAgent( 'Kilo Code', 'kilocode' );

      await applyconfigsurationsToAgents(
        [ kilocodeAgent ],
        rules,
        mcpJson,
        configs,
        tmpDir,
        false,
        false,
        true,
        undefined,
        false,
      );

      const mcpPath = path.join( kilocodeDir, 'mcp.json' );
      const mcpContent = JSON.parse( await fs.readFile( mcpPath, 'utf8' ) );

      expect( mcpContent.mcpServers.context7.type ).toBe( 'streamable-http' );
      expect( mcpContent.mcpServers.context7.url ).toBe(
        'https://mcp.context7.com/mcp',
      );
      expect( mcpContent.mcpServers.context7.headers.Authorization ).toBe(
        'Bearer CTX123456',
      );
    } );

    it( 'should preserve non-remote types for Kilo Code', async () =>
    {
      const kilocodeDir = path.join( tmpDir, '.kilocode' );
      await fs.mkdir( kilocodeDir, { recursive: true } );

      const configs: Loadedconfigs = {
        agentconfigss: {
          kilocode: {
            enabled: true,
            mcp: { enabled: true },
          },
        },
      };

      const rules = '# Test rules';
      const mcpJson = {
        mcpServers: {
          'local-stdio': {
            command: 'node',
            args: [ 'server.js' ],
            type: 'stdio',
          },
        },
      };

      const kilocodeAgent = new MockAgent( 'Kilo Code', 'kilocode' );

      await applyconfigsurationsToAgents(
        [ kilocodeAgent ],
        rules,
        mcpJson,
        configs,
        tmpDir,
        false,
        false,
        true,
        undefined,
        false,
      );

      const mcpPath = path.join( kilocodeDir, 'mcp.json' );
      const mcpContent = JSON.parse( await fs.readFile( mcpPath, 'utf8' ) );

      expect( mcpContent.mcpServers[ 'local-stdio' ].type ).toBe( 'stdio' );
      expect( mcpContent.mcpServers[ 'local-stdio' ].command ).toBe( 'node' );
    } );

    it( 'should transform remote type to http for Claude Code', async () =>
    {
      const configs: Loadedconfigs = {
        agentconfigss: {
          claude: {
            enabled: true,
            mcp: { enabled: true },
          },
        },
      };

      const rules = '# Test rules';
      const mcpJson = {
        mcpServers: {
          'remote-server': {
            url: 'https://api.example.com/mcp',
            type: 'remote',
          },
        },
      };

      const claudeAgent = new ClaudeAgent();

      await applyconfigsurationsToAgents(
        [ claudeAgent ],
        rules,
        mcpJson,
        configs,
        tmpDir,
        false,
        false,
        true,
        undefined,
        false,
      );

      const mcpPath = path.join( tmpDir, '.mcp.json' );
      const mcpContent = JSON.parse( await fs.readFile( mcpPath, 'utf8' ) );

      expect( mcpContent.mcpServers[ 'remote-server' ].type ).toBe( 'http' );
    } );

    it( 'should transform remote type to sse for SSE endpoints in Claude Code', async () =>
    {
      const configs: Loadedconfigs = {
        agentconfigss: {
          claude: {
            enabled: true,
            mcp: { enabled: true },
          },
        },
      };

      const rules = '# Test rules';
      const mcpJson = {
        mcpServers: {
          'sse-server': {
            url: 'https://api.example.com/sse/events',
            type: 'remote',
          },
        },
      };

      const claudeAgent = new ClaudeAgent();

      await applyconfigsurationsToAgents(
        [ claudeAgent ],
        rules,
        mcpJson,
        configs,
        tmpDir,
        false,
        false,
        true,
        undefined,
        false,
      );

      const mcpPath = path.join( tmpDir, '.mcp.json' );
      const mcpContent = JSON.parse( await fs.readFile( mcpPath, 'utf8' ) );

      expect( mcpContent.mcpServers[ 'sse-server' ].type ).toBe( 'sse' );
    } );
  } );
} );
