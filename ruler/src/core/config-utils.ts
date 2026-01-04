import { IAgent, IAgentconfigs } from '../agents/IAgent';

/**
 * Maps raw agent configsuration keys to their corresponding agent identifiers.
 *
 * This function normalizes configsuration keys by matching them against agent identifiers
 * and display names. It performs both exact matching (case-insensitive) with agent
 * identifiers and substring matching (case-insensitive) with agent display names
 * for backwards compatibility.
 *
 * @param raw Raw agent configsurations with user-provided keys
 * @param agents Array of all available agents
 * @returns Record with agent identifiers as keys and their configsurations as values
 */
export function mapRawAgentconfigss (
  raw: Record<string, IAgentconfigs>,
  agents: IAgent[],
): Record<string, IAgentconfigs>
{
  const mappedconfigss: Record<string, IAgentconfigs> = {};

  for ( const [ key, cfg ] of Object.entries( raw ) )
  {
    const lowerKey = key.toLowerCase();
    for ( const agent of agents )
    {
      const identifier = agent.getIdentifier();
      // Exact match with identifier or substring match with display name for backwards compatibility
      if (
        identifier === lowerKey ||
        agent.getName().toLowerCase().includes( lowerKey )
      )
      {
        mappedconfigss[ identifier ] = cfg;
      }
    }
  }

  return mappedconfigss;
}
