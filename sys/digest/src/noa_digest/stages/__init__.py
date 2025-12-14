"""Digest Pipeline Stages.

The 7-stage digest pipeline:
1. Discover - Identify sources to digest
2. Fetch - Clone or synchronize source material
3. Parse - Use language-specific parsers
4. Analyze - Generate embeddings and build knowledge graph
5. Summarize - Create summaries and documentation
6. Surface - Generate output artifacts
7. Secure - Security scanning (SBOM, vulnerabilities, secrets)
"""

from .analyze import AnalyzeStage
from .discover import DiscoverStage
from .fetch import FetchStage
from .parse import ParseStage
from .secure import SecureStage
from .summarize import SummarizeStage
from .surface import SurfaceStage

__all__ = [
    "DiscoverStage",
    "FetchStage",
    "ParseStage",
    "AnalyzeStage",
    "SummarizeStage",
    "SurfaceStage",
    "SecureStage",
]


