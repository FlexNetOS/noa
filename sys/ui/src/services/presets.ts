/**
 * User Presets Service
 *
 * Manages saving and switching between user-defined UI presets.
 */

import type { WidgetLayout } from '../components/widgets/WidgetGrid';

export interface UIPreset {
  id: string;
  name: string;
  description?: string;
  widgetLayouts: WidgetLayout[];
  settings: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}

/**
 * User Presets Service
 *
 * Handles creation, storage, and switching of UI presets.
 */
export class PresetsService {
  private storageKey = 'noa_ui_presets';
  private presets: Map<string, UIPreset> = new Map();
  private currentPresetId: string | null = null;

  /**
   * Load presets from storage
   */
  async loadPresets(): Promise<void> {
    try {
      const stored = localStorage.getItem(this.storageKey);
      if (stored) {
        const data = JSON.parse(stored);
        this.presets = new Map(Object.entries(data.presets || {}));
        this.currentPresetId = data.currentPresetId || null;
      }
    } catch (error) {
      console.error('Failed to load presets:', error);
    }
  }

  /**
   * Save presets to storage
   */
  private async savePresets(): Promise<void> {
    try {
      const data = {
        presets: Object.fromEntries(this.presets),
        currentPresetId: this.currentPresetId,
      };
      localStorage.setItem(this.storageKey, JSON.stringify(data));
    } catch (error) {
      console.error('Failed to save presets:', error);
      throw error;
    }
  }

  /**
   * Create a new preset
   */
  async createPreset(
    name: string,
    widgetLayouts: WidgetLayout[],
    settings: Record<string, unknown> = {},
    description?: string
  ): Promise<UIPreset> {
    const preset: UIPreset = {
      id: `preset_${Date.now()}`,
      name,
      description,
      widgetLayouts,
      settings,
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    };

    this.presets.set(preset.id, preset);
    await this.savePresets();

    return preset;
  }

  /**
   * Update an existing preset
   */
  async updatePreset(
    id: string,
    updates: Partial<Omit<UIPreset, 'id' | 'createdAt'>>
  ): Promise<UIPreset | null> {
    const preset = this.presets.get(id);
    if (!preset) {
      return null;
    }

    const updated: UIPreset = {
      ...preset,
      ...updates,
      updatedAt: new Date().toISOString(),
    };

    this.presets.set(id, updated);
    await this.savePresets();

    return updated;
  }

  /**
   * Delete a preset
   */
  async deletePreset(id: string): Promise<boolean> {
    if (this.presets.has(id)) {
      this.presets.delete(id);
      if (this.currentPresetId === id) {
        this.currentPresetId = null;
      }
      await this.savePresets();
      return true;
    }
    return false;
  }

  /**
   * Get all presets
   */
  getAllPresets(): UIPreset[] {
    return Array.from(this.presets.values());
  }

  /**
   * Get preset by ID
   */
  getPreset(id: string): UIPreset | null {
    return this.presets.get(id) || null;
  }

  /**
   * Switch to a preset
   */
  async switchPreset(id: string): Promise<UIPreset | null> {
    const preset = this.presets.get(id);
    if (!preset) {
      return null;
    }

    this.currentPresetId = id;
    await this.savePresets();

    return preset;
  }

  /**
   * Get current preset
   */
  getCurrentPreset(): UIPreset | null {
    if (!this.currentPresetId) {
      return null;
    }
    return this.presets.get(this.currentPresetId) || null;
  }

  /**
   * Save current state as a preset
   */
  async saveCurrentStateAsPreset(
    name: string,
    widgetLayouts: WidgetLayout[],
    settings: Record<string, unknown> = {},
    description?: string
  ): Promise<UIPreset> {
    const preset = await this.createPreset(name, widgetLayouts, settings, description);
    await this.switchPreset(preset.id);
    return preset;
  }

  /**
   * Export preset to JSON
   */
  exportPreset(id: string): string | null {
    const preset = this.presets.get(id);
    if (!preset) {
      return null;
    }
    return JSON.stringify(preset, null, 2);
  }

  /**
   * Import preset from JSON
   */
  async importPreset(json: string): Promise<UIPreset | null> {
    try {
      const preset: UIPreset = JSON.parse(json);

      // Validate preset structure
      if (!preset.id || !preset.name || !preset.widgetLayouts) {
        throw new Error('Invalid preset format');
      }

      // Update timestamps
      preset.updatedAt = new Date().toISOString();
      if (!preset.createdAt) {
        preset.createdAt = preset.updatedAt;
      }

      this.presets.set(preset.id, preset);
      await this.savePresets();

      return preset;
    } catch (error) {
      console.error('Failed to import preset:', error);
      return null;
    }
  }
}

// Singleton instance
export const presetsService = new PresetsService();

// Initialize on load
if (typeof window !== 'undefined') {
  presetsService.loadPresets();
}

