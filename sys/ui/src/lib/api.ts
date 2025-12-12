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
