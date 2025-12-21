/**
 * Multi-Modal Input Abstraction
 *
 * Provides unified interface for voice, vision, and text input with
 * graceful degradation when hardware is unavailable.
 */

export type InputMode = 'text' | 'voice' | 'vision';

export interface MultiModalInput {
  mode: InputMode;
  data: string | Blob | File;
  metadata?: Record<string, unknown>;
}

export interface HardwareCapabilities {
  voice: boolean;
  vision: boolean;
  text: boolean;
}

/**
 * Multi-Modal Service
 *
 * Manages multi-modal input with hardware capability detection and graceful degradation.
 */
export class MultiModalService {
  private capabilities: HardwareCapabilities | null = null;

  /**
   * Detect hardware capabilities
   */
  async detectCapabilities(): Promise<HardwareCapabilities> {
    if (this.capabilities) {
      return this.capabilities;
    }

    const capabilities: HardwareCapabilities = {
      text: true, // Always available
      voice: 'webkitSpeechRecognition' in window || 'SpeechRecognition' in window,
      vision: 'navigator.mediaDevices' in navigator && 'getUserMedia' in navigator.mediaDevices,
    };

    this.capabilities = capabilities;
    return capabilities;
  }

  /**
   * Get current capabilities
   */
  getCapabilities(): HardwareCapabilities | null {
    return this.capabilities;
  }

  /**
   * Check if a mode is available
   */
  isModeAvailable(mode: InputMode): boolean {
    if (!this.capabilities) {
      return mode === 'text'; // Only text is guaranteed
    }

    switch (mode) {
      case 'text':
        return true;
      case 'voice':
        return this.capabilities.voice;
      case 'vision':
        return this.capabilities.vision;
      default:
        return false;
    }
  }

  /**
   * Process multi-modal input
   */
  async processInput(input: MultiModalInput): Promise<string> {
    // Graceful degradation: if requested mode is unavailable, fall back to text
    if (!this.isModeAvailable(input.mode)) {
      console.warn(`Mode ${input.mode} unavailable, falling back to text`);
      if (typeof input.data === 'string') {
        return input.data;
      }
      throw new Error(`Cannot process ${input.mode} input: hardware unavailable`);
    }

    switch (input.mode) {
      case 'text':
        return typeof input.data === 'string' ? input.data : '';
      case 'voice':
        // Voice input processing would go here
        // For now, return placeholder
        return '[Voice input processed]';
      case 'vision':
        // Vision input processing would go here
        // For now, return placeholder
        return '[Vision input processed]';
      default:
        throw new Error(`Unknown input mode: ${input.mode}`);
    }
  }
}

// Singleton instance
export const multiModalService = new MultiModalService();

