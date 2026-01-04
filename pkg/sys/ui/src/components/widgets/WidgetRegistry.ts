/**
 * Widget Registry
 *
 * Manages available widgets for the dynamic UI system.
 */

export interface Widget
{
  id: string;
  name: string;
  component: string;
  category: string;
  icon?: string;
  defaultSize?: { width: number; height: number; };
  configsurable?: boolean;
}

/**
 * Widget Registry
 *
 * Central registry for all available widgets in the UI.
 */
export class WidgetRegistry
{
  private widgets: Map<string, Widget> = new Map();

  /**
   * Register a widget
   */
  register ( widget: Widget ): void
  {
    this.widgets.set( widget.id, widget );
  }

  /**
   * Get widget by ID
   */
  get ( id: string ): Widget | undefined
  {
    return this.widgets.get( id );
  }

  /**
   * Get all widgets
   */
  getAll (): Widget[]
  {
    return Array.from( this.widgets.values() );
  }

  /**
   * Get widgets by category
   */
  getByCategory ( category: string ): Widget[]
  {
    return Array.from( this.widgets.values() ).filter( w => w.category === category );
  }

  /**
   * Unregister a widget
   */
  unregister ( id: string ): void
  {
    this.widgets.delete( id );
  }
}

// Singleton instance
export const widgetRegistry = new WidgetRegistry();

// Register default widgets
widgetRegistry.register( {
  id: 'activity-log',
  name: 'Activity Log',
  component: 'ActivityLog',
  category: 'monitoring',
  defaultSize: { width: 400, height: 300 },
} );

widgetRegistry.register( {
  id: 'system-status',
  name: 'System Status',
  component: 'SystemStatus',
  category: 'monitoring',
  defaultSize: { width: 300, height: 200 },
} );

widgetRegistry.register( {
  id: 'chat',
  name: 'Chat',
  component: 'Chat',
  category: 'communication',
  defaultSize: { width: 500, height: 400 },
} );

