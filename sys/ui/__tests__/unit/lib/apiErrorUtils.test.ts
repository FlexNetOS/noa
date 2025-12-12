/**
 * API Error Utilities Unit Tests
 * 
 * Tests for API error handling utilities with proper Response object mocking
 */

import {
  extractErrorBody,
  createApiErrorMessage,
  createApiError,
  type ApiError,
} from '@/lib/apiErrorUtils';

describe('API Error Utilities', () => {
  /**
   * Creates a mock Response object that accurately simulates the real Response API
   * Includes the bodyUsed property to properly track if the body has been consumed
   */
  function createMockResponse(options: {
    status: number;
    statusText: string;
    url: string;
    body?: string;
    bodyUsed?: boolean;
  }): Response {
    let bodyConsumed = options.bodyUsed ?? false;
    const bodyText = options.body ?? '';

    // Use Object.defineProperty to create a proper getter for bodyUsed
    const mockResponse: any = {
      status: options.status,
      statusText: options.statusText,
      url: options.url,
      ok: options.status >= 200 && options.status < 300,
      
      // Mock the text() method to consume the body
      text: jest.fn(async () => {
        if (bodyConsumed) {
          throw new TypeError('Body has already been consumed');
        }
        bodyConsumed = true;
        return bodyText;
      }),
      
      // Mock json() method for completeness
      json: jest.fn(async () => {
        if (bodyConsumed) {
          throw new TypeError('Body has already been consumed');
        }
        bodyConsumed = true;
        return JSON.parse(bodyText);
      }),
      
      // Other Response properties for completeness
      headers: new Headers(),
      redirected: false,
      type: 'basic' as ResponseType,
      clone: jest.fn(),
      arrayBuffer: jest.fn(),
      blob: jest.fn(),
      formData: jest.fn(),
      body: null,
    };

    // Define bodyUsed as a getter property
    Object.defineProperty(mockResponse, 'bodyUsed', {
      get() {
        return bodyConsumed;
      },
      enumerable: true,
      configurable: true,
    });

    return mockResponse as Response;
  }

  describe('extractErrorBody', () => {
    it('should extract error body from a fresh Response', async () => {
      const mockResponse = createMockResponse({
        status: 404,
        statusText: 'Not Found',
        url: 'https://api.example.com/resource',
        body: 'Resource not found',
        bodyUsed: false,
      });

      const body = await extractErrorBody(mockResponse);

      expect(body).toBe('Resource not found');
      expect(mockResponse.bodyUsed).toBe(true);
    });

    it('should return undefined if body has already been consumed', async () => {
      const mockResponse = createMockResponse({
        status: 500,
        statusText: 'Internal Server Error',
        url: 'https://api.example.com/error',
        body: 'Server error',
        bodyUsed: true, // Body already consumed
      });

      const body = await extractErrorBody(mockResponse);

      expect(body).toBeUndefined();
    });

    it('should handle empty response body', async () => {
      const mockResponse = createMockResponse({
        status: 204,
        statusText: 'No Content',
        url: 'https://api.example.com/delete',
        body: '',
        bodyUsed: false,
      });

      const body = await extractErrorBody(mockResponse);

      expect(body).toBe('');
    });

    it('should handle error when reading body fails', async () => {
      const mockResponse = {
        status: 500,
        statusText: 'Internal Server Error',
        url: 'https://api.example.com/error',
        bodyUsed: false,
        text: jest.fn().mockRejectedValue(new Error('Network error')),
      } as unknown as Response;

      const body = await extractErrorBody(mockResponse);

      expect(body).toBeUndefined();
    });
  });

  describe('createApiErrorMessage', () => {
    it('should create error message with response body', async () => {
      const mockResponse = createMockResponse({
        status: 400,
        statusText: 'Bad Request',
        url: 'https://api.example.com/validate',
        body: '{"error": "Invalid input"}',
        bodyUsed: false,
      });

      const message = await createApiErrorMessage(mockResponse);

      expect(message).toContain('API Error: 400 Bad Request');
      expect(message).toContain('https://api.example.com/validate');
      expect(message).toContain('{"error": "Invalid input"}');
    });

    it('should create error message without body if bodyUsed is true', async () => {
      const mockResponse = createMockResponse({
        status: 403,
        statusText: 'Forbidden',
        url: 'https://api.example.com/protected',
        body: 'Access denied',
        bodyUsed: true,
      });

      const message = await createApiErrorMessage(mockResponse);

      expect(message).toContain('API Error: 403 Forbidden');
      expect(message).toContain('https://api.example.com/protected');
      expect(message).not.toContain('Access denied');
    });

    it('should handle responses with no body', async () => {
      const mockResponse = createMockResponse({
        status: 404,
        statusText: 'Not Found',
        url: 'https://api.example.com/missing',
        body: '',
        bodyUsed: false,
      });

      const message = await createApiErrorMessage(mockResponse);

      expect(message).toContain('API Error: 404 Not Found');
      expect(message).toContain('https://api.example.com/missing');
    });
  });

  describe('createApiError', () => {
    it('should create ApiError object with all properties', async () => {
      const mockResponse = createMockResponse({
        status: 500,
        statusText: 'Internal Server Error',
        url: 'https://api.example.com/crash',
        body: 'Unexpected error occurred',
        bodyUsed: false,
      });

      const apiError: ApiError = await createApiError(mockResponse);

      expect(apiError.status).toBe(500);
      expect(apiError.statusText).toBe('Internal Server Error');
      expect(apiError.url).toBe('https://api.example.com/crash');
      expect(apiError.body).toBe('Unexpected error occurred');
      expect(apiError.message).toContain('API Error: 500 Internal Server Error');
      expect(apiError.message).toContain('Unexpected error occurred');
    });

    it('should create ApiError without body if bodyUsed is true', async () => {
      const mockResponse = createMockResponse({
        status: 401,
        statusText: 'Unauthorized',
        url: 'https://api.example.com/auth',
        body: 'Invalid token',
        bodyUsed: true,
      });

      const apiError: ApiError = await createApiError(mockResponse);

      expect(apiError.status).toBe(401);
      expect(apiError.statusText).toBe('Unauthorized');
      expect(apiError.url).toBe('https://api.example.com/auth');
      expect(apiError.body).toBeUndefined();
      expect(apiError.message).toContain('API Error: 401 Unauthorized');
    });

    it('should handle various HTTP error codes', async () => {
      const testCases = [
        { status: 400, statusText: 'Bad Request' },
        { status: 401, statusText: 'Unauthorized' },
        { status: 403, statusText: 'Forbidden' },
        { status: 404, statusText: 'Not Found' },
        { status: 500, statusText: 'Internal Server Error' },
        { status: 502, statusText: 'Bad Gateway' },
        { status: 503, statusText: 'Service Unavailable' },
      ];

      for (const testCase of testCases) {
        const mockResponse = createMockResponse({
          status: testCase.status,
          statusText: testCase.statusText,
          url: 'https://api.example.com/test',
          body: `Error ${testCase.status}`,
          bodyUsed: false,
        });

        const apiError = await createApiError(mockResponse);

        expect(apiError.status).toBe(testCase.status);
        expect(apiError.statusText).toBe(testCase.statusText);
        expect(apiError.body).toBe(`Error ${testCase.status}`);
      }
    });
  });

  describe('Mock Response bodyUsed behavior', () => {
    it('should accurately simulate Response.bodyUsed property', async () => {
      const mockResponse = createMockResponse({
        status: 200,
        statusText: 'OK',
        url: 'https://api.example.com/test',
        body: 'test body',
        bodyUsed: false,
      });

      // Initially, body should not be used
      expect(mockResponse.bodyUsed).toBe(false);

      // After reading the body, bodyUsed should be true
      await mockResponse.text();
      expect(mockResponse.bodyUsed).toBe(true);

      // Attempting to read again should throw an error (real Response behavior)
      await expect(mockResponse.text()).rejects.toThrow('Body has already been consumed');
    });

    it('should respect initial bodyUsed state', () => {
      const mockResponse = createMockResponse({
        status: 200,
        statusText: 'OK',
        url: 'https://api.example.com/test',
        body: 'test body',
        bodyUsed: true, // Already consumed
      });

      expect(mockResponse.bodyUsed).toBe(true);
    });
  });
});
