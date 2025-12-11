/**
 * Contextual UI Auto-Adapt Service
 *
 * Automatically adapts UI based on detected context and user activity.
 */

import { contextDetector, type Context } from './contextDetector';

export interface UIAdaptation {
  component: string;
  action: 'show' | 'hide' | 'highlight' | 'reorder';
  priority?: number;
}

export interface ContextualUIState {
  context: Context | null;
  visibleComponents: string[];
  highlightedComponents: string[];
  suggestedTools: string[];
}

/**
 * Contextual UI Service
 *
 * Manages automatic UI adaptation based on context detection.
 */
export class ContextualUIService {
  private currentState: ContextualUIState = {
    context: null,
    visibleComponents: [],
    highlightedComponents: [],
    suggestedTools: [],
  };

  private listeners: Set<(state: ContextualUIState) => void> = new Set();

  /**
   * Update UI based on activity
   */
  adaptToActivity(activity: {
    type: string;
    data?: Record<string, unknown>;
    path?: string;
    keywords?: string[];
  }): ContextualUIState {
    // Detect context
    const context = contextDetector.detectContext(activity);

    // Generate adaptation signals
    const signals = contextDetector.generateSignals(context);

    // Update state
    this.currentState = {
      context,
      visibleComponents: this.getVisibleComponents(signals.uiAdaptations),
      highlightedComponents: this.getHighlightedComponents(signals.uiAdaptations),
      suggestedTools: signals.suggestedTools,
    };

    // Notify listeners
    this.notifyListeners();

    return this.currentState;
  }

  /**
   * Get visible components from adaptations
   */
  private getVisibleComponents(adaptations: UIAdaptation[]): string[] {
    return adaptations
      .filter(a => a.action === 'show')
      .sort((a, b) => (a.priority || 0) - (b.priority || 0))
      .map(a => a.component);
  }

  /**
   * Get highlighted components from adaptations
   */
  private getHighlightedComponents(adaptations: UIAdaptation[]): string[] {
    return adaptations
      .filter(a => a.action === 'highlight')
      .map(a => a.component);
  }

  /**
   * Get current UI state
   */
  getState(): ContextualUIState {
    return { ...this.currentState };
  }

  /**
   * Subscribe to state changes
   */
  subscribe(listener: (state: ContextualUIState) => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  /**
   * Notify all listeners
   */
  private notifyListeners(): void {
    this.listeners.forEach(listener => {
      try {
        listener(this.currentState);
      } catch (error) {
        console.error('Contextual UI listener error:', error);
      }
    });
  }

  /**
   * Manually set visible components
   */
  setVisibleComponents(components: string[]): void {
    this.currentState.visibleComponents = components;
    this.notifyListeners();
  }

  /**
   * Manually set highlighted components
   */
  setHighlightedComponents(components: string[]): void {
    this.currentState.highlightedComponents = components;
    this.notifyListeners();
  }

  /**
   * Reset to default state
   */
  reset(): void {
    this.currentState = {
      context: null,
      visibleComponents: [],
      highlightedComponents: [],
      suggestedTools: [],
    };
    this.notifyListeners();
  }
}

// Singleton instance
export const contextualUI = new ContextualUIService();

