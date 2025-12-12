/**
 * API Client for NOA backend services
 */

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
      throw new Error(`API Error: ${response.statusText}`);
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
    return this.get(`/api/activity?limit=${limit}`);
  }
}

export const apiClient = new APIClient();
