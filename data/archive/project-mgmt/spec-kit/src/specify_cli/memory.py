#!/usr/bin/env python3
"""
Execution Memory Database for Spec-Kit (SK003)

Implements provider registration and state management via the shared
execution-memory.db SQLite database.

Constitutional reference: §3.7, §3.13, FR-037
"""

import json
import sqlite3
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, Generator, List, Optional
from uuid import uuid4


@dataclass
class TaskRecord:
    """Represents a distributed task record."""
    task_id: str
    assigned_provider: Optional[str]
    status: str
    priority: int
    created_at: datetime
    started_at: Optional[datetime]
    completed_at: Optional[datetime]
    result: Optional[str]


@dataclass
class ProviderStateRecord:
    """Represents provider state in the database."""
    provider: str
    status: str
    last_heartbeat: Optional[datetime]
    capabilities: List[str]
    current_load: float


@dataclass
class ExecutionContext:
    """Represents shared execution context."""
    session_id: str
    provider: str
    timestamp: datetime
    context_type: str
    content: str
    metadata: Optional[Dict[str, Any]]


class ExecutionMemoryDB:
    """
    Manages the shared execution memory database for provider coordination.

    This SQLite database enables:
    - Context sharing across AI providers
    - Reasoning state persistence
    - Parallel task distribution
    - Provider state synchronization
    """

    # Schema for initialization
    SCHEMA = """
    -- NOA Shared Execution Memory Schema
    -- Enables context sharing across AI providers (§4.10)

    CREATE TABLE IF NOT EXISTS execution_context (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        session_id TEXT NOT NULL,
        provider TEXT NOT NULL,
        timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
        context_type TEXT NOT NULL,
        content TEXT NOT NULL,
        metadata TEXT,
        UNIQUE(session_id, provider, context_type)
    );

    CREATE TABLE IF NOT EXISTS reasoning_state (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        session_id TEXT NOT NULL,
        provider TEXT NOT NULL,
        timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
        state_key TEXT NOT NULL,
        state_value TEXT NOT NULL,
        UNIQUE(session_id, state_key)
    );

    CREATE TABLE IF NOT EXISTS task_distribution (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        task_id TEXT NOT NULL UNIQUE,
        assigned_provider TEXT,
        status TEXT DEFAULT 'pending',
        priority INTEGER DEFAULT 0,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        started_at DATETIME,
        completed_at DATETIME,
        result TEXT
    );

    CREATE TABLE IF NOT EXISTS provider_state (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        provider TEXT NOT NULL UNIQUE,
        status TEXT DEFAULT 'unknown',
        last_heartbeat DATETIME,
        capabilities TEXT,
        current_load REAL DEFAULT 0.0
    );

    CREATE INDEX IF NOT EXISTS idx_context_session ON execution_context(session_id);
    CREATE INDEX IF NOT EXISTS idx_reasoning_session ON reasoning_state(session_id);
    CREATE INDEX IF NOT EXISTS idx_task_status ON task_distribution(status);
    CREATE INDEX IF NOT EXISTS idx_provider_status ON provider_state(status);
    """

    def __init__(self, db_path: Optional[Path] = None, noa_root: Optional[Path] = None):
        """Initialize the execution memory database."""
        if db_path:
            self.db_path = db_path
        else:
            noa_root = noa_root or self._detect_noa_root()
            self.db_path = noa_root / "ai" / "shared" / "resources" / "execution-memory.db"

        self._ensure_initialized()

    def _detect_noa_root(self) -> Path:
        """Detect NOA_ROOT from environment or current directory."""
        import os
        noa_root = os.environ.get("NOA_ROOT")
        if noa_root:
            return Path(noa_root)
        return Path.cwd()

    def _ensure_initialized(self) -> None:
        """Ensure the database is initialized with schema."""
        self.db_path.parent.mkdir(parents=True, exist_ok=True)

        with self._connection() as conn:
            conn.executescript(self.SCHEMA)

    @contextmanager
    def _connection(self) -> Generator[sqlite3.Connection, None, None]:
        """Context manager for database connections."""
        conn = sqlite3.connect(
            str(self.db_path),
            detect_types=sqlite3.PARSE_DECLTYPES | sqlite3.PARSE_COLNAMES,
        )
        conn.row_factory = sqlite3.Row
        try:
            yield conn
            conn.commit()
        except Exception:
            conn.rollback()
            raise
        finally:
            conn.close()

    # ==================== Provider State Management ====================

    def register_provider(
        self,
        provider: str,
        capabilities: List[str],
        status: str = "connected",
    ) -> ProviderStateRecord:
        """
        Register a provider in the execution memory.

        Args:
            provider: Provider identifier
            capabilities: List of provider capabilities
            status: Initial status (default: "connected")

        Returns:
            ProviderStateRecord representing the registered provider
        """
        now = datetime.now(timezone.utc)
        capabilities_json = json.dumps(capabilities)

        with self._connection() as conn:
            conn.execute("""
                INSERT OR REPLACE INTO provider_state
                (provider, status, last_heartbeat, capabilities, current_load)
                VALUES (?, ?, ?, ?, ?)
            """, (provider, status, now.isoformat(), capabilities_json, 0.0))

        return ProviderStateRecord(
            provider=provider,
            status=status,
            last_heartbeat=now,
            capabilities=capabilities,
            current_load=0.0,
        )

    def update_provider_status(
        self,
        provider: str,
        status: str,
        current_load: Optional[float] = None,
    ) -> bool:
        """Update a provider's status and optionally its load."""
        now = datetime.now(timezone.utc)

        with self._connection() as conn:
            if current_load is not None:
                result = conn.execute("""
                    UPDATE provider_state
                    SET status = ?, last_heartbeat = ?, current_load = ?
                    WHERE provider = ?
                """, (status, now.isoformat(), current_load, provider))
            else:
                result = conn.execute("""
                    UPDATE provider_state
                    SET status = ?, last_heartbeat = ?
                    WHERE provider = ?
                """, (status, now.isoformat(), provider))

            return result.rowcount > 0

    def get_provider_state(self, provider: str) -> Optional[ProviderStateRecord]:
        """Get the current state of a provider."""
        with self._connection() as conn:
            row = conn.execute(
                "SELECT * FROM provider_state WHERE provider = ?",
                (provider,)
            ).fetchone()

            if not row:
                return None

            return ProviderStateRecord(
                provider=row["provider"],
                status=row["status"],
                last_heartbeat=datetime.fromisoformat(row["last_heartbeat"]) if row["last_heartbeat"] else None,
                capabilities=json.loads(row["capabilities"]) if row["capabilities"] else [],
                current_load=row["current_load"] or 0.0,
            )

    def get_all_providers(self, status: Optional[str] = None) -> List[ProviderStateRecord]:
        """Get all providers, optionally filtered by status."""
        with self._connection() as conn:
            if status:
                rows = conn.execute(
                    "SELECT * FROM provider_state WHERE status = ?",
                    (status,)
                ).fetchall()
            else:
                rows = conn.execute("SELECT * FROM provider_state").fetchall()

            return [
                ProviderStateRecord(
                    provider=row["provider"],
                    status=row["status"],
                    last_heartbeat=datetime.fromisoformat(row["last_heartbeat"]) if row["last_heartbeat"] else None,
                    capabilities=json.loads(row["capabilities"]) if row["capabilities"] else [],
                    current_load=row["current_load"] or 0.0,
                )
                for row in rows
            ]

    def heartbeat(self, provider: str) -> bool:
        """Update provider's heartbeat timestamp."""
        return self.update_provider_status(provider, "connected")

    # ==================== Task Distribution ====================

    def create_task(
        self,
        task_id: Optional[str] = None,
        priority: int = 0,
        assigned_provider: Optional[str] = None,
    ) -> TaskRecord:
        """Create a new distributed task."""
        task_id = task_id or str(uuid4())
        now = datetime.now(timezone.utc)

        with self._connection() as conn:
            conn.execute("""
                INSERT INTO task_distribution
                (task_id, assigned_provider, status, priority, created_at)
                VALUES (?, ?, 'pending', ?, ?)
            """, (task_id, assigned_provider, priority, now.isoformat()))

        return TaskRecord(
            task_id=task_id,
            assigned_provider=assigned_provider,
            status="pending",
            priority=priority,
            created_at=now,
            started_at=None,
            completed_at=None,
            result=None,
        )

    def assign_task(self, task_id: str, provider: str) -> bool:
        """Assign a task to a provider."""
        with self._connection() as conn:
            result = conn.execute("""
                UPDATE task_distribution
                SET assigned_provider = ?, status = 'assigned'
                WHERE task_id = ? AND status = 'pending'
            """, (provider, task_id))
            return result.rowcount > 0

    def start_task(self, task_id: str) -> bool:
        """Mark a task as started."""
        now = datetime.now(timezone.utc)

        with self._connection() as conn:
            result = conn.execute("""
                UPDATE task_distribution
                SET status = 'in-progress', started_at = ?
                WHERE task_id = ? AND status IN ('pending', 'assigned')
            """, (now.isoformat(), task_id))
            return result.rowcount > 0

    def complete_task(self, task_id: str, result: Optional[str] = None) -> bool:
        """Mark a task as completed with optional result."""
        now = datetime.now(timezone.utc)

        with self._connection() as conn:
            db_result = conn.execute("""
                UPDATE task_distribution
                SET status = 'done', completed_at = ?, result = ?
                WHERE task_id = ?
            """, (now.isoformat(), result, task_id))
            return db_result.rowcount > 0

    def fail_task(self, task_id: str, error: str) -> bool:
        """Mark a task as failed."""
        now = datetime.now(timezone.utc)

        with self._connection() as conn:
            result = conn.execute("""
                UPDATE task_distribution
                SET status = 'failed', completed_at = ?, result = ?
                WHERE task_id = ?
            """, (now.isoformat(), f"ERROR: {error}", task_id))
            return result.rowcount > 0

    def get_task(self, task_id: str) -> Optional[TaskRecord]:
        """Get a task by ID."""
        with self._connection() as conn:
            row = conn.execute(
                "SELECT * FROM task_distribution WHERE task_id = ?",
                (task_id,)
            ).fetchone()

            if not row:
                return None

            return self._row_to_task(row)

    def get_pending_tasks(self, provider: Optional[str] = None) -> List[TaskRecord]:
        """Get pending tasks, optionally filtered by assigned provider."""
        with self._connection() as conn:
            if provider:
                rows = conn.execute("""
                    SELECT * FROM task_distribution
                    WHERE status = 'pending' AND (assigned_provider = ? OR assigned_provider IS NULL)
                    ORDER BY priority DESC, created_at ASC
                """, (provider,)).fetchall()
            else:
                rows = conn.execute("""
                    SELECT * FROM task_distribution
                    WHERE status = 'pending'
                    ORDER BY priority DESC, created_at ASC
                """).fetchall()

            return [self._row_to_task(row) for row in rows]

    def _row_to_task(self, row: sqlite3.Row) -> TaskRecord:
        """Convert a database row to a TaskRecord."""
        return TaskRecord(
            task_id=row["task_id"],
            assigned_provider=row["assigned_provider"],
            status=row["status"],
            priority=row["priority"],
            created_at=datetime.fromisoformat(row["created_at"]) if row["created_at"] else datetime.now(timezone.utc),
            started_at=datetime.fromisoformat(row["started_at"]) if row["started_at"] else None,
            completed_at=datetime.fromisoformat(row["completed_at"]) if row["completed_at"] else None,
            result=row["result"],
        )

    # ==================== Execution Context ====================

    def save_context(
        self,
        session_id: str,
        provider: str,
        context_type: str,
        content: str,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> ExecutionContext:
        """Save execution context for a session."""
        now = datetime.now(timezone.utc)
        metadata_json = json.dumps(metadata) if metadata else None

        with self._connection() as conn:
            conn.execute("""
                INSERT OR REPLACE INTO execution_context
                (session_id, provider, timestamp, context_type, content, metadata)
                VALUES (?, ?, ?, ?, ?, ?)
            """, (session_id, provider, now.isoformat(), context_type, content, metadata_json))

        return ExecutionContext(
            session_id=session_id,
            provider=provider,
            timestamp=now,
            context_type=context_type,
            content=content,
            metadata=metadata,
        )

    def get_context(
        self,
        session_id: str,
        provider: Optional[str] = None,
        context_type: Optional[str] = None,
    ) -> List[ExecutionContext]:
        """Get execution context for a session."""
        with self._connection() as conn:
            query = "SELECT * FROM execution_context WHERE session_id = ?"
            params: List[Any] = [session_id]

            if provider:
                query += " AND provider = ?"
                params.append(provider)

            if context_type:
                query += " AND context_type = ?"
                params.append(context_type)

            rows = conn.execute(query, params).fetchall()

            return [
                ExecutionContext(
                    session_id=row["session_id"],
                    provider=row["provider"],
                    timestamp=datetime.fromisoformat(row["timestamp"]) if row["timestamp"] else datetime.now(timezone.utc),
                    context_type=row["context_type"],
                    content=row["content"],
                    metadata=json.loads(row["metadata"]) if row["metadata"] else None,
                )
                for row in rows
            ]

    # ==================== Reasoning State ====================

    def save_reasoning_state(
        self,
        session_id: str,
        provider: str,
        state_key: str,
        state_value: str,
    ) -> None:
        """Save reasoning state for a session."""
        now = datetime.now(timezone.utc)

        with self._connection() as conn:
            conn.execute("""
                INSERT OR REPLACE INTO reasoning_state
                (session_id, provider, timestamp, state_key, state_value)
                VALUES (?, ?, ?, ?, ?)
            """, (session_id, provider, now.isoformat(), state_key, state_value))

    def get_reasoning_state(
        self,
        session_id: str,
        state_key: Optional[str] = None,
    ) -> Dict[str, str]:
        """Get reasoning state for a session."""
        with self._connection() as conn:
            if state_key:
                rows = conn.execute("""
                    SELECT state_key, state_value FROM reasoning_state
                    WHERE session_id = ? AND state_key = ?
                """, (session_id, state_key)).fetchall()
            else:
                rows = conn.execute("""
                    SELECT state_key, state_value FROM reasoning_state
                    WHERE session_id = ?
                """, (session_id,)).fetchall()

            return {row["state_key"]: row["state_value"] for row in rows}

    # ==================== Cleanup ====================

    def cleanup_old_sessions(self, days: int = 7) -> int:
        """Clean up sessions older than specified days."""
        from datetime import timedelta
        cutoff = (datetime.now(timezone.utc) - timedelta(days=days)).isoformat()

        with self._connection() as conn:
            # Clean up execution context
            conn.execute(
                "DELETE FROM execution_context WHERE timestamp < ?",
                (cutoff,)
            )

            # Clean up reasoning state
            conn.execute(
                "DELETE FROM reasoning_state WHERE timestamp < ?",
                (cutoff,)
            )

            # Clean up completed tasks
            result = conn.execute(
                "DELETE FROM task_distribution WHERE completed_at < ? AND status IN ('done', 'failed')",
                (cutoff,)
            )

            return result.rowcount


# Convenience functions for direct usage
def get_execution_memory(noa_root: Optional[Path] = None) -> ExecutionMemoryDB:
    """Get an ExecutionMemoryDB instance."""
    return ExecutionMemoryDB(noa_root=noa_root)


def register_provider(
    provider: str,
    capabilities: List[str],
    status: str = "connected",
    noa_root: Optional[Path] = None,
) -> ProviderStateRecord:
    """Register a provider in the execution memory."""
    db = ExecutionMemoryDB(noa_root=noa_root)
    return db.register_provider(provider, capabilities, status)

