/**
 * Provider Client Service
 *
 * Provides abstraction layer for switching between different AI providers
 * (llama.cpp, Claude Code, Codex, etc.) with a unified interface.
 */

export interface ProviderMessage {
  role: 'user' | 'assistant' | 'system';
  content: string;
}

export interface ProviderResponse {
  content: string;
  provider: string;
  model?: string;
}

/**
 * Provider Client Service
 *
 * Manages communication with different AI providers through a unified interface.
 */
export class ProviderClient {
  private currentProvider: string = 'llama.cpp';
  private providers: string[] = ['llama.cpp', 'claude-code', 'codex'];

  /**
   * Set current provider
   */
  setProvider(provider: string): void {
    if (this.providers.includes(provider)) {
      this.currentProvider = provider;
    } else {
      console.warn(`Unknown provider: ${provider}`);
    }
  }

  /**
   * Get current provider
   */
  getProvider(): string {
    return this.currentProvider;
  }

  /**
   * Get available providers
   */
  getProviders(): string[] {
    return [...this.providers];
  }

  /**
   * Send message to provider
   */
  async sendMessage(
    message: string,
    history: Array<{ role: string; content: string }> = []
  ): Promise<ProviderResponse> {
    try {
      const url = '/api/v1/chat';
      const response = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          provider: this.currentProvider,
          message,
          history,
        }),
      });

      if (!response.ok) {
        let errorBody = '';
        try {
          const text = await response.text();
          errorBody = text ? ` - ${text}` : '';
        } catch {
          // If we can't read the response body, just continue without it
        }
        throw new Error(`Provider request failed [${response.status}] ${url}: ${response.statusText}${errorBody}`);
      }

      const data = await response.json();
      return {
        content: data.content || data.message || 'No response',
        provider: this.currentProvider,
        model: data.model,
      };
    } catch (error) {
      console.error('Provider client error:', error);
      throw error;
    }
  }

  /**
   * Stream message from provider
   */
  async *streamMessage(
    message: string,
    history: Array<{ role: string; content: string }> = []
  ): AsyncGenerator<string, void, unknown> {
    try {
      const url = '/api/v1/chat/stream';
      const response = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          provider: this.currentProvider,
          message,
          history,
        }),
      });

      if (!response.ok) {
        let errorBody = '';
        try {
          const text = await response.text();
          errorBody = text ? ` - ${text}` : '';
        } catch {
          // If we can't read the response body, just continue without it
        }
        throw new Error(`Provider stream failed [${response.status}] ${url}: ${response.statusText}${errorBody}`);
      }

      const reader = response.body?.getReader();
      const decoder = new TextDecoder();

      if (!reader) {
        throw new Error('No response body reader');
      }

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        const chunk = decoder.decode(value);
        const lines = chunk.split('\n');

        for (const line of lines) {
          if (line.startsWith('data: ')) {
            const data = line.slice(6);
            if (data === '[DONE]') {
              return;
            }
            try {
              const parsed = JSON.parse(data);
              yield parsed.content || parsed.token || '';
            } catch {
              // Skip invalid JSON
            }
          }
        }
      }
    } catch (error) {
      console.error('Provider stream error:', error);
      throw error;
    }
  }
}

// Singleton instance
export const providerClient = new ProviderClient();

