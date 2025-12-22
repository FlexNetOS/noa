import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

const nextArgs = process.argv.slice( 2 );

if ( nextArgs.length === 0 )
{
    console.error( 'Usage: node scripts/run-next.mjs <next-subcommand> [...args]' );
    process.exit( 1 );
}

// Workaround for environments where the native SWC addon fails to initialize on Windows.
//
// Next will fall back to wasm, but it prints noisy warnings when it has to download/cache it
// into next/wasm. If we point Next directly at a local wasm build directory, it will load wasm
// without logging those native-load attempts.
//
// NOTE: NEXT_TEST_WASM_DIR is an internal Next.js flag. We only set it on Windows, and only
// if the caller didn't already set it.
if ( process.platform === 'win32' && !process.env.NEXT_TEST_WASM_DIR )
{
    const candidates = [
        // Fall back to Next's on-demand cached download location.
        path.join( process.cwd(), 'node_modules', 'next', 'wasm', '@next', 'swc-wasm-nodejs' ),
        // Prefer an explicit dependency if present.
        // NOTE: with pnpm this path may be a symlink/junction into the pnpm store.
        path.join( process.cwd(), 'node_modules', '@next', 'swc-wasm-nodejs' )
    ];

    const selected = candidates.find( ( dir ) =>
    {
        try
        {
            return fs.existsSync( path.join( dir, 'wasm.js' ) );
        } catch
        {
            return false;
        }
    } );

    if ( selected )
    {
        process.env.NEXT_TEST_WASM_DIR = selected;
    }
}

const nextBin = path.join( process.cwd(), 'node_modules', 'next', 'dist', 'bin', 'next' );

const result = spawnSync( process.execPath, [ nextBin, ...nextArgs ], {
    stdio: 'inherit',
    env: process.env
} );

if ( result.error )
{
    console.error( result.error );
    process.exit( 1 );
}

process.exit( result.status ?? 1 );