#!/usr/bin/env python3
"""
Provider Health Check for Spec-Kit (SK009)

Implements provider health checking before spec distribution to ensure
providers are available and responsive.

Constitutional reference: §3.13, FR-037
"""

import json
import os
import shutil
import subprocess
import time
from dataclasses import dataclass
from datetime import datetime, timezone, timedelta
from pathlib import Path
from typing import Any, Dict, List, Optional
from concurrent.futures import ThreadPoolExecutor, as_completed


@dataclass
class HealthStatus:
    """Health status for a single provider."""
    provider_id: str
    healthy: bool
    latency_ms: float
    message: str
    checked_at: datetime
    details: Dict[str, Any]

    def to_dict(self) -> Dict[str, Any]:
        return {
            "providerId": self.provider_id,
            "healthy": self.healthy,
            "latencyMs": self.latency_ms,
            "message": self.message,
            "checkedAt": self.checked_at.isoformat(),
            "details": self.details,
        }


@dataclass
class HealthReport:
    """Health report for all providers."""
    total_providers: int
    healthy_count: int
    unhealthy_count: int
    statuses: List[HealthStatus]
    duration_ms: float

    @property
    def all_healthy(self) -> bool:
        return self.unhealthy_count == 0

    def to_dict(self) -> Dict[str, Any]:
        return {
            "totalProviders": self.total_providers,
            "healthyCount": self.healthy_count,
            "unhealthyCount": self.unhealthy_count,
            "allHealthy": self.all_healthy,
            "durationMs": self.duration_ms,
            "statuses": [s.to_dict() for s in self.statuses],
        }


# Provider health check configurations
PROVIDER_HEALTH_CHECKS: Dict[str, Dict[str, Any]] = {
    "cursor": {
        "type": "process",
        "check_method": "ide_running",
        "process_names": ["cursor", "Cursor.exe"],
        "timeout_ms": 1000,
    },
    "claude-code": {
        "type": "cli",
        "check_method": "cli_available",
        "cli_command": ["claude", "--version"],
        "timeout_ms": 5000,
    },
    "codex": {
        "type": "cli",
        "check_method": "cli_available",
        "cli_command": ["codex", "--version"],
        "timeout_ms": 5000,
    },
    "vscode-copilot": {
        "type": "process",
        "check_method": "ide_running",
        "process_names": ["code", "Code.exe", "code-insiders", "Code - Insiders.exe"],
        "timeout_ms": 1000,
    },
    "llama-server": {
        "type": "http",
        "check_method": "http_health",
        "health_url": "http://localhost:8080/health",
        "timeout_ms": 2000,
    },
    "ollama": {
        "type": "cli",
        "check_method": "cli_available",
        "cli_command": ["ollama", "list"],
        "timeout_ms": 5000,
    },
    "abacus": {
        "type": "cli",
        "check_method": "cli_available",
        "cli_command": ["abacusai", "--version"],
        "timeout_ms": 5000,
    },
    "git-cli": {
        "type": "cli",
        "check_method": "cli_available",
        "cli_command": ["git", "--version"],
        "timeout_ms": 2000,
    },
}


class ProviderHealthChecker:
    """
    Checks health of AI providers before spec distribution.

    Features:
    - CLI availability checks
    - Process/IDE running checks
    - HTTP endpoint health checks
    - Parallel health checking
    - Configurable timeouts
    """

    DEFAULT_TIMEOUT_MS = 5000

    def __init__(self, noa_root: Optional[Path] = None):
        """Initialize the health checker."""
        self.noa_root = noa_root or Path(os.environ.get("NOA_ROOT", Path.cwd()))
        self._custom_checks: Dict[str, callable] = {}

    def register_custom_check(
        self,
        provider_id: str,
        check_fn: callable,
    ) -> None:
        """
        Register a custom health check function for a provider.

        Args:
            provider_id: Provider to register check for
            check_fn: Function that returns (healthy: bool, message: str, details: dict)
        """
        self._custom_checks[provider_id] = check_fn

    def check_provider(
        self,
        provider_id: str,
        timeout_ms: Optional[int] = None,
    ) -> HealthStatus:
        """
        Check health of a single provider.

        Args:
            provider_id: Provider to check
            timeout_ms: Timeout in milliseconds

        Returns:
            HealthStatus for the provider
        """
        start = time.time()
        now = datetime.now(timezone.utc)

        # Use custom check if registered
        if provider_id in self._custom_checks:
            try:
                healthy, message, details = self._custom_checks[provider_id]()
                latency = (time.time() - start) * 1000
                return HealthStatus(
                    provider_id=provider_id,
                    healthy=healthy,
                    latency_ms=latency,
                    message=message,
                    checked_at=now,
                    details=details,
                )
            except Exception as e:
                latency = (time.time() - start) * 1000
                return HealthStatus(
                    provider_id=provider_id,
                    healthy=False,
                    latency_ms=latency,
                    message=f"Custom check failed: {e}",
                    checked_at=now,
                    details={"error": str(e)},
                )

        # Get check configuration
        check_config = PROVIDER_HEALTH_CHECKS.get(provider_id)
        if not check_config:
            # Default: check if CLI tool exists
            return self._check_cli_exists(provider_id, timeout_ms)

        timeout_ms = timeout_ms or check_config.get("timeout_ms", self.DEFAULT_TIMEOUT_MS)
        check_method = check_config.get("check_method")

        if check_method == "cli_available":
            return self._check_cli_available(
                provider_id,
                check_config.get("cli_command", [provider_id, "--version"]),
                timeout_ms,
            )
        elif check_method == "ide_running":
            return self._check_ide_running(
                provider_id,
                check_config.get("process_names", []),
                timeout_ms,
            )
        elif check_method == "http_health":
            return self._check_http_health(
                provider_id,
                check_config.get("health_url", ""),
                timeout_ms,
            )
        else:
            return self._check_cli_exists(provider_id, timeout_ms)

    def _check_cli_exists(
        self,
        provider_id: str,
        timeout_ms: Optional[int] = None,
    ) -> HealthStatus:
        """Check if a CLI tool exists in PATH."""
        start = time.time()
        now = datetime.now(timezone.utc)

        # Check in NOA bin first
        noa_bin = self.noa_root / "bin"
        for ext in ["", ".exe", ".cmd", ".bat"]:
            tool_path = noa_bin / f"{provider_id}{ext}"
            if tool_path.exists():
                latency = (time.time() - start) * 1000
                return HealthStatus(
                    provider_id=provider_id,
                    healthy=True,
                    latency_ms=latency,
                    message=f"Found in NOA bin: {tool_path}",
                    checked_at=now,
                    details={"path": str(tool_path), "source": "noa_bin"},
                )

        # Check system PATH
        tool_path = shutil.which(provider_id)
        if tool_path:
            latency = (time.time() - start) * 1000
            return HealthStatus(
                provider_id=provider_id,
                healthy=True,
                latency_ms=latency,
                message=f"Found in PATH: {tool_path}",
                checked_at=now,
                details={"path": tool_path, "source": "system_path"},
            )

        latency = (time.time() - start) * 1000
        return HealthStatus(
            provider_id=provider_id,
            healthy=False,
            latency_ms=latency,
            message=f"CLI tool not found: {provider_id}",
            checked_at=now,
            details={"searched": [str(noa_bin), "system PATH"]},
        )

    def _check_cli_available(
        self,
        provider_id: str,
        command: List[str],
        timeout_ms: int,
    ) -> HealthStatus:
        """Check if a CLI command can be executed."""
        start = time.time()
        now = datetime.now(timezone.utc)

        try:
            timeout_sec = timeout_ms / 1000
            result = subprocess.run(
                command,
                capture_output=True,
                text=True,
                timeout=timeout_sec,
            )

            latency = (time.time() - start) * 1000

            if result.returncode == 0:
                version_info = result.stdout.strip()[:100]  # Limit version output
                return HealthStatus(
                    provider_id=provider_id,
                    healthy=True,
                    latency_ms=latency,
                    message=f"CLI available: {version_info}",
                    checked_at=now,
                    details={"command": command, "output": version_info},
                )
            else:
                return HealthStatus(
                    provider_id=provider_id,
                    healthy=False,
                    latency_ms=latency,
                    message=f"CLI returned error (exit code {result.returncode})",
                    checked_at=now,
                    details={
                        "command": command,
                        "exitCode": result.returncode,
                        "stderr": result.stderr[:200] if result.stderr else None,
                    },
                )

        except subprocess.TimeoutExpired:
            latency = (time.time() - start) * 1000
            return HealthStatus(
                provider_id=provider_id,
                healthy=False,
                latency_ms=latency,
                message=f"CLI timed out after {timeout_ms}ms",
                checked_at=now,
                details={"command": command, "timeout": timeout_ms},
            )

        except FileNotFoundError:
            latency = (time.time() - start) * 1000
            return HealthStatus(
                provider_id=provider_id,
                healthy=False,
                latency_ms=latency,
                message=f"CLI not found: {command[0]}",
                checked_at=now,
                details={"command": command},
            )

        except Exception as e:
            latency = (time.time() - start) * 1000
            return HealthStatus(
                provider_id=provider_id,
                healthy=False,
                latency_ms=latency,
                message=f"CLI check error: {e}",
                checked_at=now,
                details={"command": command, "error": str(e)},
            )

    def _check_ide_running(
        self,
        provider_id: str,
        process_names: List[str],
        timeout_ms: int,
    ) -> HealthStatus:
        """Check if an IDE process is running."""
        start = time.time()
        now = datetime.now(timezone.utc)

        try:
            import psutil

            for proc in psutil.process_iter(['name']):
                proc_name = proc.info['name']
                if proc_name and any(pn.lower() in proc_name.lower() for pn in process_names):
                    latency = (time.time() - start) * 1000
                    return HealthStatus(
                        provider_id=provider_id,
                        healthy=True,
                        latency_ms=latency,
                        message=f"IDE process running: {proc_name}",
                        checked_at=now,
                        details={"processName": proc_name, "pid": proc.pid},
                    )

            latency = (time.time() - start) * 1000
            return HealthStatus(
                provider_id=provider_id,
                healthy=False,
                latency_ms=latency,
                message=f"IDE not running (checked: {process_names})",
                checked_at=now,
                details={"searchedProcesses": process_names},
            )

        except ImportError:
            # psutil not available, fall back to CLI check
            return self._check_cli_exists(provider_id, timeout_ms)

        except Exception as e:
            latency = (time.time() - start) * 1000
            return HealthStatus(
                provider_id=provider_id,
                healthy=False,
                latency_ms=latency,
                message=f"Process check error: {e}",
                checked_at=now,
                details={"error": str(e)},
            )

    def _check_http_health(
        self,
        provider_id: str,
        health_url: str,
        timeout_ms: int,
    ) -> HealthStatus:
        """Check HTTP health endpoint."""
        start = time.time()
        now = datetime.now(timezone.utc)

        try:
            import httpx

            timeout_sec = timeout_ms / 1000
            response = httpx.get(health_url, timeout=timeout_sec)
            latency = (time.time() - start) * 1000

            if response.status_code == 200:
                return HealthStatus(
                    provider_id=provider_id,
                    healthy=True,
                    latency_ms=latency,
                    message=f"HTTP health check passed",
                    checked_at=now,
                    details={"url": health_url, "statusCode": response.status_code},
                )
            else:
                return HealthStatus(
                    provider_id=provider_id,
                    healthy=False,
                    latency_ms=latency,
                    message=f"HTTP health check failed (status {response.status_code})",
                    checked_at=now,
                    details={"url": health_url, "statusCode": response.status_code},
                )

        except ImportError:
            # httpx not available
            latency = (time.time() - start) * 1000
            return HealthStatus(
                provider_id=provider_id,
                healthy=False,
                latency_ms=latency,
                message="HTTP check unavailable (httpx not installed)",
                checked_at=now,
                details={"url": health_url},
            )

        except Exception as e:
            latency = (time.time() - start) * 1000
            return HealthStatus(
                provider_id=provider_id,
                healthy=False,
                latency_ms=latency,
                message=f"HTTP check error: {e}",
                checked_at=now,
                details={"url": health_url, "error": str(e)},
            )

    def check_all_providers(
        self,
        provider_ids: Optional[List[str]] = None,
        max_workers: int = 8,
    ) -> HealthReport:
        """
        Check health of multiple providers in parallel.

        Args:
            provider_ids: List of providers to check (None = all known)
            max_workers: Maximum parallel threads

        Returns:
            HealthReport with results for all providers
        """
        start = time.time()

        if provider_ids is None:
            provider_ids = list(PROVIDER_HEALTH_CHECKS.keys())

        statuses: List[HealthStatus] = []

        with ThreadPoolExecutor(max_workers=min(max_workers, len(provider_ids))) as executor:
            futures = {
                executor.submit(self.check_provider, pid): pid
                for pid in provider_ids
            }

            for future in as_completed(futures):
                try:
                    status = future.result()
                    statuses.append(status)
                except Exception as e:
                    pid = futures[future]
                    statuses.append(HealthStatus(
                        provider_id=pid,
                        healthy=False,
                        latency_ms=0,
                        message=f"Check failed: {e}",
                        checked_at=datetime.now(timezone.utc),
                        details={"error": str(e)},
                    ))

        duration = (time.time() - start) * 1000
        healthy_count = sum(1 for s in statuses if s.healthy)

        return HealthReport(
            total_providers=len(provider_ids),
            healthy_count=healthy_count,
            unhealthy_count=len(provider_ids) - healthy_count,
            statuses=statuses,
            duration_ms=duration,
        )

    def get_healthy_providers(
        self,
        provider_ids: Optional[List[str]] = None,
    ) -> List[str]:
        """Get list of healthy providers."""
        report = self.check_all_providers(provider_ids)
        return [s.provider_id for s in report.statuses if s.healthy]


# Convenience functions
def check_provider_health(
    provider_id: str,
    noa_root: Optional[Path] = None,
) -> HealthStatus:
    """
    Check health of a single provider.

    This is the primary entry point for SK009 - Provider Health Check.

    Args:
        provider_id: Provider to check
        noa_root: Optional NOA root override

    Returns:
        HealthStatus for the provider
    """
    checker = ProviderHealthChecker(noa_root)
    return checker.check_provider(provider_id)


def check_all_provider_health(
    provider_ids: Optional[List[str]] = None,
    noa_root: Optional[Path] = None,
) -> HealthReport:
    """Check health of multiple providers."""
    checker = ProviderHealthChecker(noa_root)
    return checker.check_all_providers(provider_ids)


def get_healthy_providers(
    noa_root: Optional[Path] = None,
) -> List[str]:
    """Get list of healthy providers."""
    checker = ProviderHealthChecker(noa_root)
    return checker.get_healthy_providers()

