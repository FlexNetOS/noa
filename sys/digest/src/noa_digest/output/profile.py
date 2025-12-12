"""Profile.json Output Generator.

T177: Generate profile.json output
§3.4: Digest Everything Pipeline
US4: Digest Everything Pipeline
"""

import json
from pathlib import Path
from typing import Any
from datetime import datetime
import logging

logger = logging.getLogger(__name__)


def generate_profile(
    source_path: Path,
    parsed_data: dict[str, Any],
    analyzed_data: dict[str, Any],
) -> dict[str, Any]:
    """Generate profile.json with repository metadata and statistics.

    Args:
        source_path: Path to source repository
        parsed_data: Parsed code structure
        analyzed_data: Analysis results

    Returns:
        Profile dictionary
    """
    logger.info("Generating profile.json")

    # Count files by language
    language_counts = {}
    for lang, files in parsed_data.get("languages", {}).items():
        language_counts[lang] = len(files)

    profile = {
        "name": source_path.name,
        "path": str(source_path),
        "generated_at": datetime.utcnow().isoformat(),
        "statistics": {
            "total_files": len(parsed_data.get("files", [])),
            "total_functions": len(parsed_data.get("functions", [])),
            "total_classes": len(parsed_data.get("classes", [])),
            "languages": language_counts,
        },
        "structure": {
            "functions": parsed_data.get("functions", [])[:100],  # Limit for size
            "classes": parsed_data.get("classes", [])[:100],
        },
        "metadata": {
            "parser_version": "0.1.0",
            "digest_version": "0.1.0",
        },
    }

    return profile


def write_profile(profile: dict[str, Any], output_path: Path) -> None:
    """Write profile.json to file.

    Args:
        profile: Profile dictionary
        output_path: Output file path
    """
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(profile, f, indent=2, ensure_ascii=False)
    logger.info(f"Profile written to {output_path}")

