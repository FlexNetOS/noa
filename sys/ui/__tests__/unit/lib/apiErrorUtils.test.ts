/**
 * API Error Utils Unit Tests
 *
 * Tests for the API error utility functions.
 */

import { extractErrorBody, createApiErrorMessage } from '@/lib/apiErrorUtils';

describe('API Error Utils', () => {
  describe('extractErrorBody', () => {
    it('should extract error body from response', async () => {
      const mockResponse = {
        text: jest.fn().mockResolvedValue('Error details'),
      } as unknown as Response;

      const result = await extractErrorBody(mockResponse);
      expect(result).toBe(' - Error details');
    });

    it('should return empty string when response body is empty', async () => {
      const mockResponse = {
        text: jest.fn().mockResolvedValue(''),
      } as unknown as Response;

      const result = await extractErrorBody(mockResponse);
      expect(result).toBe('');
    });

    it('should return empty string when text() throws error', async () => {
      const mockResponse = {
        text: jest.fn().mockRejectedValue(new Error('Cannot read body')),
      } as unknown as Response;

      const result = await extractErrorBody(mockResponse);
      expect(result).toBe('');
    });
  });

  describe('createApiErrorMessage', () => {
    it('should create error message with all components', async () => {
      const mockResponse = {
        status: 404,
        statusText: 'Not Found',
        text: jest.fn().mockResolvedValue('Resource not available'),
      } as unknown as Response;

      const result = await createApiErrorMessage(
        mockResponse,
        'http://localhost:8080/api/test'
      );

      expect(result).toBe(
        'API Error [404] http://localhost:8080/api/test: Not Found - Resource not available'
      );
    });

    it('should use custom prefix', async () => {
      const mockResponse = {
        status: 500,
        statusText: 'Internal Server Error',
        text: jest.fn().mockResolvedValue(''),
      } as unknown as Response;

      const result = await createApiErrorMessage(
        mockResponse,
        '/api/sync',
        'Sync failed'
      );

      expect(result).toBe(
        'Sync failed [500] /api/sync: Internal Server Error'
      );
    });

    it('should work without error body', async () => {
      const mockResponse = {
        status: 401,
        statusText: 'Unauthorized',
        text: jest.fn().mockResolvedValue(''),
      } as unknown as Response;

      const result = await createApiErrorMessage(
        mockResponse,
        '/api/protected'
      );

      expect(result).toBe(
        'API Error [401] /api/protected: Unauthorized'
      );
    });

    it('should handle error body extraction failure gracefully', async () => {
      const mockResponse = {
        status: 503,
        statusText: 'Service Unavailable',
        text: jest.fn().mockRejectedValue(new Error('Cannot read')),
      } as unknown as Response;

      const result = await createApiErrorMessage(
        mockResponse,
        '/api/service'
      );

      expect(result).toBe(
        'API Error [503] /api/service: Service Unavailable'
      );
    });
  });
});
