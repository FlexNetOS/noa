/**
 * API Error Utilities
 * 
 * Shared utilities for handling API errors with enhanced context
 */

/**
 * Extracts the response body from a failed fetch response
 * Returns empty string if body cannot be read or has already been consumed
 */
export async function extractErrorBody(response: Response): Promise<string> {
  // Check if the body has already been used
  if (response.bodyUsed) {
    return '';
  }
  
  try {
    const text = await response.text();
    return text ? ` - ${text}` : '';
  } catch {
    // If we can't read the response body, just continue without it
    return '';
  }
}

/**
 * Creates a detailed error message for API errors
 * Includes status code, URL, statusText, and response body (if available)
 */
export async function createApiErrorMessage(
  response: Response,
  url: string,
  prefix: string = 'API Error'
): Promise<string> {
  const errorBody = await extractErrorBody(response);
  return `${prefix} [${response.status}] ${url}: ${response.statusText}${errorBody}`;
}
