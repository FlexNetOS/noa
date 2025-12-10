/**
 * Hardware Capabilities Detection
 *
 * Detects available hardware capabilities for multi-modal interactions.
 */

export interface HardwareCapabilities {
  voice: {
    available: boolean;
    api: 'webkit' | 'standard' | null;
  };
  vision: {
    available: boolean;
    camera: boolean;
    screenCapture: boolean;
  };
  text: {
    available: boolean;
  };
}

/**
 * Hardware Capabilities Service
 *
 * Detects and reports available hardware capabilities.
 */
export class HardwareCapabilitiesService {
  private cached: HardwareCapabilities | null = null;

  /**
   * Detect all hardware capabilities
   */
  async detect(): Promise<HardwareCapabilities> {
    if (this.cached) {
      return this.cached;
    }

    const capabilities: HardwareCapabilities = {
      voice: {
        available: false,
        api: null,
      },
      vision: {
        available: false,
        camera: false,
        screenCapture: false,
      },
      text: {
        available: true, // Always available
      },
    };

    // Detect voice capabilities
    if (typeof window !== 'undefined') {
      if ('webkitSpeechRecognition' in window) {
        capabilities.voice.available = true;
        capabilities.voice.api = 'webkit';
      } else if ('SpeechRecognition' in window) {
        capabilities.voice.available = true;
        capabilities.voice.api = 'standard';
      }
    }

    // Detect vision capabilities
    if (typeof navigator !== 'undefined' && navigator.mediaDevices) {
      try {
        const devices = await navigator.mediaDevices.enumerateDevices();
        capabilities.vision.camera = devices.some(d => d.kind === 'videoinput');
        capabilities.vision.available = capabilities.vision.camera;
        capabilities.vision.screenCapture = 'getDisplayMedia' in navigator.mediaDevices;
      } catch (error) {
        console.warn('Failed to detect vision capabilities:', error);
      }
    }

    this.cached = capabilities;
    return capabilities;
  }

  /**
   * Get cached capabilities
   */
  getCached(): HardwareCapabilities | null {
    return this.cached;
  }

  /**
   * Clear cache (force re-detection)
   */
  clearCache(): void {
    this.cached = null;
  }
}

// Singleton instance
export const hardwareCapabilities = new HardwareCapabilitiesService();

