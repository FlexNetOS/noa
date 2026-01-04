import { resolveSelectedAgents } from '../../../src/core/agent-selection';
import { Loadedconfigs } from '../../../src/core/configsLoader';
import { IAgent } from '../../../src/agents/IAgent';

// Mock agent implementation for testing
class MockAgent implements IAgent
{
  constructor ( private name: string, private identifier: string ) { }

  getIdentifier (): string
  {
    return this.identifier;
  }

  getName (): string
  {
    return this.name;
  }

  async applyRulerconfigs (): Promise<void>
  {
    // Mock implementation
  }

  getDefaultOutputPath (): string
  {
    return `.${ this.identifier }/configs.json`;
  }
}

describe( 'resolveSelectedAgents', () =>
{
  const mockAgents = [
    new MockAgent( 'Claude Code', 'claude' ),
    new MockAgent( 'GitHub Copilot', 'copilot' ),
    new MockAgent( 'Cursor', 'cursor' ),
  ];

  it( 'should select agents based on CLI filters', () =>
  {
    const configs: Loadedconfigs = {
      cliAgents: [ 'claude', 'cursor' ],
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 2 );
    expect( result.map( a => a.getIdentifier() ) ).toEqual( [ 'claude', 'cursor' ] );
  } );

  it( 'should select agents based on CLI filters using partial name matches', () =>
  {
    const configs: Loadedconfigs = {
      cliAgents: [ 'copilot' ],
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 1 );
    expect( result[ 0 ].getIdentifier() ).toBe( 'copilot' );
  } );

  it( 'should throw error for invalid CLI agent identifiers', () =>
  {
    const configs: Loadedconfigs = {
      cliAgents: [ 'invalid-agent' ],
      agentconfigss: {},
    };

    expect( () => resolveSelectedAgents( configs, mockAgents ) ).toThrow(
      'Invalid agent specified: invalid-agent'
    );
  } );

  it( 'should select agents based on default_agents when no CLI filters', () =>
  {
    const configs: Loadedconfigs = {
      defaultAgents: [ 'copilot' ],
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 1 );
    expect( result[ 0 ].getIdentifier() ).toBe( 'copilot' );
  } );

  it( 'should respect enabled flag in agent configss when using default_agents', () =>
  {
    const configs: Loadedconfigs = {
      defaultAgents: [ 'claude', 'copilot' ],
      agentconfigss: {
        claude: { enabled: false },
        copilot: { enabled: true },
      },
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 1 );
    expect( result[ 0 ].getIdentifier() ).toBe( 'copilot' );
  } );

  it( 'should throw error for invalid default_agents', () =>
  {
    const configs: Loadedconfigs = {
      defaultAgents: [ 'invalid-default' ],
      agentconfigss: {},
    };

    expect( () => resolveSelectedAgents( configs, mockAgents ) ).toThrow(
      'Invalid agent specified in default_agents: invalid-default'
    );
  } );

  it( 'should select all enabled agents when no filters or defaults', () =>
  {
    const configs: Loadedconfigs = {
      agentconfigss: {
        claude: { enabled: false },
      },
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 2 );
    expect( result.map( a => a.getIdentifier() ).sort() ).toEqual( [ 'copilot', 'cursor' ] );
  } );

  it( 'should select all agents when no configsuration is provided', () =>
  {
    const configs: Loadedconfigs = {
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 3 );
    expect( result.map( a => a.getIdentifier() ).sort() ).toEqual( [ 'claude', 'copilot', 'cursor' ] );
  } );

  it( 'should handle CLI agents precedence over default_agents', () =>
  {
    const configs: Loadedconfigs = {
      cliAgents: [ 'claude' ],
      defaultAgents: [ 'copilot', 'cursor' ],
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 1 );
    expect( result[ 0 ].getIdentifier() ).toBe( 'claude' );
  } );

  it( 'should handle partial name matches in CLI agents', () =>
  {
    const configs: Loadedconfigs = {
      cliAgents: [ 'code' ], // Should match "Claude Code"
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 1 );
    expect( result[ 0 ].getIdentifier() ).toBe( 'claude' );
  } );

  it( 'should handle partial name matches in default_agents', () =>
  {
    const configs: Loadedconfigs = {
      defaultAgents: [ 'code' ], // Should match "Claude Code"
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 1 );
    expect( result[ 0 ].getIdentifier() ).toBe( 'claude' );
  } );

  it( 'should include agents with explicit enabled=true even when not in default_agents', () =>
  {
    const configs: Loadedconfigs = {
      defaultAgents: [ 'claude' ], // Only claude is in defaults
      agentconfigss: {
        copilot: { enabled: true }, // Explicitly enabled but not in defaults - should be included
        claude: { enabled: true },  // In defaults and enabled
      },
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 2 );
    expect( result.map( a => a.getIdentifier() ).sort() ).toEqual( [ 'claude', 'copilot' ] );
  } );

  it( 'should exclude agents with explicit enabled=false even when in default_agents', () =>
  {
    const configs: Loadedconfigs = {
      defaultAgents: [ 'claude', 'copilot' ], // Both in defaults
      agentconfigss: {
        copilot: { enabled: false }, // Explicitly disabled even though in defaults
      },
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 1 );
    expect( result[ 0 ].getIdentifier() ).toBe( 'claude' );
  } );

  it( 'should handle explicit disable override in default_agents', () =>
  {
    const configs: Loadedconfigs = {
      defaultAgents: [ 'claude', 'copilot' ],
      agentconfigss: {
        claude: { enabled: false }, // Explicitly disabled
        copilot: { enabled: undefined }, // Should default to included because in default_agents
      },
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 1 );
    expect( result[ 0 ].getIdentifier() ).toBe( 'copilot' );
  } );

  it( 'should handle case insensitive agent matching', () =>
  {
    const configs: Loadedconfigs = {
      cliAgents: [ 'CLAUDE', 'CURSOR' ],
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 2 );
    expect( result.map( a => a.getIdentifier() ) ).toEqual( [ 'claude', 'cursor' ] );
  } );

  it( 'should handle empty CLI agents array', () =>
  {
    const configs: Loadedconfigs = {
      cliAgents: [],
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 3 );
    expect( result.map( a => a.getIdentifier() ).sort() ).toEqual( [ 'claude', 'copilot', 'cursor' ] );
  } );

  it( 'should handle empty default agents array', () =>
  {
    const configs: Loadedconfigs = {
      defaultAgents: [],
      agentconfigss: {},
    };

    const result = resolveSelectedAgents( configs, mockAgents );

    expect( result ).toHaveLength( 3 );
    expect( result.map( a => a.getIdentifier() ).sort() ).toEqual( [ 'claude', 'copilot', 'cursor' ] );
  } );
} );