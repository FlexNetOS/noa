/**
 * Widget Persistence Service
 *
 * Manages saving and loading widget layouts and configsurations.
 */

import type { WidgetLayout } from '../components/widgets/WidgetGrid';

export interface Widgetconfigs
{
  layouts: WidgetLayout[];
  preferences: Record<string, unknown>;
  version: string;
}

/**
 * Widget Persistence Service
 *
 * Handles persistence of widget layouts and configsurations.
 */
export class WidgetPersistenceService
{
  private storageKey = 'noa_widget_layouts';
  private version = '1.0.0';

  /**
   * Save widget layouts
   */
  async saveLayouts ( layouts: WidgetLayout[] ): Promise<void>
  {
    try
    {
      const configs: Widgetconfigs = {
        layouts,
        preferences: {},
        version: this.version,
      };
      localStorage.setItem( this.storageKey, JSON.stringify( configs ) );
    } catch ( error )
    {
      console.error( 'Failed to save widget layouts:', error );
      throw error;
    }
  }

  /**
   * Load widget layouts
   */
  async loadLayouts (): Promise<WidgetLayout[]>
  {
    try
    {
      const stored = localStorage.getItem( this.storageKey );
      if ( !stored )
      {
        return [];
      }

      const configs: Widgetconfigs = JSON.parse( stored );

      // Validate version compatibility
      if ( configs.version !== this.version )
      {
        console.warn( `Widget configs version mismatch: ${ configs.version } vs ${ this.version }` );
        // Could implement migration logic here
      }

      return configs.layouts || [];
    } catch ( error )
    {
      console.error( 'Failed to load widget layouts:', error );
      return [];
    }
  }

  /**
   * Save widget preferences
   */
  async savePreferences ( widgetId: string, preferences: Record<string, unknown> ): Promise<void>
  {
    try
    {
      const stored = localStorage.getItem( this.storageKey );
      const configs: Widgetconfigs = stored ? JSON.parse( stored ) : {
        layouts: [],
        preferences: {},
        version: this.version,
      };

      configs.preferences[ widgetId ] = preferences;
      localStorage.setItem( this.storageKey, JSON.stringify( configs ) );
    } catch ( error )
    {
      console.error( 'Failed to save widget preferences:', error );
      throw error;
    }
  }

  /**
   * Load widget preferences
   */
  async loadPreferences ( widgetId: string ): Promise<Record<string, unknown>>
  {
    try
    {
      const stored = localStorage.getItem( this.storageKey );
      if ( !stored )
      {
        return {} as Record<string, unknown>;
      }

      const configs: Widgetconfigs = JSON.parse( stored );
      return ( configs.preferences[ widgetId ] || {} ) as Record<string, unknown>;
    } catch ( error )
    {
      console.error( 'Failed to load widget preferences:', error );
      return {} as Record<string, unknown>;
    }
  }

  /**
   * Clear all widget data
   */
  async clear (): Promise<void>
  {
    try
    {
      localStorage.removeItem( this.storageKey );
    } catch ( error )
    {
      console.error( 'Failed to clear widget data:', error );
      throw error;
    }
  }

  /**
   * Sync layouts to remote (for cross-device persistence)
   */
  async syncToRemote ( layouts: WidgetLayout[] ): Promise<void>
  {
    try
    {
      const response = await fetch( '/api/v1/widgets/sync', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify( { layouts } ),
      } );

      if ( !response.ok )
      {
        throw new Error( `Sync failed: ${ response.statusText }` );
      }
    } catch ( error )
    {
      console.error( 'Failed to sync widget layouts:', error );
      throw error;
    }
  }

  /**
   * Sync layouts from remote
   */
  async syncFromRemote (): Promise<WidgetLayout[]>
  {
    try
    {
      const response = await fetch( '/api/v1/widgets/sync', {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
      } );

      if ( !response.ok )
      {
        throw new Error( `Sync failed: ${ response.statusText }` );
      }

      const data = await response.json();
      return data.layouts || [];
    } catch ( error )
    {
      console.error( 'Failed to sync widget layouts:', error );
      return [];
    }
  }
}

// Singleton instance
export const widgetPersistence = new WidgetPersistenceService();

