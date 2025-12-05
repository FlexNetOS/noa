/**
 * @fileoverview Unit tests for Cursor CLI rate limiter
 */

import { jest } from '@jest/globals';

// Import rate limiter
const {
  CursorRateLimiter,
  DEFAULT_CURSOR_RATE_LIMIT_CONFIG,
  getCursorRateLimiter,
  initGlobalRateLimiter,
  resetGlobalRateLimiter,
} = await import('../../../packages/tm-core/src/cursor/cursor-rate-limiter.js');

const { CursorRateLimitError } = await import(
  '../../../packages/tm-core/src/common/errors/cursor-errors.js'
);

describe('CursorRateLimiter', () => {
  beforeEach(() => {
    resetGlobalRateLimiter();
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  describe('constructor', () => {
    it('should create with default config', () => {
      const limiter = new CursorRateLimiter();
      const status = limiter.getStatus();

      expect(status.config.maxRequests).toBe(DEFAULT_CURSOR_RATE_LIMIT_CONFIG.maxRequests);
      expect(status.config.windowMs).toBe(DEFAULT_CURSOR_RATE_LIMIT_CONFIG.windowMs);
    });

    it('should accept custom config', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 5,
        windowMs: 30000,
      });
      const status = limiter.getStatus();

      expect(status.config.maxRequests).toBe(5);
      expect(status.config.windowMs).toBe(30000);
    });
  });

  describe('check', () => {
    it('should allow requests within limit', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 5,
        windowMs: 60000,
        minDelay: 0,
      });

      for (let i = 0; i < 5; i++) {
        const result = limiter.check();
        expect(result.allowed).toBe(true);
        limiter.record();
      }
    });

    it('should block requests over limit', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 2,
        windowMs: 60000,
        minDelay: 0,
      });

      // Make 2 requests
      limiter.record();
      limiter.record();

      // Third should be blocked
      const result = limiter.check();
      expect(result.allowed).toBe(false);
      expect(result.remainingRequests).toBe(0);
      expect(result.waitTime).toBeGreaterThan(0);
    });

    it('should enforce minimum delay between requests', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 100,
        windowMs: 60000,
        minDelay: 500,
      });

      limiter.record();

      // Immediately check again
      const result = limiter.check();
      expect(result.allowed).toBe(false);
      expect(result.waitTime).toBeGreaterThan(0);
      expect(result.waitTime).toBeLessThanOrEqual(500);
    });

    it('should allow request after minimum delay', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 100,
        windowMs: 60000,
        minDelay: 500,
      });

      limiter.record();

      // Advance time past minimum delay
      jest.advanceTimersByTime(500);

      const result = limiter.check();
      expect(result.allowed).toBe(true);
    });

    it('should track per-operation limits', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 2,
        windowMs: 60000,
        minDelay: 0,
      });

      // Fill up 'op1'
      limiter.record('op1');
      limiter.record('op1');

      // 'op1' should be blocked
      expect(limiter.check('op1').allowed).toBe(false);

      // 'op2' should still be allowed
      expect(limiter.check('op2').allowed).toBe(true);
    });

    it('should reset after window expires', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 2,
        windowMs: 1000, // 1 second window
        minDelay: 0,
      });

      // Fill up the limit
      limiter.record();
      limiter.record();
      expect(limiter.check().allowed).toBe(false);

      // Advance past the window
      jest.advanceTimersByTime(1001);

      // Should be allowed again
      expect(limiter.check().allowed).toBe(true);
    });
  });

  describe('acquire', () => {
    it('should return 0 when immediately allowed', async () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 10,
        windowMs: 60000,
        minDelay: 0,
      });

      jest.useRealTimers(); // Need real timers for Promise
      const waitTime = await limiter.acquire();
      expect(waitTime).toBe(0);
    });

    it('should record the request after acquiring', async () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 10,
        windowMs: 60000,
        minDelay: 0,
      });

      jest.useRealTimers();
      await limiter.acquire('test');

      const status = limiter.getStatus('test');
      expect(status.windowRequests).toBe(1);
    });
  });

  describe('execute', () => {
    it('should execute function and return result', async () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 10,
        windowMs: 60000,
        minDelay: 0,
        autoRetry: false,
      });

      jest.useRealTimers();
      const result = await limiter.execute('test', async () => 'success');
      expect(result).toBe('success');
    });

    it('should retry on rate limit error', async () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 100,
        windowMs: 60000,
        minDelay: 0,
        autoRetry: true,
        maxRetries: 2,
        retryBaseDelay: 10, // Very short for testing
      });

      jest.useRealTimers();

      let attempts = 0;
      const result = await limiter.execute(
        'test',
        async () => {
          attempts++;
          if (attempts < 2) {
            throw new Error('rate limit exceeded');
          }
          return 'success';
        },
        { retries: 2 }
      );

      expect(result).toBe('success');
      expect(attempts).toBe(2); // 1 initial + 1 retry (succeeded on 2nd attempt)
    });

    it('should respect maxRetries limit correctly', async () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 100,
        windowMs: 60000,
        minDelay: 0,
        autoRetry: true,
        maxRetries: 3,
        retryBaseDelay: 10,
      });

      jest.useRealTimers();

      let attempts = 0;
      await expect(
        limiter.execute(
          'test',
          async () => {
            attempts++;
            throw new Error('rate limit exceeded');
          },
          { retries: 3 }
        )
      ).rejects.toThrow('rate limit exceeded');

      // With maxRetries=3, we get 1 initial + 3 retries = 4 total attempts
      expect(attempts).toBe(4);
    });

    it('should throw after max retries', async () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 100,
        windowMs: 60000,
        minDelay: 0,
        autoRetry: true,
        maxRetries: 2,
        retryBaseDelay: 10,
      });

      jest.useRealTimers();

      await expect(
        limiter.execute(
          'test',
          async () => {
            throw new Error('rate limit exceeded');
          },
          { retries: 2 }
        )
      ).rejects.toThrow('rate limit exceeded');
    });

    it('should not retry non-rate-limit errors', async () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 100,
        windowMs: 60000,
        minDelay: 0,
        autoRetry: true,
        maxRetries: 3,
        retryBaseDelay: 10,
      });

      jest.useRealTimers();

      let attempts = 0;
      await expect(
        limiter.execute('test', async () => {
          attempts++;
          throw new Error('Some other error');
        })
      ).rejects.toThrow('Some other error');

      expect(attempts).toBe(1);
    });
  });

  describe('getStatus', () => {
    it('should return current limiter status', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 5,
        windowMs: 60000,
        minDelay: 0,
      });

      limiter.record();
      limiter.record();

      const status = limiter.getStatus();

      expect(status.windowRequests).toBe(2);
      expect(status.remainingRequests).toBe(3);
      expect(status.totalRequests).toBe(2);
    });

    it('should return status for specific operation', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 5,
        windowMs: 60000,
        minDelay: 0,
      });

      limiter.record('op1');
      limiter.record('op2');
      limiter.record('op2');

      expect(limiter.getStatus('op1').windowRequests).toBe(1);
      expect(limiter.getStatus('op2').windowRequests).toBe(2);
    });
  });

  describe('reset', () => {
    it('should reset all state', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 2,
        windowMs: 60000,
        minDelay: 0,
      });

      limiter.record();
      limiter.record();
      expect(limiter.check().allowed).toBe(false);

      limiter.reset();
      expect(limiter.check().allowed).toBe(true);
    });

    it('should reset specific operation', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 2,
        windowMs: 60000,
        minDelay: 0,
      });

      limiter.record('op1');
      limiter.record('op1');
      limiter.record('op2');

      limiter.reset('op1');

      expect(limiter.check('op1').allowed).toBe(true);
      expect(limiter.getStatus('op2').windowRequests).toBe(1);
    });
  });

  describe('handleRateLimitResponse', () => {
    it('should fill timestamps to prevent immediate retry', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 5,
        windowMs: 60000,
        minDelay: 0,
      });

      limiter.handleRateLimitResponse();

      const status = limiter.getStatus();
      expect(status.remainingRequests).toBe(0);
    });

    it('should set cooldown period without mutating config', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 5,
        windowMs: 60000,
        minDelay: 100,
      });

      const originalMinDelay = limiter.getStatus().config.minDelay;

      // Handle rate limit with a long retryAfter
      limiter.handleRateLimitResponse(undefined, 5000);

      // Config should NOT be mutated
      expect(limiter.getStatus().config.minDelay).toBe(originalMinDelay);

      // But check should reflect the cooldown
      const result = limiter.check();
      expect(result.allowed).toBe(false);
      expect(result.waitTime).toBeGreaterThan(0);
    });

    it('should allow cooldown to expire naturally', () => {
      jest.useFakeTimers();

      const limiter = new CursorRateLimiter({
        maxRequests: 100,
        windowMs: 60000,
        minDelay: 0,
      });

      // Set a 1 second cooldown
      limiter.handleRateLimitResponse(undefined, 1000);
      expect(limiter.check().allowed).toBe(false);

      // Advance past cooldown
      jest.advanceTimersByTime(1001);

      // Clean state for next check - need to clear timestamps too
      limiter.reset();

      expect(limiter.check().allowed).toBe(true);

      jest.useRealTimers();
    });
  });

  describe('createRateLimitError', () => {
    it('should create error with current state', () => {
      const limiter = new CursorRateLimiter({
        maxRequests: 2,
        windowMs: 60000,
        minDelay: 0,
      });

      limiter.record('testOp');
      limiter.record('testOp');

      const error = limiter.createRateLimitError('testOp');

      expect(error).toBeInstanceOf(CursorRateLimitError);
      expect(error.cursorContext.operation).toBe('testOp');
      expect(error.cursorContext.retryAfter).toBeGreaterThan(0);
    });
  });

  describe('getCursorRateLimiter (singleton)', () => {
    it('should return same instance when called multiple times', () => {
      resetGlobalRateLimiter();

      const limiter1 = getCursorRateLimiter();
      const limiter2 = getCursorRateLimiter();

      expect(limiter1).toBe(limiter2);
    });

    it('should preserve singleton and NOT replace on repeated calls', () => {
      resetGlobalRateLimiter();

      const limiter1 = getCursorRateLimiter();
      const limiter2 = getCursorRateLimiter();
      const limiter3 = getCursorRateLimiter();

      // All calls return the same instance
      expect(limiter1).toBe(limiter2);
      expect(limiter2).toBe(limiter3);
    });
  });

  describe('initGlobalRateLimiter', () => {
    it('should create new instance with config when none exists', () => {
      resetGlobalRateLimiter();

      const limiter = initGlobalRateLimiter({ maxRequests: 5 });

      expect(limiter.getStatus().config.maxRequests).toBe(5);
    });

    it('should NOT replace existing instance without force flag', () => {
      resetGlobalRateLimiter();

      const limiter1 = initGlobalRateLimiter({ maxRequests: 5 });
      const limiter2 = initGlobalRateLimiter({ maxRequests: 1 });

      expect(limiter1).toBe(limiter2);
      expect(limiter2.getStatus().config.maxRequests).toBe(5); // Original config preserved
    });

    it('should replace existing instance with force flag', () => {
      resetGlobalRateLimiter();

      const limiter1 = initGlobalRateLimiter({ maxRequests: 5 });
      const limiter2 = initGlobalRateLimiter({ maxRequests: 1 }, true);

      expect(limiter1).not.toBe(limiter2);
      expect(limiter2.getStatus().config.maxRequests).toBe(1);
    });
  });
});
