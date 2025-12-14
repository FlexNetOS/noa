"""Fetch Stage (Stage 2).

T166: Implement Fetch stage with git clone
§3.4: Digest Everything Pipeline - Stage 2: Fetch
US4: Digest Everything Pipeline
"""

import logging
import subprocess
import tempfile
from pathlib import Path

logger = logging.getLogger(__name__)


class FetchStage:
    """Fetch source material (git clone, file copy, etc.)."""

    def __init__(self, work_dir: Path | None = None):
        """Initialize fetch stage.

        Args:
            work_dir: Working directory for fetched sources
        """
        self.work_dir = work_dir or Path(tempfile.mkdtemp(prefix="noa-digest-"))

    def fetch(self, source: dict) -> Path:
        """Fetch source material.

        Args:
            source: Source metadata from discover stage

        Returns:
            Path to fetched source
        """
        logger.info(f"Fetching source: {source['uri']}")

        source_type = source.get("type", "unknown")
        uri = source["uri"]

        if source_type == "repository":
            return self._fetch_repository(uri)
        elif source_type in ("directory", "file"):
            return Path(uri)
        else:
            raise ValueError(f"Unknown source type: {source_type}")

    def _fetch_repository(self, repo_url: str) -> Path:
        """Clone a Git repository.

        Args:
            repo_url: Git repository URL

        Returns:
            Path to cloned repository
        """
        repo_name = repo_url.rstrip("/").split("/")[-1].replace(".git", "")
        target_path = self.work_dir / repo_name

        if target_path.exists():
            logger.info(f"Repository already exists at {target_path}, skipping clone")
            return target_path

        logger.info(f"Cloning {repo_url} to {target_path}")
        subprocess.run(
            ["git", "clone", "--depth", "1", repo_url, str(target_path)],
            check=True,
            capture_output=True,
        )

        return target_path


