/**
 * @fileoverview Unit tests for Cursor-specific error types
 */

import { jest } from '@jest/globals';

// Import errors (will use dynamic import for ESM compatibility)
const {
  CursorError,
  CursorNotInstalledError,
  CursorNotRunningError,
  CursorConnectionError,
  CursorTimeoutError,
  CursorRateLimitError,
  CursorQuotaExceededError,
  CursorCommandError,
  CursorMcpError,
  toCursorError,
  CURSOR_ERROR_CODES,
} = await import('../../../packages/tm-core/src/common/errors/cursor-errors.js');

const { TaskMasterError, ERROR_CODES } = await import(
  '../../../packages/tm-core/src/common/errors/task-master-error.js'
);

describe('Cursor Error Types', () => {
  describe('CursorError', () => {
    it('should create a basic CursorError', () => {
      const error = new CursorError('Test error');

      expect(error).toBeInstanceOf(Error);
      expect(error).toBeInstanceOf(TaskMasterError);
      expect(error).toBeInstanceOf(CursorError);
      expect(error.name).toBe('CursorError');
      expect(error.message).toBe('Test error');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR);
    });

    it('should accept custom error code', () => {
      const error = new CursorError('Test error', CURSOR_ERROR_CODES.CURSOR_CONNECTION_ERROR);

      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_CONNECTION_ERROR);
    });

    it('should accept context information', () => {
      const error = new CursorError('Command failed', CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR, {
        command: 'cursor',
        args: ['--version'],
        exitCode: 1,
        stderr: 'Error output',
      });

      expect(error.cursorContext.command).toBe('cursor');
      expect(error.cursorContext.args).toEqual(['--version']);
      expect(error.cursorContext.exitCode).toBe(1);
      expect(error.cursorContext.stderr).toBe('Error output');
    });

    it('should support error chaining with cause', () => {
      const cause = new Error('Original error');
      const error = new CursorError('Wrapped error', CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR, {}, cause);

      expect(error.cause).toBe(cause);
      expect(error.stack).toContain('Caused by:');
    });

    describe('isRecoverable', () => {
      it('should return true for rate limit errors', () => {
        const error = new CursorError('Rate limited', CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED);
        expect(error.isRecoverable()).toBe(true);
      });

      it('should return true for timeout errors', () => {
        const error = new CursorError('Timeout', CURSOR_ERROR_CODES.CURSOR_TIMEOUT);
        expect(error.isRecoverable()).toBe(true);
      });

      it('should return true for not running errors', () => {
        const error = new CursorError('Not running', CURSOR_ERROR_CODES.CURSOR_NOT_RUNNING);
        expect(error.isRecoverable()).toBe(true);
      });

      it('should return false for permanent errors', () => {
        const error = new CursorError('Not installed', CURSOR_ERROR_CODES.CURSOR_NOT_INSTALLED);
        expect(error.isRecoverable()).toBe(false);
      });
    });

    describe('getSuggestedRetryDelay', () => {
      it('should return retryAfter from context if set', () => {
        const error = new CursorError('Rate limited', CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED, {
          retryAfter: 10000,
        });
        expect(error.getSuggestedRetryDelay()).toBe(10000);
      });

      it('should return default delay for rate limit without retryAfter', () => {
        const error = new CursorError('Rate limited', CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED);
        expect(error.getSuggestedRetryDelay()).toBe(5000);
      });

      it('should return 1000ms for timeout errors', () => {
        const error = new CursorError('Timeout', CURSOR_ERROR_CODES.CURSOR_TIMEOUT);
        expect(error.getSuggestedRetryDelay()).toBe(1000);
      });
    });

    describe('getUserMessage', () => {
      it('should add installation hint for CURSOR_NOT_INSTALLED', () => {
        const error = new CursorError('Not found', CURSOR_ERROR_CODES.CURSOR_NOT_INSTALLED);
        expect(error.getUserMessage()).toContain('https://cursor.so/');
      });

      it('should add start hint for CURSOR_NOT_RUNNING', () => {
        const error = new CursorError('Not running', CURSOR_ERROR_CODES.CURSOR_NOT_RUNNING);
        expect(error.getUserMessage()).toContain('start the Cursor application');
      });

      it('should include wait time for rate limit errors with retryAfter', () => {
        const error = new CursorError('Rate limited', CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED, {
          retryAfter: 5000,
        });
        expect(error.getUserMessage()).toContain('5 seconds');
      });
    });
  });

  describe('CursorNotInstalledError', () => {
    it('should create error with default message', () => {
      const error = new CursorNotInstalledError();

      expect(error.name).toBe('CursorNotInstalledError');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_NOT_INSTALLED);
      expect(error.message).toContain('not installed');
    });

    it('should include user-friendly message', () => {
      const error = new CursorNotInstalledError();
      expect(error.context.userMessage).toContain('https://cursor.so/');
    });
  });

  describe('CursorNotRunningError', () => {
    it('should create error with default message', () => {
      const error = new CursorNotRunningError();

      expect(error.name).toBe('CursorNotRunningError');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_NOT_RUNNING);
      expect(error.message).toContain('not running');
    });
  });

  describe('CursorConnectionError', () => {
    it('should create connection error with cause', () => {
      const cause = new Error('ECONNREFUSED');
      const error = new CursorConnectionError('Failed to connect', {}, cause);

      expect(error.name).toBe('CursorConnectionError');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_CONNECTION_ERROR);
      expect(error.cause).toBe(cause);
    });
  });

  describe('CursorTimeoutError', () => {
    it('should create timeout error with duration', () => {
      const error = new CursorTimeoutError('Timed out', 60000);

      expect(error.name).toBe('CursorTimeoutError');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_TIMEOUT);
      expect(error.context.metadata?.timeout).toBe(60000);
    });

    it('should include timeout in user message', () => {
      const error = new CursorTimeoutError('Timed out', 30000);
      expect(error.context.userMessage).toContain('30 seconds');
    });
  });

  describe('CursorRateLimitError', () => {
    it('should create rate limit error with retryAfter', () => {
      const error = new CursorRateLimitError('Too many requests', 10000);

      expect(error.name).toBe('CursorRateLimitError');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_RATE_LIMITED);
      expect(error.retryAfter).toBe(10000);
    });

    it('should include wait time in user message', () => {
      const error = new CursorRateLimitError('Too many requests', 5000);
      expect(error.context.userMessage).toContain('5 seconds');
    });
  });

  describe('CursorQuotaExceededError', () => {
    it('should create quota exceeded error', () => {
      const error = new CursorQuotaExceededError();

      expect(error.name).toBe('CursorQuotaExceededError');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_QUOTA_EXCEEDED);
      expect(error.context.userMessage).toContain('quota');
    });
  });

  describe('CursorCommandError', () => {
    it('should create command error with details', () => {
      const error = new CursorCommandError(
        'Command failed',
        'cursor',
        ['--invalid'],
        1,
        'Unknown option: --invalid'
      );

      expect(error.name).toBe('CursorCommandError');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR);
      expect(error.cursorContext.command).toBe('cursor');
      expect(error.cursorContext.args).toEqual(['--invalid']);
      expect(error.cursorContext.exitCode).toBe(1);
      expect(error.cursorContext.stderr).toBe('Unknown option: --invalid');
    });
  });

  describe('CursorMcpError', () => {
    it('should create MCP error', () => {
      const error = new CursorMcpError('MCP connection failed');

      expect(error.name).toBe('CursorMcpError');
      expect(error.code).toBe(CURSOR_ERROR_CODES.CURSOR_MCP_ERROR);
      expect(error.context.userMessage).toContain('MCP');
    });
  });

  describe('toCursorError', () => {
    it('should return CursorError unchanged', () => {
      const original = new CursorError('Original');
      const result = toCursorError(original);

      expect(result).toBe(original);
    });

    it('should convert rate limit errors', () => {
      const error = new Error('Rate limit exceeded');
      const result = toCursorError(error);

      expect(result).toBeInstanceOf(CursorRateLimitError);
    });

    it('should convert 429 errors', () => {
      const error = new Error('HTTP 429: Too Many Requests');
      const result = toCursorError(error);

      expect(result).toBeInstanceOf(CursorRateLimitError);
    });

    it('should convert timeout errors', () => {
      const error = new Error('Connection timed out');
      const result = toCursorError(error);

      expect(result).toBeInstanceOf(CursorTimeoutError);
    });

    it('should convert ETIMEDOUT errors', () => {
      const error = new Error('ETIMEDOUT');
      const result = toCursorError(error);

      expect(result).toBeInstanceOf(CursorTimeoutError);
    });

    it('should convert connection errors', () => {
      const error = new Error('ECONNREFUSED');
      const result = toCursorError(error);

      expect(result).toBeInstanceOf(CursorConnectionError);
    });

    it('should convert not found errors', () => {
      const error = new Error('command not found');
      const result = toCursorError(error);

      expect(result).toBeInstanceOf(CursorNotInstalledError);
    });

    it('should convert ENOENT errors', () => {
      const error = new Error('ENOENT: no such file or directory');
      const result = toCursorError(error);

      expect(result).toBeInstanceOf(CursorNotInstalledError);
    });

    it('should convert unknown errors to generic CursorError', () => {
      const error = new Error('Some unknown error');
      const result = toCursorError(error);

      expect(result).toBeInstanceOf(CursorError);
      expect(result.code).toBe(CURSOR_ERROR_CODES.CURSOR_COMMAND_ERROR);
    });

    it('should handle non-Error objects', () => {
      const result = toCursorError('String error');

      expect(result).toBeInstanceOf(CursorError);
      expect(result.message).toBe('String error');
    });

    it('should preserve context', () => {
      const error = new Error('Connection failed');
      const context = { command: 'cursor', args: ['--version'] };
      const result = toCursorError(error, context);

      expect(result.cursorContext.command).toBe('cursor');
      expect(result.cursorContext.args).toEqual(['--version']);
    });
  });
});
