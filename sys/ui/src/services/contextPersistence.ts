/**
 * Context Persistence Service
 *
 * Manages persistence of chat context and conversation state across devices.
 */

import { createApiErrorMessage } from '../lib/apiErrorUtils';

export interface ConversationContext {
  id: string;
  messages: Array<{
    role: string;
    content: string;
    timestamp: string;
  }>;
  metadata?: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}

/**
 * Context Persistence Service
 *
 * Handles saving and loading conversation context for cross-device persistence.
 */
export class ContextPersistenceService {
  private storageKey = 'noa_conversations';

  /**
   * Save conversation context
   */
  async saveContext(context: ConversationContext): Promise<void> {
    try {
      const conversations = this.loadAllContexts();
      const index = conversations.findIndex(c => c.id === context.id);

      if (index >= 0) {
        conversations[index] = { ...context, updatedAt: new Date().toISOString() };
      } else {
        conversations.push(context);
      }

      localStorage.setItem(this.storageKey, JSON.stringify(conversations));
    } catch (error) {
      console.error('Failed to save context:', error);
      throw error;
    }
  }

  /**
   * Load conversation context by ID
   */
  async loadContext(id: string): Promise<ConversationContext | null> {
    try {
      const conversations = this.loadAllContexts();
      return conversations.find(c => c.id === id) || null;
    } catch (error) {
      console.error('Failed to load context:', error);
      return null;
    }
  }

  /**
   * Load all conversation contexts
   */
  loadAllContexts(): ConversationContext[] {
    try {
      const stored = localStorage.getItem(this.storageKey);
      return stored ? JSON.parse(stored) : [];
    } catch {
      return [];
    }
  }

  /**
   * Delete conversation context
   */
  async deleteContext(id: string): Promise<void> {
    try {
      const conversations = this.loadAllContexts();
      const filtered = conversations.filter(c => c.id !== id);
      localStorage.setItem(this.storageKey, JSON.stringify(filtered));
    } catch (error) {
      console.error('Failed to delete context:', error);
      throw error;
    }
  }

  /**
   * Sync context to remote (for cross-device persistence)
   */
  async syncToRemote(context: ConversationContext): Promise<void> {
    try {
      const url = '/api/v1/context/sync';
      const response = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(context),
      });

      if (!response.ok) {
        const errorMessage = await createApiErrorMessage(response, url, 'Sync failed');
        throw new Error(errorMessage);
      }
    } catch (error) {
      console.error('Failed to sync context:', error);
      throw error;
    }
  }

  /**
   * Sync context from remote
   */
  async syncFromRemote(): Promise<ConversationContext[]> {
    try {
      const url = '/api/v1/context/sync';
      const response = await fetch(url, {
        method: 'GET',
        headers: { 'Content-Type': 'application/json' },
      });

      if (!response.ok) {
        const errorMessage = await createApiErrorMessage(response, url, 'Sync failed');
        throw new Error(errorMessage);
      }

      const data = await response.json();
      return data.contexts || [];
    } catch (error) {
      console.error('Failed to sync context:', error);
      return [];
    }
  }
}

// Singleton instance
export const contextPersistence = new ContextPersistenceService();

