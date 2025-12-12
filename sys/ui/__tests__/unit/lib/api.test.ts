/**
 * API Client Unit Tests
 *
 * Tests for the API client error handling.
 */

import { apiClient } from '@/lib/api';

// Mock fetch globally
global.fetch = jest.fn();

describe('API Client', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('Error Handling', () => {
    it('should include status code, URL, and statusText in error message', async () => {
      const mockResponse = {
        ok: false,
        status: 404,
        statusText: 'Not Found',
        bodyUsed: false,
        text: jest.fn().mockResolvedValue(''),
      };

      (global.fetch as jest.Mock).mockResolvedValue(mockResponse);

      await expect(apiClient.get('/test-endpoint')).rejects.toThrow(
        'API Error [404] http://localhost:8080/test-endpoint: Not Found'
      );
    });

    it('should include response body in error message when available', async () => {
      const mockResponse = {
        ok: false,
        status: 400,
        statusText: 'Bad Request',
        bodyUsed: false,
        text: jest.fn().mockResolvedValue('Invalid input data'),
      };

      (global.fetch as jest.Mock).mockResolvedValue(mockResponse);

      await expect(apiClient.get('/test-endpoint')).rejects.toThrow(
        'API Error [400] http://localhost:8080/test-endpoint: Bad Request - Invalid input data'
      );
    });

    it('should handle error when response body cannot be read', async () => {
      const mockResponse = {
        ok: false,
        status: 500,
        statusText: 'Internal Server Error',
        bodyUsed: false,
        text: jest.fn().mockRejectedValue(new Error('Cannot read body')),
      };

      (global.fetch as jest.Mock).mockResolvedValue(mockResponse);

      await expect(apiClient.get('/test-endpoint')).rejects.toThrow(
        'API Error [500] http://localhost:8080/test-endpoint: Internal Server Error'
      );
    });

    it('should include full URL with base URL in error message', async () => {
      const mockResponse = {
        ok: false,
        status: 401,
        statusText: 'Unauthorized',
        bodyUsed: false,
        text: jest.fn().mockResolvedValue('Authentication required'),
      };

      (global.fetch as jest.Mock).mockResolvedValue(mockResponse);

      await expect(apiClient.post('/auth/login', { username: 'test' })).rejects.toThrow(
        'API Error [401] http://localhost:8080/auth/login: Unauthorized - Authentication required'
      );
    });

    it('should work with PUT requests', async () => {
      const mockResponse = {
        ok: false,
        status: 403,
        statusText: 'Forbidden',
        bodyUsed: false,
        text: jest.fn().mockResolvedValue('Access denied'),
      };

      (global.fetch as jest.Mock).mockResolvedValue(mockResponse);

      await expect(apiClient.put('/resource/123', { data: 'test' })).rejects.toThrow(
        'API Error [403] http://localhost:8080/resource/123: Forbidden - Access denied'
      );
    });

    it('should work with DELETE requests', async () => {
      const mockResponse = {
        ok: false,
        status: 409,
        statusText: 'Conflict',
        bodyUsed: false,
        text: jest.fn().mockResolvedValue('Resource is locked'),
      };

      (global.fetch as jest.Mock).mockResolvedValue(mockResponse);

      await expect(apiClient.delete('/resource/456')).rejects.toThrow(
        'API Error [409] http://localhost:8080/resource/456: Conflict - Resource is locked'
      );
    });
  });

  describe('Successful Requests', () => {
    it('should return data on successful GET request', async () => {
      const mockData = { id: 1, name: 'Test' };
      const mockResponse = {
        ok: true,
        json: jest.fn().mockResolvedValue(mockData),
      };

      (global.fetch as jest.Mock).mockResolvedValue(mockResponse);

      const result = await apiClient.get('/test-endpoint');
      expect(result).toEqual(mockData);
    });

    it('should return data on successful POST request', async () => {
      const mockData = { id: 2, created: true };
      const mockResponse = {
        ok: true,
        json: jest.fn().mockResolvedValue(mockData),
      };

      (global.fetch as jest.Mock).mockResolvedValue(mockResponse);

      const result = await apiClient.post('/test-endpoint', { name: 'New Item' });
      expect(result).toEqual(mockData);
    });
  });
});
