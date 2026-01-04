import { promises as fs } from 'fs';
import * as path from 'path';
import os from 'os';
import
  {
    getNativeMcpPath,
    readNativeMcp,
    writeNativeMcp,
  } from '../../../src/paths/mcp';
import { mergeMcp } from '../../../src/mcp/merge';

interface Mcpconfigs
{
  mcpServers: Record<string, { command: string; args?: string[]; }>;
  [ key: string ]: unknown;
}

describe( 'KiloCode MCP Integration', () =>
{
  let tmpDir: string;

  beforeEach( async () =>
  {
    tmpDir = await fs.mkdtemp( path.join( os.tmpdir(), 'ruler-kilocode-mcp-' ) );
  } );

  afterEach( async () =>
  {
    await fs.rm( tmpDir, { recursive: true, force: true } );
  } );

  describe( 'MCP Path Resolution', () =>
  {
    it( 'resolves correct MCP path for Kilo Code', async () =>
    {
      const mcpPath = await getNativeMcpPath( 'Kilo Code', tmpDir );
      expect( mcpPath ).toBe( path.join( tmpDir, '.kilocode', 'mcp.json' ) );
    } );

    it( 'returns first candidate path when file does not exist', async () =>
    {
      const mcpPath = await getNativeMcpPath( 'Kilo Code', tmpDir );
      expect( mcpPath ).toBe( path.join( tmpDir, '.kilocode', 'mcp.json' ) );
    } );
  } );

  describe( 'MCP configsuration Handling', () =>
  {
    it( 'creates new MCP configsuration file', async () =>
    {
      const mcpPath = path.join( tmpDir, '.kilocode', 'mcp.json' );
      const mcpconfigs = {
        mcpServers: {
          filesystem: {
            command: 'npx',
            args: [ '-y', '@modelcontextprotocol/server-filesystem', tmpDir ],
          },
        },
      };

      await writeNativeMcp( mcpPath, mcpconfigs );

      // Verify file was created
      await expect( fs.access( mcpPath ) ).resolves.toBeUndefined();

      const content = JSON.parse( await fs.readFile( mcpPath, 'utf8' ) );
      expect( content.mcpServers.filesystem.command ).toBe( 'npx' );
      expect( content.mcpServers.filesystem.args ).toEqual( [
        '-y',
        '@modelcontextprotocol/server-filesystem',
        tmpDir,
      ] );
    } );

    it( 'reads existing MCP configsuration', async () =>
    {
      const mcpPath = path.join( tmpDir, '.kilocode', 'mcp.json' );
      const existingconfigs = {
        mcpServers: {
          existing: {
            command: 'existing-command',
            args: [ 'existing-arg' ],
          },
        },
      };

      await fs.mkdir( path.dirname( mcpPath ), { recursive: true } );
      await fs.writeFile( mcpPath, JSON.stringify( existingconfigs, null, 2 ) );

      const readconfigs = await readNativeMcp( mcpPath );
      expect( readconfigs ).toEqual( existingconfigs );
    } );

    it( 'returns empty object for non-existent MCP file', async () =>
    {
      const mcpPath = path.join( tmpDir, '.kilocode', 'nonexistent.json' );
      const configs = await readNativeMcp( mcpPath );
      expect( configs ).toEqual( {} );
    } );

    it( 'merges MCP configsurations correctly', async () =>
    {
      const existing = {
        mcpServers: {
          existing: { command: 'existing-cmd', args: [ 'existing-arg' ] },
        },
      };

      const newconfigs = {
        mcpServers: {
          filesystem: { command: 'npx', args: [ 'mcp-filesystem' ] },
        },
      };

      const merged = mergeMcp(
        existing,
        newconfigs,
        'merge',
        'mcpServers',
      ) as Mcpconfigs;

      expect( merged.mcpServers.existing ).toEqual( {
        command: 'existing-cmd',
        args: [ 'existing-arg' ],
      } );
      expect( merged.mcpServers.filesystem ).toEqual( {
        command: 'npx',
        args: [ 'mcp-filesystem' ],
      } );
    } );

    it( 'overwrites MCP configsurations with overwrite strategy', async () =>
    {
      const existing = {
        mcpServers: {
          existing: { command: 'existing-cmd' },
        },
      };

      const newconfigs = {
        mcpServers: {
          filesystem: { command: 'npx', args: [ 'mcp-filesystem' ] },
        },
      };

      const merged = mergeMcp(
        existing,
        newconfigs,
        'overwrite',
        'mcpServers',
      ) as Mcpconfigs;

      expect( merged.mcpServers.existing ).toBeUndefined();
      expect( merged.mcpServers.filesystem ).toEqual( {
        command: 'npx',
        args: [ 'mcp-filesystem' ],
      } );
    } );

    it( 'overwrites servers with same name during merge', async () =>
    {
      const existing = {
        mcpServers: {
          filesystem: { command: 'old-command', args: [ 'old-arg' ] },
        },
      };

      const newconfigs = {
        mcpServers: {
          filesystem: { command: 'new-command', args: [ 'new-arg' ] },
        },
      };

      const merged = mergeMcp(
        existing,
        newconfigs,
        'merge',
        'mcpServers',
      ) as Mcpconfigs;

      expect( merged.mcpServers.filesystem ).toEqual( {
        command: 'new-command',
        args: [ 'new-arg' ],
      } );
    } );

    it( 'preserves non-MCP properties during merge', async () =>
    {
      const existing = {
        mcpServers: {
          existing: { command: 'existing-cmd' },
        },
        otherProperty: 'preserved',
      };

      const newconfigs = {
        mcpServers: {
          filesystem: { command: 'npx' },
        },
      };

      const merged = mergeMcp(
        existing,
        newconfigs,
        'merge',
        'mcpServers',
      ) as Mcpconfigs;

      expect( merged.otherProperty ).toBe( 'preserved' );
      expect( merged.mcpServers.existing ).toEqual( { command: 'existing-cmd' } );
      expect( merged.mcpServers.filesystem ).toEqual( { command: 'npx' } );
    } );
  } );
} );
