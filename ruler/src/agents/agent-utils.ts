import { IAgent, IAgentconfigs } from './IAgent';

/**
 * Gets all output paths for an agent, taking into account any configs overrides.
 */
export function getAgentOutputPaths (
  agent: IAgent,
  projectRoot: string,
  agentconfigs?: IAgentconfigs,
): string[]
{
  const paths: string[] = [];
  const defaults = agent.getDefaultOutputPath( projectRoot );

  if ( typeof defaults === 'string' )
  {
    // Single output path (most agents)
    const actualPath = agentconfigs?.outputPath ?? defaults;
    paths.push( actualPath );
  } else
  {
    // Multiple output paths (e.g., AiderAgent)
    const defaultPaths = defaults as Record<string, string>;

    // Handle instructions path
    if ( 'instructions' in defaultPaths )
    {
      const instructionsPath =
        agentconfigs?.outputPathInstructions ?? defaultPaths.instructions;
      paths.push( instructionsPath );
    }

    // Handle configs path
    if ( 'configs' in defaultPaths )
    {
      const configsPath = agentconfigs?.outputPathconfigs ?? defaultPaths.configs;
      paths.push( configsPath );
    }

    // Handle any other paths in the default paths record
    for ( const [ key, defaultPath ] of Object.entries( defaultPaths ) )
    {
      if ( key !== 'instructions' && key !== 'configs' )
      {
        // For unknown path types, use the default since we don't have specific configs overrides
        paths.push( defaultPath );
      }
    }
  }

  return paths;
}
