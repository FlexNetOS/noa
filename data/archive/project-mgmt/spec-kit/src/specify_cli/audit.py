#!/usr/bin/env python3
"""
Audit Logging for Spec-Kit (SK006)

Implements audit logging for provider spec access, tracking who
accessed what and when.

Constitutional reference: §3.5, §3.13, FR-037
"""

import json
import os
from dataclasses import dataclass
from datetime import datetime, timezone, timedelta
from enum import Enum
from pathlib import Path
from typing import Any, Dict, List, Optional
from uuid import uuid4


class AuditEventType(str, Enum):
    """Types of audit events."""
    PROVIDER_CONNECT = "provider_connect"
    PROVIDER_DISCONNECT = "provider_disconnect"
    SPEC_ACCESS = "spec_access"
    SPEC_BROADCAST = "spec_broadcast"
    LOCK_ACQUIRE = "lock_acquire"
    LOCK_RELEASE = "lock_release"
    LOCK_TIMEOUT = "lock_timeout"
    TASK_ASSIGNED = "task_assigned"
    TASK_COMPLETED = "task_completed"
    TASK_FAILED = "task_failed"
    CONTEXT_SHARED = "context_shared"
    CONFIG_CHANGE = "config_change"
    ERROR = "error"


class AuditLogLevel(str, Enum):
    """Log levels for audit events."""
    DEBUG = "debug"
    INFO = "info"
    WARN = "warn"
    ERROR = "error"


@dataclass
class AuditEvent:
    """Represents a single audit event."""
    event_id: str
    timestamp: datetime
    event_type: AuditEventType
    provider_id: Optional[str]
    spec_path: Optional[str]
    level: AuditLogLevel
    message: str
    details: Dict[str, Any]
    session_id: Optional[str]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "eventId": self.event_id,
            "timestamp": self.timestamp.isoformat(),
            "eventType": self.event_type.value,
            "providerId": self.provider_id,
            "specPath": self.spec_path,
            "level": self.level.value,
            "message": self.message,
            "details": self.details,
            "sessionId": self.session_id,
        }

    def to_log_line(self) -> str:
        """Format as a single log line."""
        parts = [
            self.timestamp.strftime("%Y-%m-%d %H:%M:%S.%f")[:-3],
            f"[{self.level.value.upper()}]",
            f"[{self.event_type.value}]",
        ]

        if self.provider_id:
            parts.append(f"provider={self.provider_id}")

        if self.spec_path:
            parts.append(f"spec={self.spec_path}")

        parts.append(self.message)

        if self.details:
            # Compact JSON for details
            details_str = json.dumps(self.details, separators=(",", ":"))
            parts.append(f"details={details_str}")

        return " ".join(parts)

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "AuditEvent":
        return cls(
            event_id=data["eventId"],
            timestamp=datetime.fromisoformat(data["timestamp"]),
            event_type=AuditEventType(data["eventType"]),
            provider_id=data.get("providerId"),
            spec_path=data.get("specPath"),
            level=AuditLogLevel(data.get("level", "info")),
            message=data["message"],
            details=data.get("details", {}),
            session_id=data.get("sessionId"),
        )


class AuditLogger:
    """
    Manages audit logging for spec-kit operations.

    Features:
    - Structured logging of all provider actions
    - File-based log storage with rotation
    - Query interface for audit analysis
    - Configurable log level and retention
    """

    DEFAULT_LOG_PATH = "ai/shared/resources/provider-access.log"
    DEFAULT_RETENTION_DAYS = 30
    DEFAULT_LOG_LEVEL = AuditLogLevel.INFO
    MAX_LOG_SIZE_MB = 10

    def __init__(self, noa_root: Optional[Path] = None):
        """Initialize the audit logger."""
        self.noa_root = noa_root or self._detect_noa_root()
        self._load_config()

        # Ensure log directory exists
        self.log_path.parent.mkdir(parents=True, exist_ok=True)

    def _detect_noa_root(self) -> Path:
        """Detect NOA_ROOT from environment."""
        return Path(os.environ.get("NOA_ROOT", Path.cwd()))

    def _load_config(self) -> None:
        """Load audit configuration from spec-distribution.json."""
        spec_dist_path = self.noa_root / "ai" / "shared" / "resources" / "spec-distribution.json"

        if spec_dist_path.exists():
            with open(spec_dist_path) as f:
                data = json.load(f)
                audit_config = data.get("auditConfig", {})

                log_path_str = audit_config.get("logPath", self.DEFAULT_LOG_PATH)
                self.log_path = self.noa_root / log_path_str
                self.retention_days = audit_config.get("retentionDays", self.DEFAULT_RETENTION_DAYS)
                self.enabled = audit_config.get("enabled", True)

                level_str = audit_config.get("logLevel", "info")
                try:
                    self.log_level = AuditLogLevel(level_str)
                except ValueError:
                    self.log_level = self.DEFAULT_LOG_LEVEL
        else:
            self.log_path = self.noa_root / self.DEFAULT_LOG_PATH
            self.retention_days = self.DEFAULT_RETENTION_DAYS
            self.log_level = self.DEFAULT_LOG_LEVEL
            self.enabled = True

    def _should_log(self, level: AuditLogLevel) -> bool:
        """Check if an event at the given level should be logged."""
        if not self.enabled:
            return False

        level_order = [AuditLogLevel.DEBUG, AuditLogLevel.INFO, AuditLogLevel.WARN, AuditLogLevel.ERROR]
        return level_order.index(level) >= level_order.index(self.log_level)

    def _rotate_if_needed(self) -> None:
        """Rotate log file if it exceeds max size."""
        if not self.log_path.exists():
            return

        size_mb = self.log_path.stat().st_size / (1024 * 1024)
        if size_mb < self.MAX_LOG_SIZE_MB:
            return

        # Rotate: rename current to .1, .1 to .2, etc.
        for i in range(9, 0, -1):
            old_path = self.log_path.with_suffix(f".log.{i}")
            new_path = self.log_path.with_suffix(f".log.{i + 1}")
            if old_path.exists():
                old_path.rename(new_path)

        self.log_path.rename(self.log_path.with_suffix(".log.1"))

    def log(
        self,
        event_type: AuditEventType,
        message: str,
        provider_id: Optional[str] = None,
        spec_path: Optional[str] = None,
        level: AuditLogLevel = AuditLogLevel.INFO,
        details: Optional[Dict[str, Any]] = None,
        session_id: Optional[str] = None,
    ) -> Optional[AuditEvent]:
        """
        Log an audit event.

        Args:
            event_type: Type of event
            message: Human-readable message
            provider_id: Provider involved in the event
            spec_path: Spec involved in the event
            level: Log level
            details: Additional details
            session_id: Session ID for correlation

        Returns:
            AuditEvent if logged, None if filtered out
        """
        if not self._should_log(level):
            return None

        event = AuditEvent(
            event_id=str(uuid4()),
            timestamp=datetime.now(timezone.utc),
            event_type=event_type,
            provider_id=provider_id,
            spec_path=spec_path,
            level=level,
            message=message,
            details=details or {},
            session_id=session_id,
        )

        self._write_event(event)
        return event

    def _write_event(self, event: AuditEvent) -> None:
        """Write an event to the log file."""
        self._rotate_if_needed()

        with open(self.log_path, "a") as f:
            f.write(event.to_log_line() + "\n")

    # Convenience methods for common events

    def log_provider_connect(
        self,
        provider_id: str,
        spec_path: str,
        access_mode: str,
        session_id: Optional[str] = None,
    ) -> Optional[AuditEvent]:
        """Log a provider connection event."""
        return self.log(
            event_type=AuditEventType.PROVIDER_CONNECT,
            message=f"Provider {provider_id} connected with {access_mode} access",
            provider_id=provider_id,
            spec_path=spec_path,
            details={"accessMode": access_mode},
            session_id=session_id,
        )

    def log_provider_disconnect(
        self,
        provider_id: str,
        session_id: Optional[str] = None,
    ) -> Optional[AuditEvent]:
        """Log a provider disconnection event."""
        return self.log(
            event_type=AuditEventType.PROVIDER_DISCONNECT,
            message=f"Provider {provider_id} disconnected",
            provider_id=provider_id,
            session_id=session_id,
        )

    def log_spec_access(
        self,
        provider_id: str,
        spec_path: str,
        access_type: str,
        session_id: Optional[str] = None,
    ) -> Optional[AuditEvent]:
        """Log a spec access event."""
        return self.log(
            event_type=AuditEventType.SPEC_ACCESS,
            message=f"Provider {provider_id} accessed spec ({access_type})",
            provider_id=provider_id,
            spec_path=spec_path,
            details={"accessType": access_type},
            session_id=session_id,
        )

    def log_spec_broadcast(
        self,
        spec_path: str,
        providers: List[str],
        successful: int,
        failed: int,
        session_id: Optional[str] = None,
    ) -> Optional[AuditEvent]:
        """Log a spec broadcast event."""
        return self.log(
            event_type=AuditEventType.SPEC_BROADCAST,
            message=f"Spec broadcast to {len(providers)} providers ({successful} ok, {failed} failed)",
            spec_path=spec_path,
            details={
                "providers": providers,
                "successful": successful,
                "failed": failed,
            },
            session_id=session_id,
        )

    def log_lock_acquire(
        self,
        provider_id: str,
        spec_path: str,
        lock_type: str,
        lock_id: str,
        session_id: Optional[str] = None,
    ) -> Optional[AuditEvent]:
        """Log a lock acquisition event."""
        return self.log(
            event_type=AuditEventType.LOCK_ACQUIRE,
            message=f"Provider {provider_id} acquired {lock_type} lock",
            provider_id=provider_id,
            spec_path=spec_path,
            details={"lockType": lock_type, "lockId": lock_id},
            session_id=session_id,
        )

    def log_lock_release(
        self,
        provider_id: str,
        spec_path: str,
        lock_id: str,
        session_id: Optional[str] = None,
    ) -> Optional[AuditEvent]:
        """Log a lock release event."""
        return self.log(
            event_type=AuditEventType.LOCK_RELEASE,
            message=f"Provider {provider_id} released lock",
            provider_id=provider_id,
            spec_path=spec_path,
            details={"lockId": lock_id},
            session_id=session_id,
        )

    def log_error(
        self,
        message: str,
        error: Optional[Exception] = None,
        provider_id: Optional[str] = None,
        spec_path: Optional[str] = None,
        session_id: Optional[str] = None,
    ) -> Optional[AuditEvent]:
        """Log an error event."""
        details = {}
        if error:
            details["errorType"] = type(error).__name__
            details["errorMessage"] = str(error)

        return self.log(
            event_type=AuditEventType.ERROR,
            message=message,
            provider_id=provider_id,
            spec_path=spec_path,
            level=AuditLogLevel.ERROR,
            details=details,
            session_id=session_id,
        )

    # Query interface

    def get_recent_events(
        self,
        limit: int = 100,
        event_type: Optional[AuditEventType] = None,
        provider_id: Optional[str] = None,
    ) -> List[AuditEvent]:
        """
        Get recent audit events.

        Args:
            limit: Maximum number of events to return
            event_type: Filter by event type
            provider_id: Filter by provider

        Returns:
            List of matching AuditEvents (newest first)
        """
        if not self.log_path.exists():
            return []

        events: List[AuditEvent] = []

        # Read log file in reverse (newest first)
        with open(self.log_path) as f:
            lines = f.readlines()

        for line in reversed(lines):
            if len(events) >= limit:
                break

            event = self._parse_log_line(line.strip())
            if not event:
                continue

            # Apply filters
            if event_type and event.event_type != event_type:
                continue
            if provider_id and event.provider_id != provider_id:
                continue

            events.append(event)

        return events

    def _parse_log_line(self, line: str) -> Optional[AuditEvent]:
        """Parse a log line back into an AuditEvent."""
        try:
            # Format: YYYY-MM-DD HH:MM:SS.mmm [LEVEL] [event_type] provider=X spec=Y message details={}
            parts = line.split(" ", 4)
            if len(parts) < 5:
                return None

            timestamp_str = f"{parts[0]} {parts[1]}"
            timestamp = datetime.strptime(timestamp_str, "%Y-%m-%d %H:%M:%S.%f")
            timestamp = timestamp.replace(tzinfo=timezone.utc)

            level_str = parts[2].strip("[]").lower()
            level = AuditLogLevel(level_str) if level_str in [l.value for l in AuditLogLevel] else AuditLogLevel.INFO

            event_type_str = parts[3].strip("[]")
            event_type = AuditEventType(event_type_str)

            remaining = parts[4]
            provider_id = None
            spec_path = None
            details = {}
            message = remaining

            # Extract provider=X
            if "provider=" in remaining:
                idx = remaining.index("provider=")
                end_idx = remaining.find(" ", idx)
                if end_idx == -1:
                    end_idx = len(remaining)
                provider_id = remaining[idx + 9:end_idx]
                remaining = remaining[:idx] + remaining[end_idx + 1:]

            # Extract spec=X
            if "spec=" in remaining:
                idx = remaining.index("spec=")
                end_idx = remaining.find(" ", idx)
                if end_idx == -1:
                    end_idx = len(remaining)
                spec_path = remaining[idx + 5:end_idx]
                remaining = remaining[:idx] + remaining[end_idx + 1:]

            # Extract details={}
            if "details=" in remaining:
                idx = remaining.index("details=")
                details_str = remaining[idx + 8:]
                try:
                    details = json.loads(details_str)
                except json.JSONDecodeError:
                    pass
                remaining = remaining[:idx]

            message = remaining.strip()

            return AuditEvent(
                event_id="",  # Not stored in log line
                timestamp=timestamp,
                event_type=event_type,
                provider_id=provider_id,
                spec_path=spec_path,
                level=level,
                message=message,
                details=details,
                session_id=None,
            )
        except (ValueError, IndexError):
            return None

    def cleanup_old_logs(self) -> int:
        """Remove log entries older than retention period."""
        if not self.log_path.exists():
            return 0

        cutoff = datetime.now(timezone.utc) - timedelta(days=self.retention_days)

        with open(self.log_path) as f:
            lines = f.readlines()

        kept_lines = []
        removed = 0

        for line in lines:
            event = self._parse_log_line(line.strip())
            if event and event.timestamp >= cutoff:
                kept_lines.append(line)
            else:
                removed += 1

        with open(self.log_path, "w") as f:
            f.writelines(kept_lines)

        return removed


# Global logger instance
_logger: Optional[AuditLogger] = None


def get_audit_logger(noa_root: Optional[Path] = None) -> AuditLogger:
    """Get or create the global audit logger."""
    global _logger
    if _logger is None:
        _logger = AuditLogger(noa_root)
    return _logger


# Convenience functions
def audit_log(
    event_type: AuditEventType,
    message: str,
    **kwargs,
) -> Optional[AuditEvent]:
    """Log an audit event using the global logger."""
    logger = get_audit_logger()
    return logger.log(event_type, message, **kwargs)

