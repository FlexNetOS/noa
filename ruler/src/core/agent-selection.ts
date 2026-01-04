import { IAgent } from '../agents/IAgent';
import { Loadedconfigs } from './configsLoader';
import { createRulerError } from '../constants';

/**
 * Resolves which agents should be selected based on configsuration.
 * Handles precedence: CLI agents > default_agents > per-agent enabled flags > all agents
 *
 * @param configs Loaded configsuration containing CLI agents, default agents, and per-agent configss
 * @param allAgents Array of all available agents
 * @returns Array of agents that should be processed
 */
export function resolveSelectedAgents (
  configs: Loadedconfigs,
  allAgents: IAgent[],
): IAgent[]
{
  // CLI --agents > configs.default_agents > per-agent.enabled flags > default all
  let selected = allAgents;

  if ( configs.cliAgents && configs.cliAgents.length > 0 )
  {
    const filters = configs.cliAgents.map( ( n ) => n.toLowerCase() );

    // Check if any of the specified agents don't exist
    const validAgentIdentifiers = new Set(
      allAgents.map( ( agent ) => agent.getIdentifier() ),
    );
    const validAgentNames = new Set(
      allAgents.map( ( agent ) => agent.getName().toLowerCase() ),
    );

    const invalidAgents = filters.filter(
      ( filter ) =>
        !validAgentIdentifiers.has( filter ) &&
        ![ ...validAgentNames ].some( ( name ) => name.includes( filter ) ),
    );

    if ( invalidAgents.length > 0 )
    {
      throw createRulerError(
        `Invalid agent specified: ${ invalidAgents.join( ', ' ) }`,
        `Valid agents are: ${ [ ...validAgentIdentifiers ].join( ', ' ) }`,
      );
    }

    selected = allAgents.filter( ( agent ) =>
      filters.some(
        ( f ) =>
          agent.getIdentifier() === f ||
          agent.getName().toLowerCase().includes( f ),
      ),
    );
  } else if ( configs.defaultAgents && configs.defaultAgents.length > 0 )
  {
    const defaults = configs.defaultAgents.map( ( n ) => n.toLowerCase() );

    // Check if any of the default agents don't exist
    const validAgentIdentifiers = new Set(
      allAgents.map( ( agent ) => agent.getIdentifier() ),
    );
    const validAgentNames = new Set(
      allAgents.map( ( agent ) => agent.getName().toLowerCase() ),
    );

    const invalidAgents = defaults.filter(
      ( filter ) =>
        !validAgentIdentifiers.has( filter ) &&
        ![ ...validAgentNames ].some( ( name ) => name.includes( filter ) ),
    );

    if ( invalidAgents.length > 0 )
    {
      throw createRulerError(
        `Invalid agent specified in default_agents: ${ invalidAgents.join( ', ' ) }`,
        `Valid agents are: ${ [ ...validAgentIdentifiers ].join( ', ' ) }`,
      );
    }

    selected = allAgents.filter( ( agent ) =>
    {
      const identifier = agent.getIdentifier();
      const override = configs.agentconfigss[ identifier ]?.enabled;
      if ( override !== undefined )
      {
        return override;
      }
      return defaults.some(
        ( d ) => identifier === d || agent.getName().toLowerCase().includes( d ),
      );
    } );
  } else
  {
    selected = allAgents.filter(
      ( agent ) => configs.agentconfigss[ agent.getIdentifier() ]?.enabled !== false,
    );
  }

  return selected;
}
