#!/usr/bin/env python3
"""
FlexNetOS Audit Logger
Comprehensive logging and audit trail for all automation activities

Features:
- Structured event logging
- Audit trail for compliance
- Cost tracking for AI usage
- Performance metrics
- Error tracking and analysis
"""

import os
import json
import logging
import hashlib
from pathlib import Path
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass, asdict, field
from datetime import datetime, timedelta
from enum import Enum
import threading
from collections import defaultdict

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class EventType(Enum):
    # Issue events
    ISSUE_AGGREGATED = "issue_aggregated"
    ISSUE_CREATED = "issue_created"
    ISSUE_UPDATED = "issue_updated"
    ISSUE_CLOSED = "issue_closed"

    # Resolution events
    RESOLUTION_STARTED = "resolution_started"
    RESOLUTION_ATTEMPTED = "resolution_attempted"
    RESOLUTION_SUCCEEDED = "resolution_succeeded"
    RESOLUTION_FAILED = "resolution_failed"
    RESOLUTION_SKIPPED = "resolution_skipped"

    # Fix events
    FIX_GENERATED = "fix_generated"
    FIX_APPLIED = "fix_applied"
    FIX_REJECTED = "fix_rejected"

    # PR events
    PR_CREATED = "pr_created"
    PR_UPDATED = "pr_updated"
    PR_MERGED = "pr_merged"
    PR_CLOSED = "pr_closed"

    # Merge events
    MERGE_STARTED = "merge_started"
    MERGE_COMPLETED = "merge_completed"
    MERGE_FAILED = "merge_failed"
    MERGE_CONFLICT = "merge_conflict"

    # AI events
    AI_REQUEST_SENT = "ai_request_sent"
    AI_RESPONSE_RECEIVED = "ai_response_received"
    AI_RATE_LIMITED = "ai_rate_limited"
    AI_ERROR = "ai_error"

    # System events
    WORKFLOW_STARTED = "workflow_started"
    WORKFLOW_COMPLETED = "workflow_completed"
    WORKFLOW_FAILED = "workflow_failed"
    RATE_LIMIT_HIT = "rate_limit_hit"
    CIRCUIT_OPENED = "circuit_opened"
    CIRCUIT_CLOSED = "circuit_closed"
    ERROR_OCCURRED = "error_occurred"

    # Rollback events
    ROLLBACK_TRIGGERED = "rollback_triggered"
    ROLLBACK_COMPLETED = "rollback_completed"
    ROLLBACK_FAILED = "rollback_failed"


class Severity(Enum):
    DEBUG = "debug"
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    CRITICAL = "critical"


@dataclass
class AuditEvent:
    """Represents a single audit event"""
    event_id: str
    event_type: str
    severity: str
    timestamp: str
    repository: str
    pr_number: Optional[int]
    actor: str  # 'bot', 'user', 'system'
    action: str
    details: Dict[str, Any]
    metadata: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)

    def to_json(self) -> str:
        return json.dumps(self.to_dict())


@dataclass
class CostRecord:
    """Record of AI/resource usage costs"""
    timestamp: str
    provider: str
    operation: str
    tokens_in: int = 0
    tokens_out: int = 0
    estimated_cost_usd: float = 0.0
    duration_ms: int = 0


@dataclass
class PerformanceMetric:
    """Performance metric record"""
    timestamp: str
    metric_name: str
    value: float
    unit: str
    tags: Dict[str, str] = field(default_factory=dict)


class AuditLogger:
    """
    Central audit logging system for FlexNetOS automation
    """

    def __init__(self, log_dir: str = ".github/logs", retention_days: int = 90):
        self.log_dir = Path(log_dir)
        self.log_dir.mkdir(parents=True, exist_ok=True)
        self.retention_days = retention_days
        self.lock = threading.Lock()

        # In-memory buffers for batching
        self.event_buffer: List[AuditEvent] = []
        self.cost_buffer: List[CostRecord] = []
        self.metric_buffer: List[PerformanceMetric] = []
        self.buffer_size = 100

        # Cost tracking
        self.provider_costs = {
            "claude": {"input": 0.000003, "output": 0.000015},  # per token
            "chatgpt": {"input": 0.00001, "output": 0.00003},
            "gemini": {"input": 0.00000025, "output": 0.0000005},
            "copilot": {"input": 0.0, "output": 0.0},  # included in subscription
        }

    def _generate_event_id(self) -> str:
        """Generate unique event ID"""
        timestamp = datetime.utcnow().isoformat()
        random_part = hashlib.md5(f"{timestamp}{os.urandom(4)}".encode()).hexdigest()[:8]
        return f"evt_{random_part}"

    def _get_log_file(self, date: datetime, log_type: str = "events") -> Path:
        """Get log file path for a date"""
        date_str = date.strftime("%Y-%m-%d")
        return self.log_dir / f"{log_type}_{date_str}.jsonl"

    def _append_to_log(self, log_file: Path, data: Dict[str, Any]):
        """Append a record to a log file"""
        with self.lock:
            with open(log_file, 'a') as f:
                f.write(json.dumps(data) + '\n')

    def log_event(
        self,
        event_type: EventType,
        action: str,
        repository: str,
        pr_number: Optional[int] = None,
        severity: Severity = Severity.INFO,
        details: Optional[Dict[str, Any]] = None,
        metadata: Optional[Dict[str, Any]] = None,
        actor: str = "bot"
    ) -> str:
        """Log an audit event"""
        event = AuditEvent(
            event_id=self._generate_event_id(),
            event_type=event_type.value,
            severity=severity.value,
            timestamp=datetime.utcnow().isoformat(),
            repository=repository,
            pr_number=pr_number,
            actor=actor,
            action=action,
            details=details or {},
            metadata=metadata or {}
        )

        # Log to console
        log_msg = f"[{event.event_type}] {repository} - {action}"
        if severity == Severity.ERROR:
            logger.error(log_msg)
        elif severity == Severity.WARNING:
            logger.warning(log_msg)
        else:
            logger.info(log_msg)

        # Add to buffer
        self.event_buffer.append(event)

        # Flush if buffer is full
        if len(self.event_buffer) >= self.buffer_size:
            self.flush_events()

        return event.event_id

    def log_cost(
        self,
        provider: str,
        operation: str,
        tokens_in: int = 0,
        tokens_out: int = 0,
        duration_ms: int = 0
    ):
        """Log AI usage cost"""
        # Calculate cost
        costs = self.provider_costs.get(provider, {"input": 0, "output": 0})
        estimated_cost = (tokens_in * costs["input"]) + (tokens_out * costs["output"])

        record = CostRecord(
            timestamp=datetime.utcnow().isoformat(),
            provider=provider,
            operation=operation,
            tokens_in=tokens_in,
            tokens_out=tokens_out,
            estimated_cost_usd=estimated_cost,
            duration_ms=duration_ms
        )

        self.cost_buffer.append(record)

        if len(self.cost_buffer) >= self.buffer_size:
            self.flush_costs()

    def log_metric(
        self,
        metric_name: str,
        value: float,
        unit: str = "",
        tags: Optional[Dict[str, str]] = None
    ):
        """Log a performance metric"""
        metric = PerformanceMetric(
            timestamp=datetime.utcnow().isoformat(),
            metric_name=metric_name,
            value=value,
            unit=unit,
            tags=tags or {}
        )

        self.metric_buffer.append(metric)

        if len(self.metric_buffer) >= self.buffer_size:
            self.flush_metrics()

    def flush_events(self):
        """Flush event buffer to disk"""
        if not self.event_buffer:
            return

        log_file = self._get_log_file(datetime.utcnow(), "events")

        with self.lock:
            events = self.event_buffer.copy()
            self.event_buffer.clear()

        for event in events:
            self._append_to_log(log_file, event.to_dict())

    def flush_costs(self):
        """Flush cost buffer to disk"""
        if not self.cost_buffer:
            return

        log_file = self._get_log_file(datetime.utcnow(), "costs")

        with self.lock:
            costs = self.cost_buffer.copy()
            self.cost_buffer.clear()

        for cost in costs:
            self._append_to_log(log_file, asdict(cost))

    def flush_metrics(self):
        """Flush metrics buffer to disk"""
        if not self.metric_buffer:
            return

        log_file = self._get_log_file(datetime.utcnow(), "metrics")

        with self.lock:
            metrics = self.metric_buffer.copy()
            self.metric_buffer.clear()

        for metric in metrics:
            self._append_to_log(log_file, asdict(metric))

    def flush_all(self):
        """Flush all buffers"""
        self.flush_events()
        self.flush_costs()
        self.flush_metrics()

    def query_events(
        self,
        start_date: Optional[datetime] = None,
        end_date: Optional[datetime] = None,
        event_types: Optional[List[EventType]] = None,
        repository: Optional[str] = None,
        pr_number: Optional[int] = None,
        severity: Optional[Severity] = None,
        limit: int = 1000
    ) -> List[AuditEvent]:
        """Query audit events"""
        if not start_date:
            start_date = datetime.utcnow() - timedelta(days=7)
        if not end_date:
            end_date = datetime.utcnow()

        events = []
        current_date = start_date

        while current_date <= end_date:
            log_file = self._get_log_file(current_date, "events")
            if log_file.exists():
                with open(log_file) as f:
                    for line in f:
                        if len(events) >= limit:
                            break
                        try:
                            data = json.loads(line)
                            event = AuditEvent(**data)

                            # Apply filters
                            if event_types and event.event_type not in [e.value for e in event_types]:
                                continue
                            if repository and event.repository != repository:
                                continue
                            if pr_number and event.pr_number != pr_number:
                                continue
                            if severity and event.severity != severity.value:
                                continue

                            events.append(event)
                        except:
                            pass

            current_date += timedelta(days=1)

        return events

    def get_cost_summary(
        self,
        start_date: Optional[datetime] = None,
        end_date: Optional[datetime] = None
    ) -> Dict[str, Any]:
        """Get cost summary for a date range"""
        if not start_date:
            start_date = datetime.utcnow() - timedelta(days=30)
        if not end_date:
            end_date = datetime.utcnow()

        costs_by_provider = defaultdict(float)
        total_tokens_in = 0
        total_tokens_out = 0
        total_cost = 0.0

        current_date = start_date
        while current_date <= end_date:
            log_file = self._get_log_file(current_date, "costs")
            if log_file.exists():
                with open(log_file) as f:
                    for line in f:
                        try:
                            data = json.loads(line)
                            costs_by_provider[data["provider"]] += data["estimated_cost_usd"]
                            total_tokens_in += data["tokens_in"]
                            total_tokens_out += data["tokens_out"]
                            total_cost += data["estimated_cost_usd"]
                        except:
                            pass
            current_date += timedelta(days=1)

        return {
            "start_date": start_date.isoformat(),
            "end_date": end_date.isoformat(),
            "total_cost_usd": round(total_cost, 4),
            "total_tokens_in": total_tokens_in,
            "total_tokens_out": total_tokens_out,
            "costs_by_provider": dict(costs_by_provider)
        }

    def get_metrics_summary(
        self,
        metric_name: str,
        start_date: Optional[datetime] = None,
        end_date: Optional[datetime] = None
    ) -> Dict[str, Any]:
        """Get metrics summary for a specific metric"""
        if not start_date:
            start_date = datetime.utcnow() - timedelta(days=7)
        if not end_date:
            end_date = datetime.utcnow()

        values = []

        current_date = start_date
        while current_date <= end_date:
            log_file = self._get_log_file(current_date, "metrics")
            if log_file.exists():
                with open(log_file) as f:
                    for line in f:
                        try:
                            data = json.loads(line)
                            if data["metric_name"] == metric_name:
                                values.append(data["value"])
                        except:
                            pass
            current_date += timedelta(days=1)

        if not values:
            return {"metric_name": metric_name, "count": 0}

        return {
            "metric_name": metric_name,
            "count": len(values),
            "min": min(values),
            "max": max(values),
            "avg": sum(values) / len(values),
            "sum": sum(values)
        }

    def cleanup_old_logs(self):
        """Remove logs older than retention period"""
        cutoff_date = datetime.utcnow() - timedelta(days=self.retention_days)

        for log_file in self.log_dir.glob("*.jsonl"):
            try:
                # Parse date from filename
                date_str = log_file.stem.split("_")[-1]
                file_date = datetime.strptime(date_str, "%Y-%m-%d")

                if file_date < cutoff_date:
                    log_file.unlink()
                    logger.info(f"Deleted old log file: {log_file}")
            except:
                pass

    def generate_audit_report(
        self,
        repository: str,
        start_date: Optional[datetime] = None,
        end_date: Optional[datetime] = None
    ) -> str:
        """Generate a human-readable audit report"""
        if not start_date:
            start_date = datetime.utcnow() - timedelta(days=7)
        if not end_date:
            end_date = datetime.utcnow()

        events = self.query_events(start_date, end_date, repository=repository)
        costs = self.get_cost_summary(start_date, end_date)

        # Count events by type
        event_counts = defaultdict(int)
        error_count = 0
        for event in events:
            event_counts[event.event_type] += 1
            if event.severity == Severity.ERROR.value:
                error_count += 1

        report = f"""# FlexNetOS Audit Report

**Repository:** {repository}
**Period:** {start_date.strftime('%Y-%m-%d')} to {end_date.strftime('%Y-%m-%d')}
**Generated:** {datetime.utcnow().isoformat()}Z

---

## Summary

| Metric | Value |
|--------|-------|
| Total Events | {len(events)} |
| Errors | {error_count} |
| Resolution Attempts | {event_counts.get('resolution_attempted', 0)} |
| Successful Fixes | {event_counts.get('fix_applied', 0)} |
| PRs Merged | {event_counts.get('pr_merged', 0)} |
| Rollbacks | {event_counts.get('rollback_triggered', 0)} |

---

## Cost Summary

| Provider | Cost (USD) |
|----------|------------|
"""
        for provider, cost in costs.get("costs_by_provider", {}).items():
            report += f"| {provider} | ${cost:.4f} |\n"

        report += f"""| **Total** | **${costs.get('total_cost_usd', 0):.4f}** |

---

## Event Breakdown

"""
        for event_type, count in sorted(event_counts.items()):
            report += f"- {event_type}: {count}\n"

        report += """
---

## Recent Errors

"""
        error_events = [e for e in events if e.severity == Severity.ERROR.value][-10:]
        for event in error_events:
            report += f"- **{event.timestamp}**: {event.action}\n"
            if event.details.get("error"):
                report += f"  - Error: {event.details['error']}\n"

        report += """
---

> 🤖 Generated by FlexNetOS Audit Logger
"""
        return report


# Convenience functions
_default_logger: Optional[AuditLogger] = None

def get_logger() -> AuditLogger:
    """Get the default audit logger"""
    global _default_logger
    if _default_logger is None:
        _default_logger = AuditLogger()
    return _default_logger

def log_event(event_type: EventType, action: str, repository: str, **kwargs) -> str:
    """Convenience function to log an event"""
    return get_logger().log_event(event_type, action, repository, **kwargs)

def log_cost(provider: str, operation: str, **kwargs):
    """Convenience function to log a cost"""
    get_logger().log_cost(provider, operation, **kwargs)

def log_metric(metric_name: str, value: float, **kwargs):
    """Convenience function to log a metric"""
    get_logger().log_metric(metric_name, value, **kwargs)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='FlexNetOS Audit Logger')
    parser.add_argument('--action', choices=['report', 'costs', 'cleanup'], required=True)
    parser.add_argument('--repo', help='Repository name')
    parser.add_argument('--days', type=int, default=7, help='Number of days to query')

    args = parser.parse_args()

    logger_instance = AuditLogger()

    end_date = datetime.utcnow()
    start_date = end_date - timedelta(days=args.days)

    if args.action == 'report':
        if not args.repo:
            print("--repo is required for report")
            exit(1)
        report = logger_instance.generate_audit_report(args.repo, start_date, end_date)
        print(report)

    elif args.action == 'costs':
        costs = logger_instance.get_cost_summary(start_date, end_date)
        print(json.dumps(costs, indent=2))

    elif args.action == 'cleanup':
        logger_instance.cleanup_old_logs()
        print("Cleanup complete")

