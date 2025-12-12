"""Surface Stage (Stage 6).

T170: Implement Surface stage for output generation
§3.4: Digest Everything Pipeline - Stage 6: Surface
US4: Digest Everything Pipeline
"""

from pathlib import Path
from typing import , Any
import logging

from ..output import (
    generate_profile,
    write_profile,
    generate_system_card,
    write_system_card,
    generate_knowledge_graph,
    write_knowledge_graph,
    generate_sbom,
    write_sbom,
)

logger = logging.getLogger(__name__)


class SurfaceStage:
    """Generate output artifacts (profile.json, system_card.md, kg.json, SBOM)."""

    def __init__(self, output_dir: Path):
        """Initialize surface stage.

        Args:
            output_dir: Directory for output artifacts
        """
        self.output_dir = output_dir
        self.output_dir.mkdir(parents=True, exist_ok=True)

    def surface(
        self,
        source_path: Path,
        parsed_data: dict[str, Any],
        analyzed_data: dict[str, Any],
    ) -> dict[str, Path]:
        """Generate output artifacts.

        Args:
            source_path: Path to source repository
            parsed_data: Parsed code structure
            analyzed_data: Analysis results

        Returns:
            Dictionary mapping artifact names to file paths
        """
        logger.info(f"Generating output artifacts in {self.output_dir}")

        artifacts = {}

        # Generate profile.json
        profile = generate_profile(source_path, parsed_data, analyzed_data)
        profile_path = self.output_dir / "profile.json"
        write_profile(profile, profile_path)
        artifacts["profile"] = profile_path

        # Generate system_card.md
        system_card_content = generate_system_card(
            source_path, parsed_data, analyzed_data, profile
        )
        system_card_path = self.output_dir / "system_card.md"
        write_system_card(system_card_content, system_card_path)
        artifacts["system_card"] = system_card_path

        # Generate kg.json
        kg = generate_knowledge_graph(parsed_data, analyzed_data)
        kg_path = self.output_dir / "kg.json"
        write_knowledge_graph(kg, kg_path)
        artifacts["knowledge_graph"] = kg_path

        # Generate SBOM
        sbom = generate_sbom(source_path, format="cyclonedx")
        sbom_path = self.output_dir / "sbom.json"
        write_sbom(sbom, sbom_path)
        artifacts["sbom"] = sbom_path

        return artifacts


