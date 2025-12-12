/**
 * API Client for NOA UI
 * Provides methods to interact with the NOA backend API
 */

// Use secure protocol in production, fallback to http for local development
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 
  (process.env.NODE_ENV === 'production' ? 'https://localhost:8000' : 'http://localhost:8000');

/**
 * API Client class for making requests to the NOA backend
 */
class ApiClient {
  private baseUrl: string;

  constructor(baseUrl: string = API_BASE_URL) {
    this.baseUrl = baseUrl;
  }

  /**
   * Make a GET request to the API
   */
  private async get<T>(endpoint: string): Promise<T> {
    try {
      const response = await fetch(`${this.baseUrl}${endpoint}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error(`API request failed: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error(`API GET request failed for ${endpoint}:`, error);
      throw error;
    }
  }

  /**
   * Make a POST request to the API
   */
  private async post<T>(endpoint: string, data?: any): Promise<T> {
    try {
      const response = await fetch(`${this.baseUrl}${endpoint}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: data ? JSON.stringify(data) : undefined,
      });

      if (!response.ok) {
        throw new Error(`API request failed: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error(`API POST request failed for ${endpoint}:`, error);
      throw error;
    }
  }

  /**
   * Check API health
   */
  async getHealth(): Promise<{ status: string }> {
    return this.get('/health');
  }

  /**
   * Get activity log entries
   */
  async getActivityLog(limit: number = 100): Promise<{ activities: any[] }> {
    return this.get(`/activity?limit=${limit}`);
  }

  /**
   * Get capsules
   */
  async getCapsules(): Promise<{ capsules: any[] }> {
    return this.get('/capsules');
  }

  /**
   * Get models
   */
  async getModels(): Promise<{ models: any[] }> {
    return this.get('/models');
  }

  /**
   * Get artifacts
   */
  async getArtifacts(): Promise<{ artifacts: any[] }> {
    return this.get('/artifacts');
  }

  /**
   * Get jobs
   */
  async getJobs(): Promise<{ jobs: any[] }> {
    return this.get('/jobs');
  }
}

// Export singleton instance
export const apiClient = new ApiClient();
 * API Client for NOA backend services
 */

import { createApiErrorMessage } from './apiErrorUtils';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

class APIClient {
  private baseURL: string;

  constructor(baseURL: string = API_BASE_URL) {
    this.baseURL = baseURL;
  }

  private async request<T>(
    endpoint: string,
    options?: RequestInit
  ): Promise<T> {
    const url = `${this.baseURL}${endpoint}`;
    const response = await fetch(url, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
    });

    if (!response.ok) {
      const errorMessage = await createApiErrorMessage(response);
      throw new Error(errorMessage);
    }

    return response.json();
  }

  async get<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'GET' });
  }

  async post<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async put<T>(endpoint: string, data?: unknown): Promise<T> {
    return this.request<T>(endpoint, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async delete<T>(endpoint: string): Promise<T> {
    return this.request<T>(endpoint, { method: 'DELETE' });
  }

  // API endpoint methods
  async getHealth(): Promise<{ status: string; timestamp: number }> {
    return this.get('/api/health');
  }

  async getArtifacts(): Promise<{ artifacts: unknown[] }> {
    return this.get('/api/artifacts');
  }

  async getCapsules(): Promise<{ capsules: unknown[] }> {
    return this.get('/api/capsules');
  }

  async getJobs(): Promise<{ jobs: unknown[] }> {
    return this.get('/api/jobs');
  }

  async getModels(): Promise<{ models: unknown[] }> {
    return this.get('/api/models');
  }

  async getActivityLog(limit: number): Promise<{ entries: unknown[] }> {
    const params = new URLSearchParams({ limit: String(limit) });
    return this.get(`/api/activity?${params.toString()}`);
  }
}

export const apiClient = new APIClient();
