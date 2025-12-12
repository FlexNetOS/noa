/**
 * Widget Persistence Service
 *
 * Manages saving and loading widget layouts and configurations.
 */

import type { WidgetLayout } from '../components/widgets/WidgetGrid';

export interface WidgetConfig {
  layouts: WidgetLayout[];
  preferences: Record<string, unknown>;
  version: string;
}

/**
 * Widget Persistence Service
 *
 * Handles persistence of widget layouts and configurations.
 */
export class WidgetPersistenceService {
  private storageKey = 'noa_widget_layouts';
  private version = '1.0.0';

  /**
   * Save widget layouts
   */
  async saveLayouts(layouts: WidgetLayout[]): Promise<void> {
    try {
      const config: WidgetConfig = {
        layouts,
        preferences: {},
        version: this.version,
      };
      localStorage.setItem(this.storageKey, JSON.stringify(config));
    } catch (error) {
      console.error('Failed to save widget layouts:', error);
      throw error;
    }
  }

  /**
   * Load widget layouts
   */
  async loadLayouts(): Promise<WidgetLayout[]> {
    try {
      const stored = localStorage.getItem(this.storageKey);
      if (!stored) {
        return [];
      }

      const config: WidgetConfig = JSON.parse(stored);

      // Validate version compatibility
      if (config.version !== this.version) {
        console.warn(`Widget config version mismatch: ${config.version} vs ${this.version}`);
        // Could implement migration logic here
      }

      return config.layouts || [];
    } catch (error) {
      console.error('Failed to load widget layouts:', error);
      return [];
    }
  }

  /**
   * Save widget preferences
   */
  async savePreferences(widgetId: string, preferences: Record<string, unknown>): Promise<void> {
    try {
      const stored = localStorage.getItem(this.storageKey);
      const config: WidgetConfig = stored ? JSON.parse(stored) : {
        layouts: [],
        preferences: {},
        version: this.version,
      };

      config.preferences[widgetId] = preferences;
      localStorage.setItem(this.storageKey, JSON.stringify(config));
    } catch (error) {
      console.error('Failed to save widget preferences:', error);
      throw error;
    }
  }

  /**
   * Load widget preferences
   */
  async loadPreferences(widgetId: string): Promise<Record<string, unknown>> {
    try {
      const stored = localStorage.getItem(this.storageKey);
      if (!stored) {
        return {} as Record<string, unknown>;
      }

      const config: WidgetConfig = JSON.parse(stored);
      return (config.preferences[widgetId] || {}) as Record<string, unknown>;
    } catch (error) {
      console.error('Failed to load widget preferences:', error);
      return {} as Record<string, unknown>;
    }
  }

  /**
   * Clear all widget data
   */
  async clear(): Promise<void> {
    try {
      localStorage.removeItem(this.storageKey);
    } catch (error) {
      console.error('Failed to clear widget data:', error);
      throw error;
    }
  }

  /**
   * Sync layouts to remote (for cross-device persistence)
   */
  async syncToRemote(layouts: WidgetLayout[]): Promise<void> {
    try {
      const url = '/api/v1/widgets/sync';
      const response = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ layouts }),
      });

      if (!response.ok) {
        let errorBody = '';
        try {
          const text = await response.text();
          errorBody = text ? ` - ${text}` : '';
        } catch {
          // If we can't read the response body, just continue without it
        }
        throw new Error(`Sync failed [${response.status}] ${url}: ${response.statusText}${errorBody}`);
      }
    } catch (error) {
      console.error('Failed to sync widget layouts:', error);
      throw error;
    }
  }

  /**
   * Sync layouts from remote
   */
  async syncFromRemote(): Promise<WidgetLayout[]> {
    try {
      const url = '/api/v1/widgets/sync';
      const response = await fetch(url, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
      });

      if (!response.ok) {
        let errorBody = '';
        try {
          const text = await response.text();
          errorBody = text ? ` - ${text}` : '';
        } catch {
          // If we can't read the response body, just continue without it
        }
        throw new Error(`Sync failed [${response.status}] ${url}: ${response.statusText}${errorBody}`);
      }

      const data = await response.json();
      return data.layouts || [];
    } catch (error) {
      console.error('Failed to sync widget layouts:', error);
      return [];
    }
  }
}

// Singleton instance
export const widgetPersistence = new WidgetPersistenceService();

