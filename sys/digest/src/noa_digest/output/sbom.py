"""SBOM Generator with Syft.

T180: Generate SBOM with Syft
§3.4: Digest Everything Pipeline
US4: Digest Everything Pipeline
"""

import json
import subprocess
from pathlib import Path
from typing import Dict, Any, Optional
import logging

logger = logging.getLogger(__name__)


def generate_sbom(
    source_path: Path,
    format: str = "cyclonedx",
) -> Dict[str, Any]:
    """Generate Software Bill of Materials.

    Args:
        source_path: Path to source repository
        format: SBOM format (cyclonedx or spdx)

    Returns:
        SBOM dictionary
    """
    logger.info(f"Generating SBOM in {format} format for {source_path}")

    # TODO: Use Syft to generate SBOM
    # syft <source_path> -o cyclonedx-json > sbom.json
    # or
    # syft <source_path> -o spdx-json > sbom.json

    try:
        # Try to run syft if available
        output_format = "cyclonedx-json" if format == "cyclonedx" else "spdx-json"
        result = subprocess.run(
            ["syft", str(source_path), "-o", output_format],
            capture_output=True,
            text=True,
            check=False,
        )

        if result.returncode == 0:
            return json.loads(result.stdout)
        else:
            logger.warning(f"Syft not available or failed: {result.stderr}")
            # Return minimal SBOM structure
            return _generate_minimal_sbom(source_path, format)

    except FileNotFoundError:
        logger.warning("Syft not found, generating minimal SBOM")
        return _generate_minimal_sbom(source_path, format)
    except json.JSONDecodeError as e:
        logger.error(f"Failed to parse Syft output: {e}")
        return _generate_minimal_sbom(source_path, format)


def _generate_minimal_sbom(source_path: Path, format: str) -> Dict[str, Any]:
    """Generate a minimal SBOM structure when Syft is not available.

    Args:
        source_path: Path to source repository
        format: SBOM format

    Returns:
        Minimal SBOM dictionary
    """
    if format == "cyclonedx":
        return {
            "bomFormat": "CycloneDX",
            "specVersion": "1.5",
            "version": 1,
            "metadata": {
                "timestamp": "",
                "tools": [{"name": "noa-digest", "version": "0.1.0"}],
                "component": {
                    "type": "application",
                    "name": source_path.name,
                    "bom-ref": str(source_path),
                },
            },
            "components": [],
        }
    else:  # SPDX
        return {
            "spdxVersion": "SPDX-2.3",
            "dataLicense": "CC0-1.0",
            "SPDXID": "SPDXRef-DOCUMENT",
            "name": f"{source_path.name} SBOM",
            "documentNamespace": f"https://noa.dev/sbom/{source_path.name}",
            "packages": [],
        }


def write_sbom(sbom: Dict[str, Any], output_path: Path) -> None:
    """Write SBOM to file.

    Args:
        sbom: SBOM dictionary
        output_path: Output file path
    """
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(sbom, f, indent=2, ensure_ascii=False)
    logger.info(f"SBOM written to {output_path}")

