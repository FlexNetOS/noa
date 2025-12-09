#!/usr/bin/env python3
"""
Spec Locking Mechanism for Spec-Kit (SK005)

Implements file-based locking for write coordination when multiple
providers need to modify specs.

Constitutional reference: §3.13, FR-037
"""

import json
import os
import time
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import datetime, timezone, timedelta
from pathlib import Path
from typing import Any, Dict, Generator, Optional
from uuid import uuid4


@dataclass
class LockInfo:
    """Information about an active lock."""
    lock_id: str
    provider_id: str
    spec_path: str
    acquired_at: datetime
    expires_at: datetime
    lock_type: str  # "read" | "write" | "exclusive"

    @property
    def is_expired(self) -> bool:
        return datetime.now(timezone.utc) > self.expires_at

    def to_dict(self) -> Dict[str, Any]:
        return {
            "lockId": self.lock_id,
            "providerId": self.provider_id,
            "specPath": self.spec_path,
            "acquiredAt": self.acquired_at.isoformat(),
            "expiresAt": self.expires_at.isoformat(),
            "lockType": self.lock_type,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "LockInfo":
        return cls(
            lock_id=data["lockId"],
            provider_id=data["providerId"],
            spec_path=data["specPath"],
            acquired_at=datetime.fromisoformat(data["acquiredAt"]),
            expires_at=datetime.fromisoformat(data["expiresAt"]),
            lock_type=data.get("lockType", "write"),
        )


@dataclass
class LockResult:
    """Result of a lock operation."""
    success: bool
    lock_info: Optional[LockInfo]
    message: str
    waited_ms: float = 0


class SpecLockManager:
    """
    Manages file-based locks for spec write coordination.

    Features:
    - Exclusive write locks prevent concurrent modifications
    - Read locks allow parallel read access
    - Automatic lock expiration with configurable timeout
    - Dead lock detection and cleanup
    """

    DEFAULT_LOCK_TIMEOUT_MS = 30000  # 30 seconds
    DEFAULT_MAX_WAIT_MS = 60000      # 60 seconds
    DEFAULT_RETRY_ATTEMPTS = 3
    LOCK_POLL_INTERVAL_MS = 100      # Poll every 100ms

    def __init__(self, noa_root: Optional[Path] = None):
        """Initialize the lock manager."""
        self.noa_root = noa_root or self._detect_noa_root()
        self.locks_dir = self.noa_root / "ai" / "shared" / "resources" / "locks"
        self.locks_dir.mkdir(parents=True, exist_ok=True)

        # Load locking policy from spec-distribution.json
        self._load_policy()

    def _detect_noa_root(self) -> Path:
        """Detect NOA_ROOT from environment."""
        import os
        return Path(os.environ.get("NOA_ROOT", Path.cwd()))

    def _load_policy(self) -> None:
        """Load locking policy from spec-distribution.json."""
        spec_dist_path = self.noa_root / "ai" / "shared" / "resources" / "spec-distribution.json"

        if spec_dist_path.exists():
            with open(spec_dist_path) as f:
                data = json.load(f)
                policy = data.get("lockingPolicy", {})
                self.lock_timeout_ms = policy.get("lockTimeout", self.DEFAULT_LOCK_TIMEOUT_MS)
                self.max_wait_ms = policy.get("maxWaitTime", self.DEFAULT_MAX_WAIT_MS)
                self.retry_attempts = policy.get("retryAttempts", self.DEFAULT_RETRY_ATTEMPTS)
        else:
            self.lock_timeout_ms = self.DEFAULT_LOCK_TIMEOUT_MS
            self.max_wait_ms = self.DEFAULT_MAX_WAIT_MS
            self.retry_attempts = self.DEFAULT_RETRY_ATTEMPTS

    def _get_lock_path(self, spec_path: str) -> Path:
        """Get the lock file path for a spec."""
        # Sanitize spec path for use as filename
        safe_name = spec_path.replace("/", "_").replace("\\", "_")
        return self.locks_dir / f"{safe_name}.lock"

    def _read_lock_file(self, lock_path: Path) -> Optional[LockInfo]:
        """Read lock info from a lock file."""
        if not lock_path.exists():
            return None

        try:
            with open(lock_path) as f:
                data = json.load(f)
                return LockInfo.from_dict(data)
        except (json.JSONDecodeError, KeyError, OSError):
            return None

    def _write_lock_file(self, lock_path: Path, lock_info: LockInfo) -> bool:
        """Write lock info to a lock file atomically."""
        temp_path = lock_path.with_suffix(".lock.tmp")

        try:
            with open(temp_path, "w") as f:
                json.dump(lock_info.to_dict(), f, indent=2)

            # Atomic rename
            temp_path.rename(lock_path)
            return True
        except OSError:
            if temp_path.exists():
                temp_path.unlink()
            return False

    def _remove_lock_file(self, lock_path: Path) -> bool:
        """Remove a lock file."""
        try:
            if lock_path.exists():
                lock_path.unlink()
            return True
        except OSError:
            return False

    def acquire_lock(
        self,
        spec_path: str,
        provider_id: str,
        lock_type: str = "write",
        timeout_ms: Optional[int] = None,
        wait: bool = True,
    ) -> LockResult:
        """
        Acquire a lock on a spec.

        Args:
            spec_path: Path to the spec to lock
            provider_id: ID of the provider requesting the lock
            lock_type: "read" | "write" | "exclusive"
            timeout_ms: Lock timeout in milliseconds (None = use default)
            wait: Whether to wait for lock if unavailable

        Returns:
            LockResult with success status and lock info
        """
        lock_path = self._get_lock_path(spec_path)
        timeout_ms = timeout_ms or self.lock_timeout_ms

        start_time = time.time()
        attempts = 0

        while True:
            # Check for existing lock
            existing_lock = self._read_lock_file(lock_path)

            if existing_lock:
                # Check if lock is expired
                if existing_lock.is_expired:
                    # Remove expired lock
                    self._remove_lock_file(lock_path)
                    existing_lock = None
                elif lock_type == "read" and existing_lock.lock_type == "read":
                    # Allow multiple read locks
                    pass  # Will create new lock below
                elif existing_lock.provider_id == provider_id:
                    # Same provider - extend lock
                    pass  # Will update lock below
                else:
                    # Lock held by another provider
                    if not wait:
                        return LockResult(
                            success=False,
                            lock_info=None,
                            message=f"Lock held by {existing_lock.provider_id}",
                            waited_ms=0,
                        )

                    # Wait and retry
                    waited_ms = (time.time() - start_time) * 1000
                    if waited_ms > self.max_wait_ms:
                        return LockResult(
                            success=False,
                            lock_info=None,
                            message=f"Timeout waiting for lock (waited {waited_ms:.0f}ms)",
                            waited_ms=waited_ms,
                        )

                    attempts += 1
                    if attempts > self.retry_attempts * 10:  # More retries for polling
                        return LockResult(
                            success=False,
                            lock_info=None,
                            message=f"Max retry attempts exceeded",
                            waited_ms=waited_ms,
                        )

                    time.sleep(self.LOCK_POLL_INTERVAL_MS / 1000)
                    continue

            # Create new lock
            now = datetime.now(timezone.utc)
            lock_info = LockInfo(
                lock_id=str(uuid4()),
                provider_id=provider_id,
                spec_path=spec_path,
                acquired_at=now,
                expires_at=now + timedelta(milliseconds=timeout_ms),
                lock_type=lock_type,
            )

            if self._write_lock_file(lock_path, lock_info):
                waited_ms = (time.time() - start_time) * 1000
                return LockResult(
                    success=True,
                    lock_info=lock_info,
                    message="Lock acquired",
                    waited_ms=waited_ms,
                )

            # Write failed, retry
            attempts += 1
            if attempts > self.retry_attempts:
                waited_ms = (time.time() - start_time) * 1000
                return LockResult(
                    success=False,
                    lock_info=None,
                    message="Failed to write lock file",
                    waited_ms=waited_ms,
                )

            time.sleep(self.LOCK_POLL_INTERVAL_MS / 1000)

    def release_lock(
        self,
        spec_path: str,
        provider_id: str,
        lock_id: Optional[str] = None,
    ) -> bool:
        """
        Release a lock on a spec.

        Args:
            spec_path: Path to the spec
            provider_id: ID of the provider releasing the lock
            lock_id: Optional lock ID for verification

        Returns:
            True if lock was released, False otherwise
        """
        lock_path = self._get_lock_path(spec_path)
        existing_lock = self._read_lock_file(lock_path)

        if not existing_lock:
            return True  # No lock to release

        # Verify ownership
        if existing_lock.provider_id != provider_id:
            return False  # Not owner

        if lock_id and existing_lock.lock_id != lock_id:
            return False  # Lock ID mismatch

        return self._remove_lock_file(lock_path)

    def get_lock_status(self, spec_path: str) -> Optional[LockInfo]:
        """Get the current lock status for a spec."""
        lock_path = self._get_lock_path(spec_path)
        lock_info = self._read_lock_file(lock_path)

        if lock_info and lock_info.is_expired:
            # Clean up expired lock
            self._remove_lock_file(lock_path)
            return None

        return lock_info

    def is_locked(self, spec_path: str) -> bool:
        """Check if a spec is currently locked."""
        return self.get_lock_status(spec_path) is not None

    def extend_lock(
        self,
        spec_path: str,
        provider_id: str,
        additional_ms: Optional[int] = None,
    ) -> LockResult:
        """
        Extend an existing lock's timeout.

        Args:
            spec_path: Path to the spec
            provider_id: ID of the provider holding the lock
            additional_ms: Additional time in milliseconds

        Returns:
            LockResult with updated lock info
        """
        lock_path = self._get_lock_path(spec_path)
        existing_lock = self._read_lock_file(lock_path)

        if not existing_lock:
            return LockResult(
                success=False,
                lock_info=None,
                message="No lock to extend",
            )

        if existing_lock.provider_id != provider_id:
            return LockResult(
                success=False,
                lock_info=None,
                message="Not lock owner",
            )

        additional_ms = additional_ms or self.lock_timeout_ms
        new_expiry = datetime.now(timezone.utc) + timedelta(milliseconds=additional_ms)

        extended_lock = LockInfo(
            lock_id=existing_lock.lock_id,
            provider_id=existing_lock.provider_id,
            spec_path=existing_lock.spec_path,
            acquired_at=existing_lock.acquired_at,
            expires_at=new_expiry,
            lock_type=existing_lock.lock_type,
        )

        if self._write_lock_file(lock_path, extended_lock):
            return LockResult(
                success=True,
                lock_info=extended_lock,
                message="Lock extended",
            )

        return LockResult(
            success=False,
            lock_info=None,
            message="Failed to extend lock",
        )

    def cleanup_expired_locks(self) -> int:
        """Remove all expired locks."""
        removed = 0

        for lock_file in self.locks_dir.glob("*.lock"):
            lock_info = self._read_lock_file(lock_file)
            if lock_info and lock_info.is_expired:
                if self._remove_lock_file(lock_file):
                    removed += 1

        return removed

    def get_all_locks(self) -> list[LockInfo]:
        """Get all active locks."""
        locks = []

        for lock_file in self.locks_dir.glob("*.lock"):
            lock_info = self._read_lock_file(lock_file)
            if lock_info and not lock_info.is_expired:
                locks.append(lock_info)

        return locks

    @contextmanager
    def lock(
        self,
        spec_path: str,
        provider_id: str,
        lock_type: str = "write",
        timeout_ms: Optional[int] = None,
    ) -> Generator[LockInfo, None, None]:
        """
        Context manager for acquiring and releasing locks.

        Usage:
            with lock_manager.lock("specs/001", "claude-code") as lock_info:
                # Do work with exclusive access
                pass
            # Lock automatically released

        Args:
            spec_path: Path to the spec
            provider_id: Provider ID
            lock_type: Lock type
            timeout_ms: Lock timeout

        Yields:
            LockInfo for the acquired lock

        Raises:
            RuntimeError: If lock cannot be acquired
        """
        result = self.acquire_lock(spec_path, provider_id, lock_type, timeout_ms)

        if not result.success or not result.lock_info:
            raise RuntimeError(f"Failed to acquire lock: {result.message}")

        try:
            yield result.lock_info
        finally:
            self.release_lock(spec_path, provider_id, result.lock_info.lock_id)


# Convenience functions for direct usage
def acquire_spec_lock(
    spec_path: str,
    provider_id: str,
    lock_type: str = "write",
    noa_root: Optional[Path] = None,
) -> LockResult:
    """
    Acquire a lock on a spec.

    This is the primary entry point for SK005 - Spec Locking.

    Args:
        spec_path: Path to the spec to lock
        provider_id: ID of the provider requesting the lock
        lock_type: "read" | "write" | "exclusive"
        noa_root: Optional NOA root override

    Returns:
        LockResult with success status and lock info
    """
    manager = SpecLockManager(noa_root)
    return manager.acquire_lock(spec_path, provider_id, lock_type)


def release_spec_lock(
    spec_path: str,
    provider_id: str,
    noa_root: Optional[Path] = None,
) -> bool:
    """Release a lock on a spec."""
    manager = SpecLockManager(noa_root)
    return manager.release_lock(spec_path, provider_id)


@contextmanager
def spec_lock(
    spec_path: str,
    provider_id: str,
    lock_type: str = "write",
    noa_root: Optional[Path] = None,
) -> Generator[LockInfo, None, None]:
    """
    Context manager for spec locking.

    Usage:
        with spec_lock("specs/001", "claude-code") as lock:
            # Work with exclusive access
            pass
    """
    manager = SpecLockManager(noa_root)
    with manager.lock(spec_path, provider_id, lock_type) as lock_info:
        yield lock_info

