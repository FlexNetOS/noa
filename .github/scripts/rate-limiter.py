#!/usr/bin/env python3
"""
FlexNetOS Rate Limiter & Circuit Breaker
Handles rate limiting, backoff, and circuit breaker patterns for automation
"""

import os
import json
import time
import logging
from pathlib import Path
from typing import Dict, Optional, Any
from dataclasses import dataclass, asdict
from datetime import datetime, timedelta
from enum import Enum
import threading
import hashlib

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class CircuitState(Enum):
    CLOSED = "closed"      # Normal operation
    OPEN = "open"          # Failing, reject requests
    HALF_OPEN = "half_open"  # Testing recovery


@dataclass
class RateLimitConfig:
    """Configuration for rate limiting"""
    requests_per_hour: int = 100
    requests_per_day: int = 1000
    burst_limit: int = 10
    backoff_initial_seconds: float = 1.0
    backoff_max_seconds: float = 300.0
    backoff_multiplier: float = 2.0


@dataclass
class CircuitBreakerConfig:
    """Configuration for circuit breaker"""
    failure_threshold: int = 5
    success_threshold: int = 3
    recovery_timeout_seconds: int = 300


@dataclass
class RateLimitState:
    """Current state of rate limiting for a resource"""
    resource_id: str
    requests_this_hour: int = 0
    requests_this_day: int = 0
    hour_start: str = ""
    day_start: str = ""
    current_backoff: float = 0.0
    last_request: Optional[str] = None


@dataclass
class CircuitBreakerState:
    """Current state of circuit breaker for a resource"""
    resource_id: str
    state: str = CircuitState.CLOSED.value
    failure_count: int = 0
    success_count: int = 0
    last_failure_time: Optional[str] = None
    last_state_change: Optional[str] = None


class RateLimiter:
    """
    Rate limiter with sliding window and exponential backoff
    """

    def __init__(self, state_dir: str = ".github/state/rate-limits"):
        self.state_dir = Path(state_dir)
        self.state_dir.mkdir(parents=True, exist_ok=True)
        self.configs: Dict[str, RateLimitConfig] = {}
        self.lock = threading.Lock()

    def configure(self, resource_id: str, config: RateLimitConfig):
        """Configure rate limits for a resource"""
        self.configs[resource_id] = config

    def _get_state_file(self, resource_id: str) -> Path:
        """Get state file path for a resource"""
        safe_id = hashlib.md5(resource_id.encode()).hexdigest()[:16]
        return self.state_dir / f"{safe_id}.json"

    def _load_state(self, resource_id: str) -> RateLimitState:
        """Load state from disk"""
        state_file = self._get_state_file(resource_id)
        if state_file.exists():
            try:
                with open(state_file) as f:
                    data = json.load(f)
                return RateLimitState(**data)
            except:
                pass
        return RateLimitState(resource_id=resource_id)

    def _save_state(self, state: RateLimitState):
        """Save state to disk"""
        state_file = self._get_state_file(state.resource_id)
        with open(state_file, 'w') as f:
            json.dump(asdict(state), f, indent=2)

    def _reset_if_needed(self, state: RateLimitState) -> RateLimitState:
        """Reset counters if time window has passed"""
        now = datetime.utcnow()
        current_hour = now.strftime("%Y-%m-%d-%H")
        current_day = now.strftime("%Y-%m-%d")

        if state.hour_start != current_hour:
            state.requests_this_hour = 0
            state.hour_start = current_hour

        if state.day_start != current_day:
            state.requests_this_day = 0
            state.day_start = current_day

        return state

    def check_limit(self, resource_id: str) -> tuple[bool, float]:
        """
        Check if request is allowed
        Returns: (allowed, wait_time_seconds)
        """
        with self.lock:
            config = self.configs.get(resource_id, RateLimitConfig())
            state = self._load_state(resource_id)
            state = self._reset_if_needed(state)

            # Check daily limit
            if state.requests_this_day >= config.requests_per_day:
                logger.warning(f"Daily rate limit reached for {resource_id}")
                # Calculate time until midnight
                now = datetime.utcnow()
                tomorrow = (now + timedelta(days=1)).replace(hour=0, minute=0, second=0, microsecond=0)
                wait_time = (tomorrow - now).total_seconds()
                return False, wait_time

            # Check hourly limit
            if state.requests_this_hour >= config.requests_per_hour:
                logger.warning(f"Hourly rate limit reached for {resource_id}")
                # Calculate time until next hour
                now = datetime.utcnow()
                next_hour = (now + timedelta(hours=1)).replace(minute=0, second=0, microsecond=0)
                wait_time = (next_hour - now).total_seconds()
                return False, wait_time

            # Check backoff
            if state.current_backoff > 0:
                return False, state.current_backoff

            return True, 0.0

    def record_request(self, resource_id: str, success: bool = True):
        """Record a request and update state"""
        with self.lock:
            config = self.configs.get(resource_id, RateLimitConfig())
            state = self._load_state(resource_id)
            state = self._reset_if_needed(state)

            state.requests_this_hour += 1
            state.requests_this_day += 1
            state.last_request = datetime.utcnow().isoformat()

            if success:
                # Reset backoff on success
                state.current_backoff = 0.0
            else:
                # Increase backoff on failure
                if state.current_backoff == 0:
                    state.current_backoff = config.backoff_initial_seconds
                else:
                    state.current_backoff = min(
                        state.current_backoff * config.backoff_multiplier,
                        config.backoff_max_seconds
                    )

            self._save_state(state)

    def wait_if_needed(self, resource_id: str) -> bool:
        """Wait if rate limited, return True if waited"""
        allowed, wait_time = self.check_limit(resource_id)

        if not allowed and wait_time > 0:
            logger.info(f"Rate limited for {resource_id}, waiting {wait_time:.1f}s")
            time.sleep(wait_time)
            return True

        return False

    def get_status(self, resource_id: str) -> Dict[str, Any]:
        """Get current rate limit status"""
        config = self.configs.get(resource_id, RateLimitConfig())
        state = self._load_state(resource_id)
        state = self._reset_if_needed(state)

        return {
            "resource_id": resource_id,
            "requests_this_hour": state.requests_this_hour,
            "requests_this_day": state.requests_this_day,
            "hourly_limit": config.requests_per_hour,
            "daily_limit": config.requests_per_day,
            "hourly_remaining": max(0, config.requests_per_hour - state.requests_this_hour),
            "daily_remaining": max(0, config.requests_per_day - state.requests_this_day),
            "current_backoff": state.current_backoff,
            "last_request": state.last_request
        }


class CircuitBreaker:
    """
    Circuit breaker pattern implementation
    Prevents cascading failures by stopping requests to failing services
    """

    def __init__(self, state_dir: str = ".github/state/circuit-breakers"):
        self.state_dir = Path(state_dir)
        self.state_dir.mkdir(parents=True, exist_ok=True)
        self.configs: Dict[str, CircuitBreakerConfig] = {}
        self.lock = threading.Lock()

    def configure(self, resource_id: str, config: CircuitBreakerConfig):
        """Configure circuit breaker for a resource"""
        self.configs[resource_id] = config

    def _get_state_file(self, resource_id: str) -> Path:
        """Get state file path for a resource"""
        safe_id = hashlib.md5(resource_id.encode()).hexdigest()[:16]
        return self.state_dir / f"cb_{safe_id}.json"

    def _load_state(self, resource_id: str) -> CircuitBreakerState:
        """Load state from disk"""
        state_file = self._get_state_file(resource_id)
        if state_file.exists():
            try:
                with open(state_file) as f:
                    data = json.load(f)
                return CircuitBreakerState(**data)
            except:
                pass
        return CircuitBreakerState(resource_id=resource_id)

    def _save_state(self, state: CircuitBreakerState):
        """Save state to disk"""
        state_file = self._get_state_file(state.resource_id)
        with open(state_file, 'w') as f:
            json.dump(asdict(state), f, indent=2)

    def is_open(self, resource_id: str) -> bool:
        """Check if circuit is open (failing)"""
        with self.lock:
            config = self.configs.get(resource_id, CircuitBreakerConfig())
            state = self._load_state(resource_id)

            if state.state == CircuitState.CLOSED.value:
                return False

            if state.state == CircuitState.OPEN.value:
                # Check if recovery timeout has passed
                if state.last_state_change:
                    last_change = datetime.fromisoformat(state.last_state_change)
                    elapsed = (datetime.utcnow() - last_change).total_seconds()
                    if elapsed >= config.recovery_timeout_seconds:
                        # Transition to half-open
                        state.state = CircuitState.HALF_OPEN.value
                        state.last_state_change = datetime.utcnow().isoformat()
                        self._save_state(state)
                        logger.info(f"Circuit breaker for {resource_id} transitioning to HALF_OPEN")
                        return False
                return True

            # Half-open - allow request through for testing
            return False

    def record_success(self, resource_id: str):
        """Record a successful request"""
        with self.lock:
            config = self.configs.get(resource_id, CircuitBreakerConfig())
            state = self._load_state(resource_id)

            if state.state == CircuitState.HALF_OPEN.value:
                state.success_count += 1
                if state.success_count >= config.success_threshold:
                    # Close the circuit
                    state.state = CircuitState.CLOSED.value
                    state.failure_count = 0
                    state.success_count = 0
                    state.last_state_change = datetime.utcnow().isoformat()
                    logger.info(f"Circuit breaker for {resource_id} CLOSED (recovered)")
            else:
                # Reset failure count on success in closed state
                state.failure_count = 0

            self._save_state(state)

    def record_failure(self, resource_id: str):
        """Record a failed request"""
        with self.lock:
            config = self.configs.get(resource_id, CircuitBreakerConfig())
            state = self._load_state(resource_id)

            state.failure_count += 1
            state.last_failure_time = datetime.utcnow().isoformat()

            if state.state == CircuitState.HALF_OPEN.value:
                # Immediate transition back to open
                state.state = CircuitState.OPEN.value
                state.success_count = 0
                state.last_state_change = datetime.utcnow().isoformat()
                logger.warning(f"Circuit breaker for {resource_id} OPEN (test failed)")
            elif state.state == CircuitState.CLOSED.value:
                if state.failure_count >= config.failure_threshold:
                    # Open the circuit
                    state.state = CircuitState.OPEN.value
                    state.last_state_change = datetime.utcnow().isoformat()
                    logger.warning(f"Circuit breaker for {resource_id} OPEN (threshold reached)")

            self._save_state(state)

    def get_status(self, resource_id: str) -> Dict[str, Any]:
        """Get current circuit breaker status"""
        config = self.configs.get(resource_id, CircuitBreakerConfig())
        state = self._load_state(resource_id)

        return {
            "resource_id": resource_id,
            "state": state.state,
            "failure_count": state.failure_count,
            "success_count": state.success_count,
            "failure_threshold": config.failure_threshold,
            "success_threshold": config.success_threshold,
            "recovery_timeout": config.recovery_timeout_seconds,
            "last_failure": state.last_failure_time,
            "last_state_change": state.last_state_change
        }

    def force_close(self, resource_id: str):
        """Force close the circuit (manual recovery)"""
        with self.lock:
            state = self._load_state(resource_id)
            state.state = CircuitState.CLOSED.value
            state.failure_count = 0
            state.success_count = 0
            state.last_state_change = datetime.utcnow().isoformat()
            self._save_state(state)
            logger.info(f"Circuit breaker for {resource_id} forced CLOSED")

    def force_open(self, resource_id: str):
        """Force open the circuit (manual intervention)"""
        with self.lock:
            state = self._load_state(resource_id)
            state.state = CircuitState.OPEN.value
            state.last_state_change = datetime.utcnow().isoformat()
            self._save_state(state)
            logger.warning(f"Circuit breaker for {resource_id} forced OPEN")


class AutomationGuard:
    """
    Combined rate limiter and circuit breaker for automation
    """

    def __init__(self, state_dir: str = ".github/state"):
        self.rate_limiter = RateLimiter(f"{state_dir}/rate-limits")
        self.circuit_breaker = CircuitBreaker(f"{state_dir}/circuit-breakers")

    def configure_resource(
        self,
        resource_id: str,
        rate_limit: Optional[RateLimitConfig] = None,
        circuit_breaker: Optional[CircuitBreakerConfig] = None
    ):
        """Configure both rate limiting and circuit breaker for a resource"""
        if rate_limit:
            self.rate_limiter.configure(resource_id, rate_limit)
        if circuit_breaker:
            self.circuit_breaker.configure(resource_id, circuit_breaker)

    def can_proceed(self, resource_id: str) -> tuple[bool, str]:
        """
        Check if a request can proceed
        Returns: (can_proceed, reason)
        """
        # Check circuit breaker first
        if self.circuit_breaker.is_open(resource_id):
            return False, "circuit_open"

        # Check rate limit
        allowed, wait_time = self.rate_limiter.check_limit(resource_id)
        if not allowed:
            return False, f"rate_limited:{wait_time:.1f}s"

        return True, "ok"

    def execute(self, resource_id: str, func, *args, **kwargs):
        """
        Execute a function with rate limiting and circuit breaker protection
        """
        can_proceed, reason = self.can_proceed(resource_id)

        if not can_proceed:
            if reason == "circuit_open":
                raise CircuitOpenError(f"Circuit is open for {resource_id}")
            elif reason.startswith("rate_limited"):
                wait_time = float(reason.split(":")[1].rstrip("s"))
                raise RateLimitError(f"Rate limited for {resource_id}", wait_time)

        try:
            result = func(*args, **kwargs)
            self.rate_limiter.record_request(resource_id, success=True)
            self.circuit_breaker.record_success(resource_id)
            return result
        except Exception as e:
            self.rate_limiter.record_request(resource_id, success=False)
            self.circuit_breaker.record_failure(resource_id)
            raise

    def get_status(self, resource_id: str) -> Dict[str, Any]:
        """Get combined status for a resource"""
        return {
            "rate_limit": self.rate_limiter.get_status(resource_id),
            "circuit_breaker": self.circuit_breaker.get_status(resource_id)
        }


class CircuitOpenError(Exception):
    """Raised when circuit breaker is open"""
    pass


class RateLimitError(Exception):
    """Raised when rate limit is exceeded"""
    def __init__(self, message: str, wait_time: float):
        super().__init__(message)
        self.wait_time = wait_time


# Pre-configured guards for common resources
def create_github_api_guard() -> AutomationGuard:
    """Create a guard configured for GitHub API limits"""
    guard = AutomationGuard()
    guard.configure_resource(
        "github_api",
        rate_limit=RateLimitConfig(
            requests_per_hour=5000,
            requests_per_day=50000,
            burst_limit=100,
            backoff_initial_seconds=1.0,
            backoff_max_seconds=60.0
        ),
        circuit_breaker=CircuitBreakerConfig(
            failure_threshold=10,
            success_threshold=5,
            recovery_timeout_seconds=300
        )
    )
    return guard


def create_ai_provider_guard(provider: str) -> AutomationGuard:
    """Create a guard configured for AI provider limits"""
    limits = {
        "claude": RateLimitConfig(requests_per_hour=50, requests_per_day=500),
        "chatgpt": RateLimitConfig(requests_per_hour=60, requests_per_day=1000),
        "gemini": RateLimitConfig(requests_per_hour=100, requests_per_day=1500),
        "copilot": RateLimitConfig(requests_per_hour=200, requests_per_day=5000),
    }

    guard = AutomationGuard()
    guard.configure_resource(
        f"ai_{provider}",
        rate_limit=limits.get(provider, RateLimitConfig()),
        circuit_breaker=CircuitBreakerConfig(
            failure_threshold=5,
            success_threshold=3,
            recovery_timeout_seconds=300
        )
    )
    return guard


if __name__ == "__main__":
    # Demo usage
    guard = create_github_api_guard()

    # Check if we can proceed
    can_proceed, reason = guard.can_proceed("github_api")
    print(f"Can proceed: {can_proceed}, Reason: {reason}")

    # Get status
    status = guard.get_status("github_api")
    print(f"Status: {json.dumps(status, indent=2)}")

