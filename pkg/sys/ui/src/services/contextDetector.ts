/**
 * Context Detection Service
 *
 * Detects the current user context (coding task, project management, etc.)
 * and provides context-aware UI adaptation signals.
 */

export type ContextType =
  | 'coding'
  | 'project_management'
  | 'research'
  | 'communication'
  | 'administration'
  | 'general';

export interface Context {
  type: ContextType;
  confidence: number;
  metadata?: Record<string, unknown>;
}

export interface ContextSignal {
  context: Context;
  suggestedTools: string[];
  uiAdaptations: UIAdaptation[];
}

export interface UIAdaptation {
  component: string;
  action: 'show' | 'hide' | 'highlight' | 'reorder';
  priority?: number;
}

/**
 * Context Detector Service
 *
 * Analyzes user activity and current state to determine context
 * and provide UI adaptation recommendations.
 */
export class ContextDetector {
  private currentContext: Context | null = null;
  private history: Context[] = [];

  /**
   * Detect context from current activity
   */
  detectContext(activity: {
    type: string;
    data?: Record<string, unknown>;
    path?: string;
    keywords?: string[];
  }): Context {
    let contextType: ContextType = 'general';
    let confidence = 0.5;

    // Analyze activity type
    if (activity.type.includes('code') || activity.type.includes('file') || activity.path?.match(/\.(ts|tsx|js|jsx|rs|go|py)$/)) {
      contextType = 'coding';
      confidence = 0.8;
    } else if (activity.type.includes('task') || activity.type.includes('project')) {
      contextType = 'project_management';
      confidence = 0.7;
    } else if (activity.type.includes('search') || activity.type.includes('research')) {
      contextType = 'research';
      confidence = 0.7;
    } else if (activity.type.includes('message') || activity.type.includes('chat')) {
      contextType = 'communication';
      confidence = 0.6;
    } else if (activity.type.includes('admin') || activity.type.includes('settings')) {
      contextType = 'administration';
      confidence = 0.7;
    }

    // Analyze keywords
    if (activity.keywords) {
      const codingKeywords = ['function', 'class', 'import', 'export', 'def', 'fn'];
      const projectKeywords = ['task', 'todo', 'issue', 'milestone', 'deadline'];
      const researchKeywords = ['search', 'find', 'analyze', 'study', 'research'];

      const hasCodingKeywords = activity.keywords.some(k => codingKeywords.includes(k.toLowerCase()));
      const hasProjectKeywords = activity.keywords.some(k => projectKeywords.includes(k.toLowerCase()));
      const hasResearchKeywords = activity.keywords.some(k => researchKeywords.includes(k.toLowerCase()));

      if (hasCodingKeywords) {
        contextType = 'coding';
        confidence = Math.max(confidence, 0.9);
      } else if (hasProjectKeywords) {
        contextType = 'project_management';
        confidence = Math.max(confidence, 0.8);
      } else if (hasResearchKeywords) {
        contextType = 'research';
        confidence = Math.max(confidence, 0.8);
      }
    }

    const context: Context = {
      type: contextType,
      confidence,
      metadata: activity.data,
    };

    this.currentContext = context;
    this.history.push(context);

    // Keep only last 100 contexts
    if (this.history.length > 100) {
      this.history.shift();
    }

    return context;
  }

  /**
   * Get current context
   */
  getCurrentContext(): Context | null {
    return this.currentContext;
  }

  /**
   * Get context history
   */
  getHistory(): Context[] {
    return [...this.history];
  }

  /**
   * Generate UI adaptation signals based on context
   */
  generateSignals(context: Context): ContextSignal {
    const suggestedTools: string[] = [];
    const uiAdaptations: UIAdaptation[] = [];

    switch (context.type) {
      case 'coding':
        suggestedTools.push('Code Editor', 'Terminal', 'File Explorer', 'Git');
        uiAdaptations.push(
          { component: 'CodeEditor', action: 'show', priority: 1 },
          { component: 'Terminal', action: 'show', priority: 2 },
          { component: 'FileExplorer', action: 'show', priority: 3 }
        );
        break;

      case 'project_management':
        suggestedTools.push('Task Board', 'Calendar', 'Timeline', 'Reports');
        uiAdaptations.push(
          { component: 'TaskBoard', action: 'show', priority: 1 },
          { component: 'Calendar', action: 'show', priority: 2 }
        );
        break;

      case 'research':
        suggestedTools.push('Search', 'Knowledge Graph', 'Notes', 'References');
        uiAdaptations.push(
          { component: 'Search', action: 'highlight', priority: 1 },
          { component: 'KnowledgeGraph', action: 'show', priority: 2 }
        );
        break;

      case 'communication':
        suggestedTools.push('Chat', 'Messages', 'Contacts');
        uiAdaptations.push(
          { component: 'Chat', action: 'show', priority: 1 }
        );
        break;

      case 'administration':
        suggestedTools.push('Settings', 'Users', 'Permissions', 'Logs');
        uiAdaptations.push(
          { component: 'Settings', action: 'show', priority: 1 },
          { component: 'ActivityLog', action: 'show', priority: 2 }
        );
        break;

      default:
        // General context - show common tools
        suggestedTools.push('Dashboard', 'Chat', 'Activity Log');
    }

    return {
      context,
      suggestedTools,
      uiAdaptations,
    };
  }

  /**
   * Reset context detector
   */
  reset(): void {
    this.currentContext = null;
    this.history = [];
  }
}

// Singleton instance
export const contextDetector = new ContextDetector();

