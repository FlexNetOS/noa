"""Discover Stage (Stage 1).

T165: Implement Discover stage
§3.4: Digest Everything Pipeline - Stage 1: Discover
US4: Digest Everything Pipeline
"""

from pathlib import Path
import logging

logger = logging.getLogger(__name__)


class DiscoverStage:
    """Discover sources to digest (repositories, files, APIs, documents)."""

    def __init__(self, base_path: Path | None = None):
        """Initialize discover stage.

        Args:
            base_path: Base path for discovery (defaults to current directory)
        """
        self.base_path = base_path or Path.cwd()

    def discover(self, source: str) -> list[dict]:
        """Discover sources to digest.

        Args:
            source: Source URI or path (GitHub URL, file path, etc.)

        Returns:
            List of discovered sources with metadata
        """
        logger.info(f"Discovering sources from: {source}")

        sources = []

        # Check if source is a GitHub repository URL
        if source.startswith("https://github.com/") or source.startswith(
            "http://github.com/"
        ):
            sources.append(
                {
                    "type": "repository",
                    "uri": source,
                    "name": self._extract_repo_name(source),
                }
            )
        # Check if source is a local directory
        elif Path(source).exists() and Path(source).is_dir():
            sources.append(
                {
                    "type": "directory",
                    "uri": str(Path(source).absolute()),
                    "name": Path(source).name,
                }
            )
        # Check if source is a local file
        elif Path(source).exists() and Path(source).is_file():
            sources.append(
                {
                    "type": "file",
                    "uri": str(Path(source).absolute()),
                    "name": Path(source).name,
                }
            )
        else:
            # Unknown source type
            logger.warning(f"Unknown source type: {source}")
            sources.append(
                {
                    "type": "unknown",
                    "uri": source,
                    "name": source.split("/")[-1],
                }
            )

        return sources

    def _extract_repo_name(self, github_url: str) -> str:
        """Extract repository name from GitHub URL."""
        parts = github_url.rstrip("/").split("/")
        if len(parts) >= 2:
            return f"{parts[-2]}/{parts[-1]}".replace(".git", "")
        return parts[-1].replace(".git", "")


