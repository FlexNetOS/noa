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
 * Creates a detailed API error message including status code, URL, and response body
 * 
 * @param response - The Response object from a failed fetch
 * @returns Promise resolving to a formatted error message string
 */
export async function createApiErrorMessage(response: Response): Promise<string> {
  const body = await extractErrorBody(response);
  
  let message = `API Error: ${response.status} ${response.statusText} at ${response.url}`;
  
  if (body) {
    message += `\nResponse body: ${body}`;
  }
  
  return message;
}

/**
 * Creates an ApiError object from a Response
 * 
 * @param response - The Response object from a failed fetch
 * @returns Promise resolving to an ApiError object
 */
export async function createApiError(response: Response): Promise<ApiError> {
  // Extract the body once to avoid consuming it twice
  const body = await extractErrorBody(response);
  
  // Manually construct the message since we already have the body
  let message = `API Error: ${response.status} ${response.statusText} at ${response.url}`;
  
  if (body) {
    message += `\nResponse body: ${body}`;
  }
  
  return {
    status: response.status,
    statusText: response.statusText,
    url: response.url,
    body,
    message,
  };
}
