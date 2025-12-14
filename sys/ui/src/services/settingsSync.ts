/**
 * Settings Sync Service
 *
 * Manages cross-platform synchronization of settings across IDE, CLI, and web interfaces.
 * Supports global, per-device, and per-project sync scopes with conflict resolution.
 */

export type SyncScope = 'global' | 'per_device' | 'per_project';

export interface SyncConflict {
  key: string;
  localValue: unknown;
  remoteValue: unknown;
  timestamp: string;
}

export interface SyncResult {
  success: boolean;
  conflicts?: SyncConflict[];
  syncedKeys?: string[];
  error?: string;
}

/**
 * Settings Sync Service
 *
 * Handles synchronization of settings across different platforms and devices.
 */
export class SettingsSyncService {
  private scope: SyncScope = 'global';
  private syncEnabled = true;
  private conflictResolution: 'last_write_wins' | 'manual' | 'merge' = 'last_write_wins';

  /**
   * Set sync scope
   */
  setScope(scope: SyncScope): void {
    this.scope = scope;
  }

  /**
   * Get current sync scope
   */
  getScope(): SyncScope {
    return this.scope;
  }

  /**
   * Enable or disable sync
   */
  setEnabled(enabled: boolean): void {
    this.syncEnabled = enabled;
  }

  /**
   * Check if sync is enabled
   */
  isEnabled(): boolean {
    return this.syncEnabled;
  }

  /**
   * Set conflict resolution strategy
   */
  setConflictResolution(strategy: 'last_write_wins' | 'manual' | 'merge'): void {
    this.conflictResolution = strategy;
  }

  /**
   * Sync settings to remote
   */
  async syncToRemote(settings: Record<string, unknown>): Promise<SyncResult> {
    if (!this.syncEnabled) {
      return { success: false, error: 'Sync is disabled' };
    }

    try {
      // In a real implementation, this would call the backend API
      // For now, we'll simulate the sync
      const response = await fetch('/api/v1/settings/sync', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          scope: this.scope,
          settings,
        }),
      });

      if (!response.ok) {
        throw new Error(`Sync failed: ${response.statusText}`);
      }

      const data = await response.json();
      return {
        success: true,
        syncedKeys: Object.keys(settings),
        conflicts: data.conflicts,
      };
    } catch (error) {
      console.error('Settings sync failed:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  /**
   * Sync settings from remote
   */
  async syncFromRemote(): Promise<SyncResult> {
    if (!this.syncEnabled) {
      return { success: false, error: 'Sync is disabled' };
    }

    try {
      const response = await fetch(`/api/v1/settings/sync?scope=${this.scope}`, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
      });

      if (!response.ok) {
        throw new Error(`Sync failed: ${response.statusText}`);
      }

      const data = await response.json();
      const conflicts: SyncConflict[] = [];

      // Detect conflicts by comparing local and remote values
      if (data.settings) {
        const localSettings = this.getLocalSettings();
        for (const [key, remoteValue] of Object.entries(data.settings)) {
          const localValue = localSettings[key];
          if (localValue !== undefined && localValue !== remoteValue) {
            conflicts.push({
              key,
              localValue,
              remoteValue,
              timestamp: new Date().toISOString(),
            });
          }
        }
      }

      return {
        success: true,
        conflicts: conflicts.length > 0 ? conflicts : undefined,
      };
    } catch (error) {
      console.error('Settings sync failed:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  /**
   * Resolve conflicts
   */
  async resolveConflicts(
    conflicts: SyncConflict[],
    resolutions: Record<string, unknown>
  ): Promise<SyncResult> {
    try {
      const response = await fetch('/api/v1/settings/sync/resolve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          scope: this.scope,
          resolutions,
        }),
      });

      if (!response.ok) {
        throw new Error(`Conflict resolution failed: ${response.statusText}`);
      }

      return {
        success: true,
        syncedKeys: Object.keys(resolutions),
      };
    } catch (error) {
      console.error('Conflict resolution failed:', error);
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  /**
   * Get local settings (from localStorage or API)
   */
  private getLocalSettings(): Record<string, unknown> {
    try {
      const stored = localStorage.getItem('noa_settings');
      return stored ? JSON.parse(stored) : {};
    } catch {
      return {};
    }
  }

  /**
   * Auto-resolve conflicts based on strategy
   */
  autoResolveConflicts(conflicts: SyncConflict[]): Record<string, unknown> {
    const resolutions: Record<string, unknown> = {};

    for (const conflict of conflicts) {
      switch (this.conflictResolution) {
        case 'last_write_wins':
          // Use remote value (assumed to be newer)
          resolutions[conflict.key] = conflict.remoteValue;
          break;
        case 'merge':
          // Try to merge if both are objects
          if (
            typeof conflict.localValue === 'object' &&
            typeof conflict.remoteValue === 'object' &&
            conflict.localValue !== null &&
            conflict.remoteValue !== null
          ) {
            resolutions[conflict.key] = {
              ...(conflict.localValue as Record<string, unknown>),
              ...(conflict.remoteValue as Record<string, unknown>),
            };
          } else {
            resolutions[conflict.key] = conflict.remoteValue;
          }
          break;
        case 'manual':
          // Keep local value, require manual resolution
          resolutions[conflict.key] = conflict.localValue;
          break;
      }
    }

    return resolutions;
  }
}

// Singleton instance
export const settingsSync = new SettingsSyncService();

