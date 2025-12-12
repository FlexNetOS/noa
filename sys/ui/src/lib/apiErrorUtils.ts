/**
 * API Error Utilities
 * 
 * Utilities for handling API errors with proper response body extraction
 */

export interface ApiError {
  status: number;
  statusText: string;
  url: string;
  body?: string;
  message: string;
}

/**
 * Extracts the error body from a Response object
 * Checks response.bodyUsed to avoid attempting to read an already consumed body
 * 
 * @param response - The Response object from a failed fetch
 * @returns Promise resolving to the error body as a string, or undefined if body cannot be read
 */
export async function extractErrorBody(response: Response): Promise<string | undefined> {
  // Check if the body has already been read
  if (response.bodyUsed) {
    return undefined;
  }

  try {
    // Attempt to read the response body as text
    const text = await response.text();
    return text;
  } catch (error) {
    // If reading fails, return undefined
    return undefined;
  }
}

/**
 * Formats an error message from response details and optional body
 * @private
 */
function formatErrorMessage(
  status: number,
  statusText: string,
  url: string,
  body?: string
): string {
  let message = `API Error: ${status} ${statusText} at ${url}`;
  
  if (body) {
    message += `\nResponse body: ${body}`;
  }
  
  return message;
}

/**
 * Creates a detailed API error message including status code, URL, and response body
 * 
 * @param response - The Response object from a failed fetch
 * @returns Promise resolving to a formatted error message string
 */
export async function createApiErrorMessage(response: Response): Promise<string> {
  const body = await extractErrorBody(response);
  return formatErrorMessage(response.status, response.statusText, response.url, body);
}

/**
 * Creates an ApiError object from a Response
 * 
 * @param response - The Response object from a failed fetch
 * @returns Promise resolving to an ApiError object
 */
export async function createApiError(response: Response): Promise<ApiError> {
  const body = await extractErrorBody(response);
  const message = formatErrorMessage(response.status, response.statusText, response.url, body);
  
  return {
    status: response.status,
    statusText: response.statusText,
    url: response.url,
    body,
    message,
  };
}
