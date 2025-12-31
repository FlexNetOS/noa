#!/usr/bin/env python3
"""
Provider Connection System for Spec-Kit (SK001)

Implements the universal provider connection function that enables
all AI providers to access the same spec simultaneously.

Constitutional reference: §3.13, FR-037
"""

import json
import hashlib
import sqlite3
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Optional, List, Dict, Any, Literal

AccessMode = Literal["read", "write", "coordinate"]
ProviderStatus = Literal["connected", "disconnected", "error", "syncing"]


@dataclass
class ProviderCapabilities:
    """Capabilities that a provider supports."""
    reasoning: bool = False
    code_generation: bool = False
    code_completion: bool = False
    analysis: bool = False
    tool_use: bool = False
    long_context: bool = False
    local_inference: bool = False


@dataclass
class ProviderConnection:
    """Represents a connection to an AI provider."""
    provider_id: str
    provider_type: str  # "local", "cloud", "hybrid", "ide"
    access_mode: AccessMode
    connected_at: datetime
    last_sync: Optional[datetime] = None
    status: ProviderStatus = "connected"
    capabilities: ProviderCapabilities = field(default_factory=ProviderCapabilities)
    spec_path: Optional[str] = None
    parallel_enabled: bool = True

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for JSON serialization."""
        return {
            "providerId": self.provider_id,
            "providerType": self.provider_type,
            "accessMode": self.access_mode,
            "connectedAt": self.connected_at.isoformat(),
            "lastSync": self.last_sync.isoformat() if self.last_sync else None,
            "status": self.status,
            "capabilities": [
                cap for cap, enabled in vars(self.capabilities).items() if enabled
            ],
            "specPath": self.spec_path,
            "parallelEnabled": self.parallel_enabled,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "ProviderConnection":
        """Create from dictionary."""
        capabilities = ProviderCapabilities()
        for cap in data.get("capabilities", []):
            if hasattr(capabilities, cap):
                setattr(capabilities, cap, True)

        return cls(
            provider_id=data["providerId"],
            provider_type=data.get("providerType", "unknown"),
            access_mode=data.get("accessMode", "read"),
            connected_at=datetime.fromisoformat(data["connectedAt"]),
            last_sync=datetime.fromisoformat(data["lastSync"]) if data.get("lastSync") else None,
            status=data.get("status", "connected"),
            capabilities=capabilities,
            spec_path=data.get("specPath"),
            parallel_enabled=data.get("parallelEnabled", True),
        )


# Provider capability definitions (from task-distribution.yaml)
PROVIDER_CAPABILITIES: Dict[str, Dict[str, Any]] = {
    "cursor": {
        "type": "hybrid",
        "capabilities": ["reasoning", "code_generation", "analysis", "tool_use", "long_context"],
        "strengths": ["ide_integration", "codebase_context", "multi_file_edits"],
    },
    "claude-code": {
        "type": "cloud",
        "capabilities": ["reasoning", "code_generation", "analysis", "tool_use", "long_context"],
        "strengths": ["complex_reasoning", "long_context", "agentic_coding"],
    },
    "codex": {
        "type": "cloud",
        "capabilities": ["code_generation", "code_completion"],
        "strengths": ["code_completion", "multi_language"],
    },
    "abacus": {
        "type": "cloud",
        "capabilities": ["reasoning", "code_generation", "analysis"],
        "strengths": ["enterprise_workflows", "data_analysis", "custom_agents"],
    },
    "vscode-copilot": {
        "type": "ide",
        "capabilities": ["code_completion"],
        "strengths": ["inline_completion", "fast_suggestions", "ide_native"],
    },
    "llama-server": {
        "type": "local",
        "capabilities": ["code_generation", "local_inference"],
        "strengths": ["low_latency", "customizable", "multiple_models"],
    },
    "ollama": {
        "type": "local",
        "capabilities": ["local_inference"],
        "strengths": ["quick_responses", "local_inference", "privacy_sensitive"],
    },
    "git-cli": {
        "type": "local",
        "capabilities": [],
        "strengths": ["version_control"],
    },
}


class ProviderConnectionManager:
    """
    Manages connections to AI providers via the shared spec distribution system.

    All providers sharing the same spec_path get synchronized access via
    the execution-memory.db bus without creating duplicate copies.
    """

    def __init__(self, noa_root: Optional[Path] = None):
        """Initialize the provider connection manager."""
        self.noa_root = noa_root or self._detect_noa_root()
        self.shared_resources = self.noa_root / "ai" / "shared" / "resources"
        self.spec_distribution_path = self.shared_resources / "spec-distribution.json"
        self.execution_memory_path = self.shared_resources / "execution-memory.db"
        self._connections: Dict[str, ProviderConnection] = {}

    def _detect_noa_root(self) -> Path:
        """Detect NOA_ROOT from environment or current directory."""
        import os
        noa_root = os.environ.get("NOA_ROOT")
        if noa_root:
            return Path(noa_root)

        # Walk up from current directory looking for .noa-env
        current = Path.cwd()
        while current != current.parent:
            if (current / ".noa-env").exists():
                return current
            current = current.parent

        # Fall back to current directory
        return Path.cwd()

    def _load_spec_distribution(self) -> Dict[str, Any]:
        """Load the spec distribution configuration."""
        if not self.spec_distribution_path.exists():
            return self._create_default_spec_distribution()

        with open(self.spec_distribution_path) as f:
            return json.load(f)

    def _save_spec_distribution(self, data: Dict[str, Any]) -> None:
        """Save the spec distribution configuration."""
        self.spec_distribution_path.parent.mkdir(parents=True, exist_ok=True)
        data["lastUpdated"] = datetime.now(timezone.utc).isoformat()
        with open(self.spec_distribution_path, "w") as f:
            json.dump(data, f, indent=2)

    def _create_default_spec_distribution(self) -> Dict[str, Any]:
        """Create default spec distribution configuration."""
        return {
            "$schema": "https://noa.local/schemas/spec-distribution.json",
            "version": "1.0.0",
            "description": "Manages shared spec distribution across all AI providers",
            "lastUpdated": datetime.now(timezone.utc).isoformat(),
            "activeSpec": {
                "path": None,
                "hash": None,
                "files": [],
                "lastAccessed": None,
                "accessCount": 0,
            },
            "connectedProviders": [],
            "parallelExecution": {
                "enabled": True,
                "coordinator": "spec-kit",
                "coordinatorEntry": "/speckit.implement",
                "taskDistribution": "capability-based",
                "maxConcurrentProviders": 8,
                "syncInterval": 1000,
            },
            "lockingPolicy": {
                "enabled": True,
                "lockTimeout": 30000,
                "maxWaitTime": 60000,
                "retryAttempts": 3,
            },
        }

    def _compute_spec_hash(self, spec_path: Path) -> str:
        """Compute SHA-256 hash of spec contents for change detection."""
        hasher = hashlib.sha256()

        spec_files = ["spec.md", "plan.md", "tasks.md", "data-model.md", "research.md"]
        for filename in spec_files:
            filepath = spec_path / filename
            if filepath.exists():
                hasher.update(filepath.read_bytes())

        return f"sha256:{hasher.hexdigest()[:16]}"

    def _get_provider_capabilities(self, provider_id: str) -> ProviderCapabilities:
        """Get capabilities for a provider."""
        caps = ProviderCapabilities()
        provider_info = PROVIDER_CAPABILITIES.get(provider_id, {})

        for cap in provider_info.get("capabilities", []):
            if hasattr(caps, cap):
                setattr(caps, cap, True)

        return caps

    def connect_provider(
        self,
        provider_id: str,
        spec_path: str,
        access_mode: AccessMode = "read",
        parallel: bool = True,
    ) -> ProviderConnection:
        """
        Connect a provider to the shared spec distribution system.

        All providers sharing the same spec_path get synchronized access via
        the execution-memory.db bus without creating duplicate copies.

        Args:
            provider_id: Provider identifier (e.g., "claude-code", "codex", "cursor")
            spec_path: Path to the spec (relative to noa_root)
            access_mode: "read" | "write" | "coordinate"
            parallel: Enable parallel access by other providers

        Returns:
            ProviderConnection object representing the connection
        """
        # Get provider info
        provider_info = PROVIDER_CAPABILITIES.get(provider_id, {})
        provider_type = provider_info.get("type", "unknown")

        # Create connection
        now = datetime.now(timezone.utc)
        connection = ProviderConnection(
            provider_id=provider_id,
            provider_type=provider_type,
            access_mode=access_mode,
            connected_at=now,
            last_sync=now,
            status="connected",
            capabilities=self._get_provider_capabilities(provider_id),
            spec_path=spec_path,
            parallel_enabled=parallel,
        )

        # Update spec distribution
        spec_dist = self._load_spec_distribution()

        # Update active spec
        full_spec_path = self.noa_root / spec_path
        if full_spec_path.exists():
            spec_files = [f.name for f in full_spec_path.iterdir() if f.suffix == ".md"]
            spec_dist["activeSpec"] = {
                "path": spec_path,
                "hash": self._compute_spec_hash(full_spec_path),
                "files": spec_files,
                "lastAccessed": now.isoformat(),
                "accessCount": spec_dist.get("activeSpec", {}).get("accessCount", 0) + 1,
            }

        # Add/update provider in connected list
        providers = spec_dist.get("connectedProviders", [])
        # Remove existing entry for this provider
        providers = [p for p in providers if p.get("providerId") != provider_id]
        # Add new entry
        providers.append(connection.to_dict())
        spec_dist["connectedProviders"] = providers

        # Save updated config
        self._save_spec_distribution(spec_dist)

        # Store in memory
        self._connections[provider_id] = connection

        # Register in execution memory database
        self._register_in_execution_memory(connection)

        return connection

    def _register_in_execution_memory(self, connection: ProviderConnection) -> None:
        """Register provider connection in the execution memory database."""
        if not self.execution_memory_path.exists():
            self._init_execution_memory_db()

        conn = sqlite3.connect(str(self.execution_memory_path))
        try:
            cursor = conn.cursor()

            # Upsert provider state
            capabilities_json = json.dumps([
                cap for cap, enabled in vars(connection.capabilities).items() if enabled
            ])

            cursor.execute("""
                INSERT OR REPLACE INTO provider_state (provider, status, last_heartbeat, capabilities, current_load)
                VALUES (?, ?, ?, ?, ?)
            """, (
                connection.provider_id,
                connection.status,
                connection.connected_at.isoformat(),
                capabilities_json,
                0.0,
            ))

            conn.commit()
        finally:
            conn.close()

    def _init_execution_memory_db(self) -> None:
        """Initialize the execution memory database with schema."""
        schema_path = self.shared_resources / "schema" / "execution-memory.sql"

        self.execution_memory_path.parent.mkdir(parents=True, exist_ok=True)
        conn = sqlite3.connect(str(self.execution_memory_path))

        try:
            cursor = conn.cursor()

            if schema_path.exists():
                schema = schema_path.read_text()
                cursor.executescript(schema)
            else:
                # Inline schema if file not found
                cursor.executescript("""
                    CREATE TABLE IF NOT EXISTS provider_state (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        provider TEXT NOT NULL UNIQUE,
                        status TEXT DEFAULT 'unknown',
                        last_heartbeat DATETIME,
                        capabilities TEXT,
                        current_load REAL DEFAULT 0.0
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

                    CREATE INDEX IF NOT EXISTS idx_task_status ON task_distribution(status);
                """)

            conn.commit()
        finally:
            conn.close()

    def disconnect_provider(self, provider_id: str) -> bool:
        """
        Disconnect a provider from the shared spec distribution system.

        Args:
            provider_id: Provider identifier to disconnect

        Returns:
            True if disconnected successfully, False if provider wasn't connected
        """
        spec_dist = self._load_spec_distribution()

        # Remove from connected providers
        providers = spec_dist.get("connectedProviders", [])
        original_count = len(providers)
        providers = [p for p in providers if p.get("providerId") != provider_id]

        if len(providers) == original_count:
            return False  # Provider wasn't connected

        spec_dist["connectedProviders"] = providers
        self._save_spec_distribution(spec_dist)

        # Update execution memory
        if self.execution_memory_path.exists():
            conn = sqlite3.connect(str(self.execution_memory_path))
            try:
                cursor = conn.cursor()
                cursor.execute(
                    "UPDATE provider_state SET status = 'disconnected' WHERE provider = ?",
                    (provider_id,)
                )
                conn.commit()
            finally:
                conn.close()

        # Remove from memory
        self._connections.pop(provider_id, None)

        return True

    def get_connected_providers(self) -> List[ProviderConnection]:
        """Get all currently connected providers."""
        spec_dist = self._load_spec_distribution()
        providers = []

        for p in spec_dist.get("connectedProviders", []):
            try:
                providers.append(ProviderConnection.from_dict(p))
            except (KeyError, ValueError):
                continue

        return providers

    def get_active_spec(self) -> Optional[Dict[str, Any]]:
        """Get the currently active spec information."""
        spec_dist = self._load_spec_distribution()
        return spec_dist.get("activeSpec")

    def sync_provider(self, provider_id: str) -> bool:
        """
        Sync a provider's state with the shared spec distribution.

        Args:
            provider_id: Provider to sync

        Returns:
            True if sync successful
        """
        spec_dist = self._load_spec_distribution()
        now = datetime.now(timezone.utc).isoformat()

        for p in spec_dist.get("connectedProviders", []):
            if p.get("providerId") == provider_id:
                p["lastSync"] = now
                p["status"] = "connected"
                self._save_spec_distribution(spec_dist)
                return True

        return False


# Convenience function for direct usage
def connect_provider(
    provider_id: str,
    spec_path: str,
    access_mode: AccessMode = "read",
    parallel: bool = True,
    noa_root: Optional[Path] = None,
) -> ProviderConnection:
    """
    Connect a provider to the shared spec distribution system.

    This is the primary entry point for SK001 - Universal Provider Connection.

    All providers sharing the same spec_path get synchronized access via
    the execution-memory.db bus without creating duplicate copies.

    Args:
        provider_id: Provider identifier (e.g., "claude-code", "codex", "cursor")
        spec_path: Path to the spec (relative to noa_root)
        access_mode: "read" | "write" | "coordinate"
        parallel: Enable parallel access by other providers
        noa_root: Optional NOA root override

    Returns:
        ProviderConnection object representing the connection

    Example:
        >>> from specify_cli.providers import connect_provider
        >>> conn = connect_provider("claude-code", "specs/001-noa-seed-foundation")
        >>> print(conn.status)
        'connected'
    """
    manager = ProviderConnectionManager(noa_root)
    return manager.connect_provider(provider_id, spec_path, access_mode, parallel)


def disconnect_provider(
    provider_id: str,
    noa_root: Optional[Path] = None,
) -> bool:
    """Disconnect a provider from the shared spec distribution system."""
    manager = ProviderConnectionManager(noa_root)
    return manager.disconnect_provider(provider_id)


def get_connected_providers(
    noa_root: Optional[Path] = None,
) -> List[ProviderConnection]:
    """Get all currently connected providers."""
    manager = ProviderConnectionManager(noa_root)
    return manager.get_connected_providers()

