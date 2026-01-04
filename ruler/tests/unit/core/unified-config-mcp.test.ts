import * as path from 'path';
import { loadUnifiedconfigs } from '../../../src/core/UnifiedconfigsLoader';

// Uses existing integration fixture for simplicity

describe( 'UnifiedconfigsLoader MCP normalization', () =>
{
  // From tests/unit/core -> go up to tests, then into integration/fixtures/unified
  const projectRoot = path.join( __dirname, '../../integration/fixtures/unified' );
  test( 'normalizes mcp.json servers', async () =>
  {
    const unified = await loadUnifiedconfigs( { projectRoot } );
    // Expect non-null after implementation
    expect( unified.mcp ).not.toBeNull();
    expect( unified.mcp?.servers.example.command ).toBe( 'node' );
  } );
} );
