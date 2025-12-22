/**
 * AI-Assisted Insights Service
 *
 * Provides AI-powered insights and recommendations for UI optimization
 * and user productivity improvements.
 */

import { contextDetector } from './contextDetector';

export interface Insight
{
  id: string;
  type: 'productivity' | 'optimization' | 'suggestion' | 'warning';
  title: string;
  message: string;
  priority: 'low' | 'medium' | 'high';
  actionable: boolean;
  action?: {
    label: string;
    handler: () => void | Promise<void>;
  };
  timestamp: string;
}

/**
 * AI-Assisted Insights Service
 *
 * Generates and manages AI-powered insights for the user.
 */
export class InsightsService
{
  private insights: Map<string, Insight> = new Map();
  private listeners: Set<( insights: Insight[] ) => void> = new Set();

  /**
   * Generate insights based on current context and activity
   */
  async generateInsights ( activity?: {
    type: string;
    data?: Record<string, unknown>;
  } ): Promise<Insight[]>
  {
    try
    {
      // Get current context
      const context = contextDetector.getCurrentContext();

      // Generate insights based on context
      const insights: Insight[] = [];

      // Example: Productivity insight for coding context
      if ( context?.type === 'coding' )
      {
        insights.push( {
          id: `insight_${ Date.now() }_1`,
          type: 'productivity',
          title: 'Coding Context Detected',
          message: 'Consider using the Code Editor widget for better code navigation.',
          priority: 'medium',
          actionable: true,
          action: {
            label: 'Show Code Editor',
            handler: async () =>
            {
              // Implementation would show code editor widget
            },
          },
          timestamp: new Date().toISOString(),
        } );
      }

      // Example: Optimization insight
      if ( activity?.type === 'performance' )
      {
        insights.push( {
          id: `insight_${ Date.now() }_2`,
          type: 'optimization',
          title: 'Performance Optimization',
          message: 'Your system performance could be improved by reducing active widgets.',
          priority: 'high',
          actionable: true,
          action: {
            label: 'Optimize Widgets',
            handler: async () =>
            {
              // Implementation would optimize widgets
            },
          },
          timestamp: new Date().toISOString(),
        } );
      }

      // Store insights
      insights.forEach( insight =>
      {
        this.insights.set( insight.id, insight );
      } );

      // Notify listeners
      this.notifyListeners();

      return insights;
    } catch ( error )
    {
      console.error( 'Failed to generate insights:', error );
      return [];
    }
  }

  /**
   * Get all insights
   */
  getAllInsights (): Insight[]
  {
    return Array.from( this.insights.values() ).sort( ( a, b ) =>
    {
      // Sort by priority and timestamp
      const priorityOrder = { high: 3, medium: 2, low: 1 };
      const priorityDiff = priorityOrder[ b.priority ] - priorityOrder[ a.priority ];
      if ( priorityDiff !== 0 ) return priorityDiff;
      return new Date( b.timestamp ).getTime() - new Date( a.timestamp ).getTime();
    } );
  }

  /**
   * Get insights by type
   */
  getInsightsByType ( type: Insight[ 'type' ] ): Insight[]
  {
    return this.getAllInsights().filter( insight => insight.type === type );
  }

  /**
   * Get high-priority insights
   */
  getHighPriorityInsights (): Insight[]
  {
    return this.getAllInsights().filter( insight => insight.priority === 'high' );
  }

  /**
   * Dismiss an insight
   */
  dismissInsight ( id: string ): void
  {
    if ( this.insights.has( id ) )
    {
      this.insights.delete( id );
      this.notifyListeners();
    }
  }

  /**
   * Dismiss all insights
   */
  dismissAllInsights (): void
  {
    this.insights.clear();
    this.notifyListeners();
  }

  /**
   * Subscribe to insights updates
   */
  subscribe ( listener: ( insights: Insight[] ) => void ): () => void
  {
    this.listeners.add( listener );
    return () =>
    {
      this.listeners.delete( listener );
    };
  }

  /**
   * Notify all listeners
   */
  private notifyListeners (): void
  {
    const insights = this.getAllInsights();
    this.listeners.forEach( listener =>
    {
      try
      {
        listener( insights );
      } catch ( error )
      {
        console.error( 'Insights listener error:', error );
      }
    } );
  }

  /**
   * Request AI-generated insights from backend
   */
  async requestAIInsights ( prompt: string ): Promise<Insight[]>
  {
    try
    {
      // In a real implementation, this would call the backend AI service
      const response = await fetch( '/api/v1/insights/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify( { prompt } ),
      } );

      if ( !response.ok )
      {
        throw new Error( `AI insights request failed: ${ response.statusText }` );
      }

      const data = await response.json();
      const insights: Insight[] = data.insights || [];

      // Store insights
      insights.forEach( insight =>
      {
        this.insights.set( insight.id, insight );
      } );

      this.notifyListeners();
      return insights;
    } catch ( error )
    {
      console.error( 'Failed to request AI insights:', error );
      return [];
    }
  }
}

// Singleton instance
export const insightsService = new InsightsService();

