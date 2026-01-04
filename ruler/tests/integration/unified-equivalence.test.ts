import * as path from 'path';
import { loadUnifiedconfigs } from '../../src/core/UnifiedconfigsLoader';
import
  {
    loadSingleconfigsuration,
    Rulerconfigsuration,
  } from '../../src/core/apply-engine';

describe( 'Unified configs equivalence (subset)', () =>
{
  const projectRoot = path.join( __dirname, 'fixtures/agents' );
  test( 'matches defaults and concatenated rules', async () =>
  {
    const legacy = await loadSingleconfigsuration( projectRoot, undefined, false );
    const unified = await loadUnifiedconfigs( { projectRoot } );
    const legacyconfigs = legacy as Rulerconfigsuration;
    // Legacy default agents live under legacy.configs.defaultAgents
    expect( unified.toml.defaultAgents ).toEqual(
      legacyconfigs.configs.defaultAgents,
    );
    // Both bundles should contain some markdown content (alpha.md/beta.md not created yet, so empty OK)
    expect( typeof unified.rules.concatenated ).toBe( 'string' );
    // Enabled agents set equals legacy defaults
    expect( new Set( Object.keys( unified.agents ) ) ).toEqual(
      new Set( legacyconfigs.configs.defaultAgents || [] ),
    );
  } );
} );
