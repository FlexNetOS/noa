/**
 * API Route: /api/directories
 * Manage and inspect NOA directory structure
 */

import { NextRequest, NextResponse } from 'next/server';
import { getDirectoryManager } from '@/lib/configs/directories';
import fs from 'fs';
import path from 'path';

/**
 * Get all NOA paths from DirectoryManager
 */
function getAllPaths ( manager: ReturnType<typeof getDirectoryManager> )
{
  return {
    // Immutable layer
    immutable: manager.immutable,
    schemas: manager.schemas,
    kernels: manager.kernels,
    providers: manager.providers,
    sandbox: manager.sandbox,
    trust: manager.trust,

    // Mutable layer
    mutable: manager.mutable,
    configs: manager.configs,
    agents: manager.agents,
    skills: manager.skills,
    tools: manager.tools,
    prompts: manager.prompts,
    workflows: manager.workflows,
    commands: manager.commands,
    orchestration: manager.orchestration,
    hooks: manager.hooks,

    // CAS layer
    cas: manager.cas,
    casObjects: manager.casObjects,
    casRefs: manager.casRefs,
    casTags: manager.casTags,
    casRegistry: manager.casRegistry,
    casGC: manager.casGC,
    casMerkle: manager.casMerkle,

    // Cache
    cache: manager.cache,
    cacheModels: manager.cacheModels,
    cacheEmbeddings: manager.cacheEmbeddings,

    // State
    state: manager.state,
    stateSessions: manager.stateSessions,
    stateConversations: manager.stateConversations,
    stateCheckpoints: manager.stateCheckpoints,

    // Data
    data: manager.data,
    dataIndexes: manager.dataIndexes,
    dataKnowledge: manager.dataKnowledge,

    // Logs
    logs: manager.logs,
    logsAgents: manager.logsAgents,
    logsProviders: manager.logsProviders,
    logsOrchestration: manager.logsOrchestration,
  };
}

/**
 * Get directory info (existence, file count, etc.)
 */
function getDirectoryInfo ( dirPath: string )
{
  try
  {
    const exists = fs.existsSync( dirPath );
    if ( !exists )
    {
      return { exists: false, fileCount: 0, size: 0 };
    }

    const files = fs.readdirSync( dirPath );
    const stats = files.map( file =>
    {
      try
      {
        const filePath = path.join( dirPath, file );
        const stat = fs.statSync( filePath );
        return stat.size;
      } catch
      {
        return 0;
      }
    } );

    return {
      exists: true,
      fileCount: files.length,
      size: stats.reduce( ( acc, size ) => acc + size, 0 ),
    };
  } catch ( error )
  {
    return { exists: false, fileCount: 0, size: 0, error: String( error ) };
  }
}

/**
 * List files in a directory
 */
function listFiles ( dirPath: string, ext?: string )
{
  try
  {
    if ( !fs.existsSync( dirPath ) )
    {
      return [];
    }

    let files = fs.readdirSync( dirPath );

    if ( ext )
    {
      files = files.filter( file => file.endsWith( ext ) );
    }

    return files.map( file =>
    {
      const filePath = path.join( dirPath, file );
      const stat = fs.statSync( filePath );
      return {
        name: file,
        path: filePath,
        isDirectory: stat.isDirectory(),
        size: stat.size,
        modified: stat.mtime,
      };
    } );
  } catch ( error )
  {
    return [];
  }
}

export async function GET ( request: NextRequest )
{
  const { searchParams } = new URL( request.url );
  const action = searchParams.get( 'action' );

  const manager = getDirectoryManager();
  const paths = getAllPaths( manager );

  switch ( action )
  {
    case 'info':
      // Get info for all directories
      const info: Record<string, any> = {};
      for ( const [ key, dirPath ] of Object.entries( paths ) )
      {
        info[ key ] = getDirectoryInfo( dirPath );
      }
      return NextResponse.json( info );

    case 'paths':
      return NextResponse.json( paths );

    case 'list':
      const dirKey = searchParams.get( 'dir' );
      const ext = searchParams.get( 'ext' ) || undefined;

      if ( !dirKey )
      {
        return NextResponse.json( { error: 'dir parameter required' }, { status: 400 } );
      }

      const dirPath = ( paths as any )[ dirKey ];
      if ( !dirPath )
      {
        return NextResponse.json( { error: `Invalid directory key: ${ dirKey }` }, { status: 400 } );
      }

      const files = listFiles( dirPath, ext );
      return NextResponse.json( { files } );

    default:
      // Default: return full directory info
      return NextResponse.json( {
        paths,
        info: Object.fromEntries(
          Object.entries( paths ).map( ( [ key, dirPath ] ) => [ key, getDirectoryInfo( dirPath ) ] )
        ),
      } );
  }
}

export async function POST ( request: NextRequest )
{
  try
  {
    const body = await request.json();
    const { action } = body;

    const manager = getDirectoryManager();

    switch ( action )
    {
      case 'initialize':
        manager.ensureDirectories();
        return NextResponse.json( {
          success: true,
          message: 'All NOA directories initialized',
          paths: getAllPaths( manager ),
        } );

      default:
        return NextResponse.json( { error: 'Unknown action' }, { status: 400 } );
    }
  } catch ( error )
  {
    return NextResponse.json(
      { error: error instanceof Error ? error.message : 'Unknown error' },
      { status: 500 }
    );
  }
}
