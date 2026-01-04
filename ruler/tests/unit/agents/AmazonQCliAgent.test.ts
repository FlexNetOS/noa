import { AmazonQCliAgent } from '../../../src/agents/AmazonQCliAgent';
import { promises as fs } from 'fs';
import * as path from 'path';
import { mkdtemp } from 'fs/promises';
import * as os from 'os';

describe( 'AmazonQCliAgent Unit Tests', () =>
{
  let agent: AmazonQCliAgent;
  let tmpDir: string;

  beforeEach( async () =>
  {
    agent = new AmazonQCliAgent();
    tmpDir = await mkdtemp( path.join( os.tmpdir(), 'amazonq-agent-test-' ) );
  } );

  afterEach( async () =>
  {
    try
    {
      await fs.rm( tmpDir, { recursive: true, force: true } );
    } catch
    {
      // Ignore cleanup errors
    }
  } );

  describe( 'Basic Properties', () =>
  {
    it( 'returns correct identifier', () =>
    {
      expect( agent.getIdentifier() ).toBe( 'amazonqcli' );
    } );

    it( 'returns correct name', () =>
    {
      expect( agent.getName() ).toBe( 'Amazon Q CLI' );
    } );

    it( 'returns correct default output paths', () =>
    {
      const paths = agent.getDefaultOutputPath( '/tmp/test' );
      expect( paths.instructions ).toBe( '/tmp/test/.amazonq/rules/ruler_q_rules.md' );
      expect( paths.mcp ).toBe( '/tmp/test/.amazonq/mcp.json' );
    } );

    it( 'supports MCP stdio', () =>
    {
      expect( agent.supportsMcpStdio() ).toBe( true );
    } );

    it( 'supports MCP remote', () =>
    {
      expect( agent.supportsMcpRemote() ).toBe( true );
    } );

    it( 'returns correct MCP server key', () =>
    {
      expect( agent.getMcpServerKey() ).toBe( 'mcpServers' );
    } );
  } );

  describe( 'applyRulerconfigs', () =>
  {
    it( 'creates rules file in correct location', async () =>
    {
      const rules = '# Test Rules\nUse TypeScript';

      await agent.applyRulerconfigs( rules, tmpDir, null, undefined, false );

      const rulesPath = path.join( tmpDir, '.amazonq', 'rules', 'ruler_q_rules.md' );
      const content = await fs.readFile( rulesPath, 'utf8' );
      expect( content ).toBe( rules );
    } );

    it( 'creates MCP configsuration when provided', async () =>
    {
      const rules = '# Test Rules';
      const mcpconfigs = {
        mcpServers: {
          filesystem: {
            command: 'npx',
            args: [ '-y', '@modelcontextprotocol/server-filesystem', '.' ]
          }
        }
      };

      await agent.applyRulerconfigs( rules, tmpDir, mcpconfigs, undefined, false );

      const mcpPath = path.join( tmpDir, '.amazonq', 'mcp.json' );
      const mcpContent = await fs.readFile( mcpPath, 'utf8' );
      const parsedMcp = JSON.parse( mcpContent );

      expect( parsedMcp.mcpServers.filesystem ).toEqual( {
        command: 'npx',
        args: [ '-y', '@modelcontextprotocol/server-filesystem', '.' ]
      } );
    } );

    it( 'merges MCP configsuration with existing configs', async () =>
    {
      const rules = '# Test Rules';

      // First, create existing MCP configs
      const existingMcpPath = path.join( tmpDir, '.amazonq', 'mcp.json' );
      const existingMcp = {
        mcpServers: {
          existing: { command: 'existing-cmd' }
        }
      };

      await fs.mkdir( path.dirname( existingMcpPath ), { recursive: true } );
      await fs.writeFile( existingMcpPath, JSON.stringify( existingMcp, null, 2 ) );

      // Now apply new MCP configs
      const newMcpconfigs = {
        mcpServers: {
          filesystem: { command: 'npx', args: [ 'mcp-filesystem' ] }
        }
      };

      await agent.applyRulerconfigs( rules, tmpDir, newMcpconfigs, undefined, false );

      const mcpContent = await fs.readFile( existingMcpPath, 'utf8' );
      const parsedMcp = JSON.parse( mcpContent );

      // Should contain both existing and new servers
      expect( parsedMcp.mcpServers.existing ).toEqual( { command: 'existing-cmd' } );
      expect( parsedMcp.mcpServers.filesystem ).toEqual( {
        command: 'npx',
        args: [ 'mcp-filesystem' ]
      } );
    } );

    it( 'skips MCP when disabled in configs', async () =>
    {
      const rules = '# Test Rules';
      const mcpconfigs = {
        mcpServers: {
          filesystem: { command: 'npx' }
        }
      };
      const agentconfigs = {
        mcp: { enabled: false }
      };

      await agent.applyRulerconfigs( rules, tmpDir, mcpconfigs, agentconfigs, false );

      const mcpPath = path.join( tmpDir, '.amazonq', 'mcp.json' );

      // MCP file should not exist
      await expect( fs.access( mcpPath ) ).rejects.toThrow();
    } );

    it( 'uses custom output path when provided', async () =>
    {
      const rules = '# Custom Rules';
      const agentconfigs = {
        outputPath: 'custom/path/rules.md'
      };

      await agent.applyRulerconfigs( rules, tmpDir, null, agentconfigs, false );

      const customPath = path.join( tmpDir, 'custom', 'path', 'rules.md' );
      const content = await fs.readFile( customPath, 'utf8' );
      expect( content ).toBe( rules );
    } );
  } );
} );