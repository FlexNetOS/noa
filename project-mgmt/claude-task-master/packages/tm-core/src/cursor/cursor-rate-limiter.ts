/**
 * @fileoverview Rate limiter specifically designed for Cursor CLI operations
 * Implements token bucket algorithm with sliding window for precise rate control
 */

import { CursorRateLimitError } from '../common/errors/cursor-errors.js';

/**
 * Configuration options for the Cursor rate limiter
 */
export interface CursorRateLimiterConfig {
  /** Maximum number of requests allowed in the window */
  maxRequests: number;
  /** Time window in milliseconds */
  windowMs: number;
  /** Minimum delay between consecutive requests (ms) */
  minDelay: number;
  /** Enable automatic retry with exponential backoff */
  autoRetry: boolean;
  /** Maximum number of retry attempts */
  maxRetries: number;
  /** Base delay for exponential backoff (ms) */
  retryBaseDelay: number;
}

/**
 * Default configuration for Cursor CLI rate limiting
 * Conservative defaults to avoid hitting Cursor's limits
 */
export const DEFAULT_CURSOR_RATE_LIMIT_CONFIG: CursorRateLimiterConfig = {
  maxRequests: 10,      // 10 requests
  windowMs: 60000,      // per minute
  minDelay: 500,        // 500ms between requests
  autoRetry: true,
  maxRetries: 3,
  retryBaseDelay: 1000, // 1 second base delay
};

/**
 * Rate limiter state for tracking requests
 */
interface RateLimiterState {
  timestamps: number[];
  lastRequest: number;
  /** Temporary cooldown until timestamp (ms since epoch) - resets automatically */
  cooldownUntil: number;
}

/**
 * Result of a rate limit check
 */
export interface RateLimitCheckResult {
  allowed: boolean;
  remainingRequests: number;
  resetTime: number;
  waitTime: number;
}

/**
 * Cursor CLI Rate Limiter
 *
 * Implements a sliding window rate limiter with the following features:
 * - Token bucket algorithm for burst handling
 * - Minimum delay enforcement between requests
 * - Per-operation rate limiting
 * - Automatic retry with exponential backoff
 *
 * @example
 * ```typescript
 * const limiter = new CursorRateLimiter();
 *
 * // Check if request is allowed
 * const result = limiter.check('openFile');
 * if (!result.allowed) {
 *   console.log(`Rate limited. Wait ${result.waitTime}ms`);
 * }
 *
 * // Or use with automatic waiting
 * await limiter.acquire('openFile');
 * // ... execute cursor command
 * ```
 */
export class CursorRateLimiter {
  private readonly config: Readonly<CursorRateLimiterConfig>;
  private state: Map<string, RateLimiterState> = new Map();
  private globalState: RateLimiterState = { timestamps: [], lastRequest: 0, cooldownUntil: 0 };

  constructor(config: Partial<CursorRateLimiterConfig> = {}) {
    this.config = Object.freeze({ ...DEFAULT_CURSOR_RATE_LIMIT_CONFIG, ...config });
  }

  /**
   * Check if a request is allowed without consuming a token
   */
  public check(operation?: string): RateLimitCheckResult {
    const now = Date.now();
    const state = operation ? this.getOperationState(operation) : this.globalState;

    // Clean up old timestamps
    const windowStart = now - this.config.windowMs;
    const validTimestamps = state.timestamps.filter(t => t > windowStart);

    // Calculate remaining requests
    const remainingRequests = Math.max(0, this.config.maxRequests - validTimestamps.length);

    // Calculate wait time
    let waitTime = 0;

    // Check if we're in a cooldown period (from rate limit response)
    if (state.cooldownUntil > now) {
      waitTime = Math.max(waitTime, state.cooldownUntil - now);
    }

    // Check minimum delay since last request
    const timeSinceLastRequest = now - state.lastRequest;
    if (timeSinceLastRequest < this.config.minDelay) {
      waitTime = Math.max(waitTime, this.config.minDelay - timeSinceLastRequest);
    }

    // Check if we've hit the rate limit
    if (validTimestamps.length >= this.config.maxRequests) {
      // Calculate when the oldest request will fall outside the window
      const oldestTimestamp = Math.min(...validTimestamps);
      const resetTime = oldestTimestamp + this.config.windowMs;
      waitTime = Math.max(waitTime, resetTime - now);
    }

    const resetTime = validTimestamps.length > 0
      ? Math.min(...validTimestamps) + this.config.windowMs
      : now + this.config.windowMs;

    return {
      allowed: waitTime === 0,
      remainingRequests,
      resetTime,
      waitTime,
    };
  }

  /**
   * Record a request and update rate limiting state
   */
  public record(operation?: string): void {
    const now = Date.now();

    // Update global state
    this.updateState(this.globalState, now);

    // Update operation-specific state if provided
    if (operation) {
      const state = this.getOperationState(operation);
      this.updateState(state, now);
    }
  }

  /**
   * Acquire permission to make a request, waiting if necessary
   * Returns the actual wait time that was applied
   */
  public async acquire(operation?: string): Promise<number> {
    const result = this.check(operation);

    if (!result.allowed) {
      await this.sleep(result.waitTime);
    }

    this.record(operation);
    return result.waitTime;
  }

  /**
   * Execute a function with rate limiting and optional retries
   * @param operation - Operation name for rate limiting
   * @param fn - Function to execute
   * @param options.retries - Maximum number of retry attempts (not including initial attempt)
   */
  public async execute<T>(
    operation: string,
    fn: () => Promise<T>,
    options: { retries?: number } = {}
  ): Promise<T> {
    const maxRetries = options.retries ?? (this.config.autoRetry ? this.config.maxRetries : 0);

    let lastError: Error | undefined;

    // Total attempts = 1 (initial) + maxRetries
    for (let attempt = 0; attempt <= maxRetries; attempt++) {
      // Acquire rate limit token
      await this.acquire(operation);

      try {
        return await fn();
      } catch (error) {
        lastError = error instanceof Error ? error : new Error(String(error));

        // Check if this is a rate limit error from Cursor
        const isRateLimit = this.isRateLimitError(lastError);

        // Only retry if it's a rate limit error and we have retries remaining
        const hasRetriesRemaining = attempt < maxRetries;
        if (isRateLimit && hasRetriesRemaining) {
          const backoffDelay = this.calculateBackoff(attempt + 1);

          // Mark this as a rate limited response
          this.handleRateLimitResponse(operation);

          await this.sleep(backoffDelay);
          continue;
        }

        // For non-rate-limit errors or exhausted retries, throw
        throw lastError;
      }
    }

    // This should never be reached, but TypeScript needs it
    throw lastError || new Error('Rate limit exceeded');
  }

  /**
   * Get current rate limit status
   */
  public getStatus(operation?: string): {
    totalRequests: number;
    windowRequests: number;
    remainingRequests: number;
    resetTime: number;
    config: CursorRateLimiterConfig;
  } {
    const now = Date.now();
    const state = operation ? this.getOperationState(operation) : this.globalState;
    const windowStart = now - this.config.windowMs;
    const validTimestamps = state.timestamps.filter(t => t > windowStart);

    return {
      totalRequests: state.timestamps.length,
      windowRequests: validTimestamps.length,
      remainingRequests: Math.max(0, this.config.maxRequests - validTimestamps.length),
      resetTime: validTimestamps.length > 0
        ? Math.min(...validTimestamps) + this.config.windowMs
        : now + this.config.windowMs,
      config: { ...this.config },
    };
  }

  /**
   * Reset rate limiter state
   */
  public reset(operation?: string): void {
    if (operation) {
      this.state.delete(operation);
    } else {
      this.state.clear();
      this.globalState = { timestamps: [], lastRequest: 0, cooldownUntil: 0 };
    }
  }

  /**
   * Handle a rate limit response from Cursor
   * This adjusts internal state to be more conservative without mutating config
   */
  public handleRateLimitResponse(operation?: string, retryAfter?: number): void {
    const now = Date.now();
    const state = operation ? this.getOperationState(operation) : this.globalState;

    // Add synthetic timestamps to prevent immediate retries
    const fillCount = Math.max(0, this.config.maxRequests - state.timestamps.length);
    for (let i = 0; i < fillCount; i++) {
      state.timestamps.push(now);
    }

    // Update last request time
    state.lastRequest = now;

    // If we have a retryAfter hint, set a temporary cooldown period
    // This does NOT mutate config - the cooldown naturally expires
    if (retryAfter && retryAfter > 0) {
      const cooldownDuration = Math.min(retryAfter, 30000); // Cap at 30 seconds
      state.cooldownUntil = now + cooldownDuration;
    }
  }

  /**
   * Create a rate limit error with current state information
   */
  public createRateLimitError(operation?: string): CursorRateLimitError {
    const result = this.check(operation);
    return new CursorRateLimitError(
      `Rate limit exceeded for${operation ? ` ${operation}` : ''} operation`,
      result.waitTime,
      {
        operation,
        retryAfter: result.waitTime,
        metadata: {
          remainingRequests: result.remainingRequests,
          resetTime: new Date(result.resetTime).toISOString(),
        },
      }
    );
  }

  // Private helpers

  private getOperationState(operation: string): RateLimiterState {
    let state = this.state.get(operation);
    if (!state) {
      state = { timestamps: [], lastRequest: 0, cooldownUntil: 0 };
      this.state.set(operation, state);
    }
    return state;
  }

  private updateState(state: RateLimiterState, timestamp: number): void {
    // Clean up old timestamps
    const windowStart = timestamp - this.config.windowMs;
    state.timestamps = state.timestamps.filter(t => t > windowStart);

    // Add new timestamp
    state.timestamps.push(timestamp);
    state.lastRequest = timestamp;
  }

  private calculateBackoff(retryCount: number): number {
    // Exponential backoff with jitter
    const exponentialDelay = this.config.retryBaseDelay * Math.pow(2, retryCount - 1);
    const jitter = Math.random() * 0.3 * exponentialDelay; // 30% jitter
    return Math.min(exponentialDelay + jitter, 30000); // Cap at 30 seconds
  }

  private isRateLimitError(error: Error): boolean {
    const message = error.message.toLowerCase();
    return (
      error instanceof CursorRateLimitError ||
      message.includes('rate limit') ||
      message.includes('too many requests') ||
      message.includes('429') ||
      message.includes('quota exceeded')
    );
  }

  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

/**
 * Singleton instance for global rate limiting
 */
let globalRateLimiter: CursorRateLimiter | null = null;

/**
 * Get the global Cursor rate limiter instance
 * Creates a new instance with default config if none exists
 *
 * Note: This function preserves the singleton - passing config does NOT replace
 * the existing instance. Use `resetGlobalRateLimiter()` first if you need to
 * reconfigure the global instance.
 */
export function getCursorRateLimiter(): CursorRateLimiter {
  if (!globalRateLimiter) {
    globalRateLimiter = new CursorRateLimiter();
  }
  return globalRateLimiter;
}

/**
 * Initialize or replace the global rate limiter with custom config
 * Use this only during application initialization
 *
 * @param config - Custom configuration for the rate limiter
 * @param force - If true, replaces existing instance; if false, only creates if none exists
 */
export function initGlobalRateLimiter(
  config: Partial<CursorRateLimiterConfig>,
  force = false
): CursorRateLimiter {
  if (!globalRateLimiter || force) {
    globalRateLimiter = new CursorRateLimiter(config);
  }
  return globalRateLimiter;
}

/**
 * Reset the global rate limiter
 */
export function resetGlobalRateLimiter(): void {
  globalRateLimiter = null;
}
