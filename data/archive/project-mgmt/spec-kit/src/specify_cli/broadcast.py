#!/usr/bin/env python3
"""
Parallel Spec Broadcast for Spec-Kit (SK004)

Implements parallel spec broadcast to all connected providers,
enabling simultaneous access to the same spec without duplication.

Constitutional reference: §3.13, FR-037
"""

import asyncio
import json
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable, Dict, List, Optional, Set

from .providers import ProviderConnection, ProviderConnectionManager, get_connected_providers
from .memory import ExecutionMemoryDB, get_execution_memory


@dataclass
class BroadcastResult:
    """Result of broadcasting to a single provider."""
    provider_id: str
    success: bool
    message: str
    timestamp: datetime
    latency_ms: float


@dataclass
class BroadcastReport:
    """Summary report of a broadcast operation."""
    total_providers: int
    successful: int
    failed: int
    results: List[BroadcastResult]
    duration_ms: float

    @property
    def all_successful(self) -> bool:
        return self.failed == 0

    def to_dict(self) -> Dict[str, Any]:
        return {
            "totalProviders": self.total_providers,
            "successful": self.successful,
            "failed": self.failed,
            "allSuccessful": self.all_successful,
            "durationMs": self.duration_ms,
            "results": [
                {
                    "providerId": r.provider_id,
                    "success": r.success,
                    "message": r.message,
                    "timestamp": r.timestamp.isoformat(),
                    "latencyMs": r.latency_ms,
                }
                for r in self.results
            ],
        }


class SpecBroadcaster:
    """
    Broadcasts spec updates to all connected providers in parallel.

    This enables all providers to access the same spec simultaneously
    via the shared execution memory bus.
    """

    def __init__(self, noa_root: Optional[Path] = None):
        """Initialize the spec broadcaster."""
        self.noa_root = noa_root or self._detect_noa_root()
        self.shared_resources = self.noa_root / "ai" / "shared" / "resources"
        self.spec_distribution_path = self.shared_resources / "spec-distribution.json"
        self.connection_manager = ProviderConnectionManager(self.noa_root)
        self.memory_db = ExecutionMemoryDB(noa_root=self.noa_root)
        self._broadcast_handlers: Dict[str, Callable] = {}

    def _detect_noa_root(self) -> Path:
        """Detect NOA_ROOT from environment."""
        import os
        return Path(os.environ.get("NOA_ROOT", Path.cwd()))

    def register_broadcast_handler(
        self,
        provider_id: str,
        handler: Callable[[str, Dict[str, Any]], bool],
    ) -> None:
        """
        Register a custom broadcast handler for a provider.

        Args:
            provider_id: Provider to register handler for
            handler: Function that takes (spec_path, metadata) and returns success bool
        """
        self._broadcast_handlers[provider_id] = handler

    def _default_broadcast_handler(
        self,
        provider_id: str,
        spec_path: str,
        metadata: Dict[str, Any],
    ) -> BroadcastResult:
        """Default handler that updates provider state in execution memory."""
        import time
        start = time.time()

        try:
            # Update provider state in database
            self.memory_db.update_provider_status(provider_id, "syncing")

            # Save broadcast context
            session_id = metadata.get("session_id", f"broadcast-{datetime.now(timezone.utc).strftime('%Y%m%d%H%M%S')}")
            self.memory_db.save_context(
                session_id=session_id,
                provider=provider_id,
                context_type="spec_broadcast",
                content=spec_path,
                metadata=metadata,
            )

            # Update provider state to connected after successful broadcast
            self.memory_db.update_provider_status(provider_id, "connected")

            latency = (time.time() - start) * 1000
            return BroadcastResult(
                provider_id=provider_id,
                success=True,
                message="Spec broadcast successful",
                timestamp=datetime.now(timezone.utc),
                latency_ms=latency,
            )
        except Exception as e:
            latency = (time.time() - start) * 1000
            self.memory_db.update_provider_status(provider_id, "error")
            return BroadcastResult(
                provider_id=provider_id,
                success=False,
                message=f"Broadcast failed: {e}",
                timestamp=datetime.now(timezone.utc),
                latency_ms=latency,
            )

    def broadcast_to_provider(
        self,
        provider_id: str,
        spec_path: str,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> BroadcastResult:
        """
        Broadcast spec to a single provider.

        Args:
            provider_id: Target provider
            spec_path: Path to spec (relative to noa_root)
            metadata: Optional metadata to include

        Returns:
            BroadcastResult
        """
        metadata = metadata or {}
        metadata["spec_path"] = spec_path
        metadata["broadcast_time"] = datetime.now(timezone.utc).isoformat()

        # Use custom handler if registered
        if provider_id in self._broadcast_handlers:
            import time
            start = time.time()
            try:
                success = self._broadcast_handlers[provider_id](spec_path, metadata)
                latency = (time.time() - start) * 1000
                return BroadcastResult(
                    provider_id=provider_id,
                    success=success,
                    message="Custom handler executed" if success else "Custom handler failed",
                    timestamp=datetime.now(timezone.utc),
                    latency_ms=latency,
                )
            except Exception as e:
                latency = (time.time() - start) * 1000
                return BroadcastResult(
                    provider_id=provider_id,
                    success=False,
                    message=f"Custom handler error: {e}",
                    timestamp=datetime.now(timezone.utc),
                    latency_ms=latency,
                )

        # Use default handler
        return self._default_broadcast_handler(provider_id, spec_path, metadata)

    def broadcast_parallel(
        self,
        spec_path: str,
        provider_ids: Optional[List[str]] = None,
        max_workers: int = 8,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> BroadcastReport:
        """
        Broadcast spec to multiple providers in parallel.

        Args:
            spec_path: Path to spec (relative to noa_root)
            provider_ids: List of providers (None = all connected)
            max_workers: Maximum parallel threads
            metadata: Optional metadata to include

        Returns:
            BroadcastReport with results for all providers
        """
        import time
        start = time.time()

        # Get target providers
        if provider_ids is None:
            connected = get_connected_providers(self.noa_root)
            provider_ids = [p.provider_id for p in connected]

        if not provider_ids:
            return BroadcastReport(
                total_providers=0,
                successful=0,
                failed=0,
                results=[],
                duration_ms=0,
            )

        results: List[BroadcastResult] = []

        # Execute broadcasts in parallel
        with ThreadPoolExecutor(max_workers=min(max_workers, len(provider_ids))) as executor:
            futures = {
                executor.submit(
                    self.broadcast_to_provider,
                    pid,
                    spec_path,
                    metadata,
                ): pid
                for pid in provider_ids
            }

            for future in as_completed(futures):
                try:
                    result = future.result()
                    results.append(result)
                except Exception as e:
                    pid = futures[future]
                    results.append(BroadcastResult(
                        provider_id=pid,
                        success=False,
                        message=f"Execution error: {e}",
                        timestamp=datetime.now(timezone.utc),
                        latency_ms=0,
                    ))

        duration = (time.time() - start) * 1000
        successful = sum(1 for r in results if r.success)

        return BroadcastReport(
            total_providers=len(provider_ids),
            successful=successful,
            failed=len(provider_ids) - successful,
            results=results,
            duration_ms=duration,
        )

    async def broadcast_parallel_async(
        self,
        spec_path: str,
        provider_ids: Optional[List[str]] = None,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> BroadcastReport:
        """
        Async version of parallel broadcast.

        Args:
            spec_path: Path to spec
            provider_ids: List of providers (None = all connected)
            metadata: Optional metadata

        Returns:
            BroadcastReport
        """
        import time
        start = time.time()

        # Get target providers
        if provider_ids is None:
            connected = get_connected_providers(self.noa_root)
            provider_ids = [p.provider_id for p in connected]

        if not provider_ids:
            return BroadcastReport(
                total_providers=0,
                successful=0,
                failed=0,
                results=[],
                duration_ms=0,
            )

        # Create async tasks
        loop = asyncio.get_event_loop()
        tasks = [
            loop.run_in_executor(
                None,
                self.broadcast_to_provider,
                pid,
                spec_path,
                metadata,
            )
            for pid in provider_ids
        ]

        # Wait for all to complete
        results = await asyncio.gather(*tasks, return_exceptions=True)

        # Process results
        broadcast_results: List[BroadcastResult] = []
        for i, result in enumerate(results):
            if isinstance(result, Exception):
                broadcast_results.append(BroadcastResult(
                    provider_id=provider_ids[i],
                    success=False,
                    message=f"Async error: {result}",
                    timestamp=datetime.now(timezone.utc),
                    latency_ms=0,
                ))
            else:
                broadcast_results.append(result)

        duration = (time.time() - start) * 1000
        successful = sum(1 for r in broadcast_results if r.success)

        return BroadcastReport(
            total_providers=len(provider_ids),
            successful=successful,
            failed=len(provider_ids) - successful,
            results=broadcast_results,
            duration_ms=duration,
        )

    def update_spec_distribution(
        self,
        spec_path: str,
        broadcast_report: BroadcastReport,
    ) -> None:
        """
        Update spec-distribution.json after a broadcast.

        Args:
            spec_path: The spec that was broadcast
            broadcast_report: Results of the broadcast
        """
        if not self.spec_distribution_path.exists():
            return

        with open(self.spec_distribution_path) as f:
            data = json.load(f)

        # Update active spec
        full_path = self.noa_root / spec_path
        if full_path.exists():
            data["activeSpec"]["path"] = spec_path
            data["activeSpec"]["lastAccessed"] = datetime.now(timezone.utc).isoformat()
            data["activeSpec"]["accessCount"] = data.get("activeSpec", {}).get("accessCount", 0) + 1

        # Update connected providers with sync timestamps
        for result in broadcast_report.results:
            for provider in data.get("connectedProviders", []):
                if provider.get("providerId") == result.provider_id:
                    if result.success:
                        provider["lastSync"] = result.timestamp.isoformat()
                        provider["status"] = "connected"
                    else:
                        provider["status"] = "error"

        data["lastUpdated"] = datetime.now(timezone.utc).isoformat()

        with open(self.spec_distribution_path, "w") as f:
            json.dump(data, f, indent=2)


# Convenience functions for direct usage
def broadcast_spec(
    spec_path: str,
    provider_ids: Optional[List[str]] = None,
    max_workers: int = 8,
    noa_root: Optional[Path] = None,
) -> BroadcastReport:
    """
    Broadcast a spec to all connected providers in parallel.

    This is the primary entry point for SK004 - Parallel Spec Broadcast.

    Args:
        spec_path: Path to spec (relative to noa_root)
        provider_ids: List of providers (None = all connected)
        max_workers: Maximum parallel threads
        noa_root: Optional NOA root override

    Returns:
        BroadcastReport with results for all providers

    Example:
        >>> from specify_cli.broadcast import broadcast_spec
        >>> report = broadcast_spec("specs/001-noa-seed-foundation")
        >>> print(f"Broadcast to {report.total_providers} providers")
        >>> print(f"Success rate: {report.successful}/{report.total_providers}")
    """
    broadcaster = SpecBroadcaster(noa_root)
    return broadcaster.broadcast_parallel(spec_path, provider_ids, max_workers)


async def broadcast_spec_async(
    spec_path: str,
    provider_ids: Optional[List[str]] = None,
    noa_root: Optional[Path] = None,
) -> BroadcastReport:
    """Async version of broadcast_spec."""
    broadcaster = SpecBroadcaster(noa_root)
    return await broadcaster.broadcast_parallel_async(spec_path, provider_ids)

