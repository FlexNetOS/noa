package throttle

import (
	"sync"
	"time"
)

// RateLimiter provides lightweight P2P throttling based on peer capacity.
type RateLimiter struct {
	mu        sync.Mutex
	capacity  int
	interval  time.Duration
	lastReset time.Time
	tokens    int
}

// New creates a new rate limiter.
func New(capacity int, interval time.Duration) *RateLimiter {
	return &RateLimiter{
		capacity:  capacity,
		interval:  interval,
		lastReset: time.Now(),
		tokens:    capacity,
	}
}

// Allow returns true if a token is available, false otherwise.
func (r *RateLimiter) Allow() bool {
	r.mu.Lock()
	defer r.mu.Unlock()

	if time.Since(r.lastReset) >= r.interval {
		r.tokens = r.capacity
		r.lastReset = time.Now()
	}

	if r.tokens > 0 {
		r.tokens--
		return true
	}
	return false
}
