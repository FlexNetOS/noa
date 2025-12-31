/**
 * @fileoverview Cursor-specific error classes
 * Provides specialized error handling for Cursor CLI operations
 */

import { TaskMasterError, ERROR_CODES, ErrorContext } from './task-master-error.js';

/**
 * Cursor-specific error codes
 */
export const CURSOR_ERROR_CODES = {
  // Connection errors
  CURSOR_CONNECTION_ERROR: 'CURSOR_CONNECTION_ERROR',
  CURSOR_NOT_INSTALLED: 'CURSOR_NOT_INSTALLED',
  CURSOR_NOT_RUNNING: 'CURSOR_NOT_RUNNING',
  CURSOR_TIMEOUT: 'CURSOR_TIMEOUT',

  // Rate limiting
  CURSOR_RATE_LIMITED: 'CURSOR_RATE_LIMITED',
  CURSOR_QUOTA_EXCEEDED: 'CURSOR_QUOTA_EXCEEDED',

  // Command errors
  CURSOR_COMMAND_ERROR: 'CURSOR_COMMAND_ERROR',
  CURSOR_COMMAND_NOT_FOUND: 'CURSOR_COMMAND_NOT_FOUND',
  CURSOR_INVALID_ARGUMENT: 'CURSOR_INVALID_ARGUMENT',

  // Session errors
  CURSOR_SESSION_ERROR: 'CURSOR_SESSION_ERROR',
  CURSOR_SESSION_EXPIRED: 'CURSOR_SESSION_EXPIRED',

  // MCP errors
  CURSOR_MCP_ERROR: 'CURSOR_MCP_ERROR',
  CURSOR_MCP_CONNECTION_FAILED: 'CURSOR_MCP_CONNECTION_FAILED',
} as const;

export type CursorErrorCode = (typeof CURSOR_ERROR_CODES)[keyof typeof CURSOR_ERROR_CODES];

/**
 * Extended error context for Cursor operations
 */
export interface CursorErrorContext extends ErrorContext {
  /** Cursor CLI command that was executed */
  command?: string;
  /** Arguments passed to the command */
  args?: string[];
  /** Exit code from Cursor CLI */
  exitCode?: number;
  /** stderr output from Cursor CLI */
  stderr?: string;
  /** Retry count for rate limited requests */
  retryCount?: number;
  /** Time until rate limit resets (ms) */
  retryAfter?: number;
  /** Cursor version if detected */
  cursorVersion?: string;
}

/**
 * Base error class for all Cursor-related operations
 *
 * @example
 * ```typescript
 * throw new CursorError(
 *   'Failed to execute Cursor command',
 *   CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR,
 *   {
 *     command: 'cursor',
 *     args: ['--version'],
 *     exitCode: 1,
 *     stderr: 'Command not found'
 *   }
 * );
 * ```
 */
export class CursorError extends TaskMasterError {
  public readonly cursorContext: CursorErrorContext;

  constructor(
    message: string,
    code: CursorErrorCode | string = CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR,
    context: CursorErrorContext = {},
    cause?: Error
  ) {
    super(message, code, context, cause);
    this.name = 'CursorError';
    this.cursorContext = context;

    // Fix prototype chain
    Object.setPrototypeOf(this, CursorError.prototype);
  }

  /**
   * Get a user-friendly error message with Cursor-specific hints
   */
  public override getUserMessage(): string {
    const baseMessage = super.getUserMessage();

    // Add helpful hints based on error type
    switch (this.code) {
      case CURSOR_ERROR_CODES.CURSOR_NOT_INSTALLED:
        return `${baseMessage}\n\nHint: Install Cursor from https://cursor.so/`;

      case CURSOR_ERROR_CODES.CURSOR_NOT_RUNNING:
        return `${baseMessage}\n\nHint: Please start the Cursor application.`;

      case CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED:
        const retryAfter = this.cursorContext.retryAfter;
        return retryAfter
          ? `${baseMessage}\n\nPlease wait ${Math.ceil(retryAfter / 1000)} seconds before retrying.`
          : baseMessage;

      case CURSOR_ERROR_CODES.CURSOR_TIMEOUT:
        return `${baseMessage}\n\nHint: The Cursor CLI is taking too long to respond. Try closing unused tabs or restarting Cursor.`;

      default:
        return baseMessage;
    }
  }

  /**
   * Check if the error is recoverable (can be retried)
   */
  public isRecoverable(): boolean {
    return [
      CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED,
      CURSOR_ERROR_CODES.CURSOR_TIMEOUT,
      CURSOR_ERROR_CODES.CURSOR_NOT_RUNNING,
      ERROR_CODES.NETWORK_ERROR,
    ].includes(this.code as any);
  }

  /**
   * Get suggested retry delay in milliseconds
   */
  public getSuggestedRetryDelay(): number {
    if (this.cursorContext.retryAfter) {
      return this.cursorContext.retryAfter;
    }

    switch (this.code) {
      case CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED:
        return 5000; // 5 seconds
      case CURSOR_ERROR_CODES.CURSOR_TIMEOUT:
        return 1000; // 1 second
      case CURSOR_ERROR_CODES.CURSOR_NOT_RUNNING:
        return 2000; // 2 seconds
      default:
        return 1000;
    }
  }
}

/**
 * Error thrown when Cursor CLI is not installed or not found in PATH
 */
export class CursorNotInstalledError extends CursorError {
  constructor(message = 'Cursor CLI is not installed or not found in PATH', context?: CursorErrorContext) {
    super(message, CURSOR_ERROR_CODES.CURSOR_NOT_INSTALLED, {
      ...context,
      userMessage: 'Cursor is not installed. Please install Cursor from https://cursor.so/',
    });
    this.name = 'CursorNotInstalledError';
    Object.setPrototypeOf(this, CursorNotInstalledError.prototype);
  }
}

/**
 * Error thrown when Cursor application is not running
 */
export class CursorNotRunningError extends CursorError {
  constructor(message = 'Cursor application is not running', context?: CursorErrorContext) {
    super(message, CURSOR_ERROR_CODES.CURSOR_NOT_RUNNING, {
      ...context,
      userMessage: 'Cursor is not running. Please start the Cursor application.',
    });
    this.name = 'CursorNotRunningError';
    Object.setPrototypeOf(this, CursorNotRunningError.prototype);
  }
}

/**
 * Error thrown when connection to Cursor fails
 */
export class CursorConnectionError extends CursorError {
  constructor(message: string, context?: CursorErrorContext, cause?: Error) {
    super(message, CURSOR_ERROR_CODES.CURSOR_CONNECTION_ERROR, {
      ...context,
      userMessage: 'Failed to connect to Cursor. Please ensure Cursor is running and responsive.',
    }, cause);
    this.name = 'CursorConnectionError';
    Object.setPrototypeOf(this, CursorConnectionError.prototype);
  }
}

/**
 * Error thrown when Cursor CLI command times out
 */
export class CursorTimeoutError extends CursorError {
  constructor(
    message: string = 'Cursor CLI command timed out',
    timeout: number = 30000,
    context?: CursorErrorContext
  ) {
    super(message, CURSOR_ERROR_CODES.CURSOR_TIMEOUT, {
      ...context,
      metadata: { ...context?.metadata, timeout },
      userMessage: `Cursor command timed out after ${timeout / 1000} seconds. The operation may still be running in the background.`,
    });
    this.name = 'CursorTimeoutError';
    Object.setPrototypeOf(this, CursorTimeoutError.prototype);
  }
}

/**
 * Error thrown when rate limited by Cursor
 */
export class CursorRateLimitError extends CursorError {
  /** Time to wait before retrying (ms) */
  public readonly retryAfter: number;

  constructor(
    message: string = 'Rate limited by Cursor',
    retryAfter: number = 5000,
    context?: CursorErrorContext
  ) {
    super(message, CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED, {
      ...context,
      retryAfter,
      userMessage: `Too many requests to Cursor. Please wait ${Math.ceil(retryAfter / 1000)} seconds before retrying.`,
    });
    this.name = 'CursorRateLimitError';
    this.retryAfter = retryAfter;
    Object.setPrototypeOf(this, CursorRateLimitError.prototype);
  }
}

/**
 * Error thrown when Cursor quota is exceeded
 */
export class CursorQuotaExceededError extends CursorError {
  constructor(message: string = 'Cursor quota exceeded', context?: CursorErrorContext) {
    super(message, CURSOR_ERROR_CODES.CURSOR_QUOTA_EXCEEDED, {
      ...context,
      userMessage: 'Your Cursor usage quota has been exceeded. Please check your account or upgrade your plan.',
    });
    this.name = 'CursorQuotaExceededError';
    Object.setPrototypeOf(this, CursorQuotaExceededError.prototype);
  }
}

/**
 * Error thrown when a Cursor command is invalid
 */
export class CursorCommandError extends CursorError {
  constructor(
    message: string,
    command?: string,
    args?: string[],
    exitCode?: number,
    stderr?: string,
    cause?: Error
  ) {
    super(message, CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR, {
      command,
      args,
      exitCode,
      stderr,
      userMessage: `Failed to execute Cursor command${command ? `: ${command}` : ''}`,
    }, cause);
    this.name = 'CursorCommandError';
    Object.setPrototypeOf(this, CursorCommandError.prototype);
  }
}

/**
 * Error thrown when MCP connection to Cursor fails
 */
export class CursorMcpError extends CursorError {
  constructor(message: string, context?: CursorErrorContext, cause?: Error) {
    super(message, CURSOR_ERROR_CODES.CURSOR_MCP_ERROR, {
      ...context,
      userMessage: 'Failed to communicate with Cursor via MCP. Please check your MCP configuration.',
    }, cause);
    this.name = 'CursorMcpError';
    Object.setPrototypeOf(this, CursorMcpError.prototype);
  }
}

/**
 * Utility function to convert generic errors to Cursor-specific errors
 */
export function toCursorError(error: Error | unknown, context?: CursorErrorContext): CursorError {
  if (error instanceof CursorError) {
    return error;
  }

  if (error instanceof Error) {
    const message = error.message.toLowerCase();

    // Detect rate limiting
    if (message.includes('rate limit') || message.includes('too many requests') || message.includes('429')) {
      return new CursorRateLimitError(error.message, 5000, context);
    }

    // Detect timeout
    if (message.includes('timeout') || message.includes('timed out') || message.includes('etimedout')) {
      return new CursorTimeoutError(error.message, 30000, context);
    }

    // Detect connection errors
    if (message.includes('econnrefused') || message.includes('connection') || message.includes('network')) {
      return new CursorConnectionError(error.message, context, error);
    }

    // Detect not found / not installed
    if (message.includes('not found') || message.includes('enoent') || message.includes('command not found')) {
      return new CursorNotInstalledError(error.message, context);
    }

    // Generic cursor error
    return new CursorError(error.message, CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR, context, error);
  }

  // Unknown error type
  return new CursorError(
    String(error),
    CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR,
    context
  );
}
