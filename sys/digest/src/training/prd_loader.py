"""
PRD-based training data loader.

T379: Parses simple PRD markdown into training examples for DSPy modules.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List


@dataclass
class PRDExample:
    user_story: str
    acceptance_criteria: List[str]
    priority: str
    metadata: Dict[str, str]


def load_prd_samples(path: Path) -> List[PRDExample]:
    if not path.exists():
        # Return a minimal fallback dataset
        return [
            PRDExample(
                user_story="As a user, I want quick responses so I can stay productive.",
                acceptance_criteria=[
                    "p95 latency under 500ms",
                    "error rate below 1%",
                ],
                priority="P0",
                metadata={"source": "fallback"},
            )
        ]

    content = path.read_text(encoding="utf-8")
    blocks = [block.strip() for block in content.split("\n\n") if block.strip()]

    examples: List[PRDExample] = []
    for block in blocks:
        lines = [line.strip() for line in block.splitlines() if line.strip()]
        if not lines:
            continue

        user_story = lines[0]
        acceptance_criteria = [line.lstrip("- ").strip() for line in lines[1:] if line]
        examples.append(
            PRDExample(
                user_story=user_story,
                acceptance_criteria=acceptance_criteria or ["Define measurable acceptance."],
                priority="P1",
                metadata={"source": path.name},
            )
        )

    return examples
