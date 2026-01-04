import * as path from 'path';
import * as fs from 'fs';
import { IAgent, IAgentconfigs } from './IAgent';
import
  {
    backupFile,
    writeGeneratedFile,
    ensureDirExists,
  } from '../core/FileSystemUtils';

/**
 * Firebender rule configsuration object.
 */
interface FirebenderRule
{
  filePathMatches: string;
  rulesPaths: string;
}

/**
 * Firebender configsuration structure.
 */
interface Firebenderconfigs
{
  rules: ( FirebenderRule | string )[];
  mcpServers?: Record<string, unknown>;
}

/**
 * Firebender agent adapter.
 */
export class FirebenderAgent implements IAgent
{
  /**
   * Type guard function to safely check if an object is a FirebenderRule.
   */
  private isFirebenderRule ( rule: unknown ): rule is FirebenderRule
  {
    return (
      typeof rule === 'object' &&
      rule !== null &&
      'filePathMatches' in rule &&
      'rulesPaths' in rule &&
      typeof ( rule as Record<string, unknown> ).filePathMatches === 'string' &&
      typeof ( rule as Record<string, unknown> ).rulesPaths === 'string'
    );
  }

  getIdentifier (): string
  {
    return 'firebender';
  }

  getName (): string
  {
    return 'Firebender';
  }

  async applyRulerconfigs (
    concatenatedRules: string,
    projectRoot: string,
    rulerMcpJson: Record<string, unknown> | null,
    agentconfigs?: IAgentconfigs,
    backup = true,
  ): Promise<void>
  {
    const rulesPath = this.resolveOutputPath( projectRoot, agentconfigs );
    await ensureDirExists( path.dirname( rulesPath ) );

    const firebenderconfigs = await this.loadExistingconfigs( rulesPath );
    const newRules = this.createRulesFromConcatenatedRules(
      concatenatedRules,
      projectRoot,
    );

    firebenderconfigs.rules.push( ...newRules );
    this.removeDuplicateRules( firebenderconfigs );

    const mcpEnabled = agentconfigs?.mcp?.enabled ?? true;
    if ( mcpEnabled && rulerMcpJson )
    {
      await this.handleMcpconfigsuration(
        firebenderconfigs,
        rulerMcpJson,
        agentconfigs,
      );
    }

    await this.saveconfigs( rulesPath, firebenderconfigs, backup );
  }

  private resolveOutputPath (
    projectRoot: string,
    agentconfigs?: IAgentconfigs,
  ): string
  {
    const outputPaths = this.getDefaultOutputPath( projectRoot );
    const output =
      agentconfigs?.outputPath ??
      agentconfigs?.outputPathInstructions ??
      outputPaths[ 'instructions' ];
    return path.resolve( projectRoot, output );
  }

  private async loadExistingconfigs (
    rulesPath: string,
  ): Promise<Firebenderconfigs>
  {
    try
    {
      const existingContent = await fs.promises.readFile( rulesPath, 'utf8' );
      const configs = JSON.parse( existingContent );

      if ( !configs.rules )
      {
        configs.rules = [];
      }

      return configs;
    } catch ( error: unknown )
    {
      if (
        error &&
        typeof error === 'object' &&
        'code' in error &&
        ( error as { code?: string; } ).code === 'ENOENT'
      )
      {
        return { rules: [] };
      }
      console.warn( `Failed to read/parse existing firebender.json: ${ error }` );
      return { rules: [] };
    }
  }

  private createRulesFromConcatenatedRules (
    concatenatedRules: string,
    projectRoot: string,
  ): ( FirebenderRule | string )[]
  {
    const filePaths = this.extractFilePathsFromRules(
      concatenatedRules,
      projectRoot,
    );

    if ( filePaths.length > 0 )
    {
      return this.createRuleObjectsFromFilePaths( filePaths );
    } else
    {
      return this.createRulesFromPlainText( concatenatedRules );
    }
  }

  private createRuleObjectsFromFilePaths (
    filePaths: string[],
  ): FirebenderRule[]
  {
    return filePaths.map( ( filePath ) => ( {
      filePathMatches: '**/*',
      rulesPaths: filePath,
    } ) );
  }

  private createRulesFromPlainText ( concatenatedRules: string ): string[]
  {
    return concatenatedRules.split( '\n' ).filter( ( rule ) => rule.trim() );
  }

  private removeDuplicateRules ( firebenderconfigs: Firebenderconfigs ): void
  {
    const seen = new Set<string>();
    firebenderconfigs.rules = firebenderconfigs.rules.filter(
      ( rule: FirebenderRule | string ) =>
      {
        let key: string;
        if ( this.isFirebenderRule( rule ) )
        {
          const filePathMatchesPart = rule.filePathMatches;
          const rulesPathsPart = rule.rulesPaths;
          key = `${ filePathMatchesPart }::${ rulesPathsPart }`;
        } else
        {
          key = String( rule );
        }

        if ( seen.has( key ) )
        {
          return false;
        }
        seen.add( key );
        return true;
      },
    );
  }

  private async saveconfigs (
    rulesPath: string,
    configs: Firebenderconfigs,
    backup: boolean,
  ): Promise<void>
  {
    const updatedContent = JSON.stringify( configs, null, 2 );

    if ( backup )
    {
      await backupFile( rulesPath );
    }

    await writeGeneratedFile( rulesPath, updatedContent );
  }

  /**
   * Handle MCP server configsuration for Firebender.
   * Merges or overwrites MCP servers in the firebender.json configsuration based on strategy.
   */
  private async handleMcpconfigsuration (
    firebenderconfigs: Firebenderconfigs,
    rulerMcpJson: Record<string, unknown>,
    agentconfigs?: IAgentconfigs,
  ): Promise<void>
  {
    const strategy = agentconfigs?.mcp?.strategy ?? 'merge';

    const incomingServers =
      ( rulerMcpJson.mcpServers as Record<string, unknown> ) || {};

    if ( !firebenderconfigs.mcpServers )
    {
      firebenderconfigs.mcpServers = {};
    }

    if ( strategy === 'overwrite' )
    {
      firebenderconfigs.mcpServers = { ...incomingServers };
    } else if ( strategy === 'merge' )
    {
      const existingServers = firebenderconfigs.mcpServers || {};
      firebenderconfigs.mcpServers = { ...existingServers, ...incomingServers };
    }
  }

  getDefaultOutputPath ( projectRoot: string ): Record<string, string>
  {
    return {
      instructions: path.join( projectRoot, 'firebender.json' ),
      mcp: path.join( projectRoot, 'firebender.json' ),
    };
  }

  getMcpServerKey (): string
  {
    return 'mcpServers';
  }

  supportsMcpStdio (): boolean
  {
    return true;
  }

  supportsMcpRemote (): boolean
  {
    return true;
  }

  /**
   * Extracts file paths from concatenated rules by parsing HTML source comments.
   * @param concatenatedRules The concatenated rules string with HTML comments
   * @param projectRoot The project root directory
   * @returns Array of file paths relative to project root
   */
  private extractFilePathsFromRules (
    concatenatedRules: string,
    projectRoot: string,
  ): string[]
  {
    const sourceCommentRegex = /<!-- Source: (.+?) -->/g;
    const filePaths: string[] = [];
    let match;

    while ( ( match = sourceCommentRegex.exec( concatenatedRules ) ) !== null )
    {
      const relativePath = match[ 1 ];
      const absolutePath = path.resolve( projectRoot, relativePath );

      const normalizedProjectRoot = path.resolve( projectRoot );
      // Ensure the absolutePath is within the project root (cross-platform compatible)
      // This prevents path traversal attacks while handling Windows/Unix path differences
      const isWithinProject =
        absolutePath.startsWith( normalizedProjectRoot ) &&
        ( absolutePath.length === normalizedProjectRoot.length ||
          absolutePath[ normalizedProjectRoot.length ] === path.sep );
      if ( isWithinProject )
      {
        const projectRelativePath = path.relative( projectRoot, absolutePath );
        filePaths.push( projectRelativePath );
      }
    }

    return filePaths;
  }
}
