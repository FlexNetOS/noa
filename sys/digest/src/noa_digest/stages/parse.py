"""Parse Stage (Stage 3).

T167: Implement Parse stage with tree-sitter
§3.4: Digest Everything Pipeline - Stage 3: Parse
US4: Digest Everything Pipeline
"""

from pathlib import Path
from typing import , Any
import logging

from ..parsers import (
    PythonParser,
    TypeScriptParser,
    RustParser,
    GoParser,
    JavaParser,
)

logger = logging.getLogger(__name__)


class ParseStage:
    """Parse code using language-specific parsers."""

    def __init__(self):
        """Initialize parse stage."""
        self.parsers = {
            "python": PythonParser(),
            "typescript": TypeScriptParser(),
            "rust": RustParser(),
            "go": GoParser(),
            "java": JavaParser(),
        }

    def parse(self, source_path: Path) -> dict[str, Any]:
        """Parse source code.

        Args:
            source_path: Path to source code directory

        Returns:
            Parsed code structure with AST, functions, classes, etc.
        """
        logger.info(f"Parsing source: {source_path}")

        results = {
            "files": [],
            "functions": [],
            "classes": [],
            "modules": [],
            "dependencies": [],
            "languages": {},
        }

        # Detect languages and parse accordingly
        if not source_path.exists():
            logger.warning(f"Source path does not exist: {source_path}")
            return results

        # Parse Python files
        if any(source_path.rglob("*.py")):
            python_results = self.parsers["python"].parse_directory(source_path)
            results["languages"]["python"] = python_results
            for file_result in python_results:
                results["files"].append(file_result["file"])
                results["functions"].extend(file_result.get("functions", []))
                results["classes"].extend(file_result.get("classes", []))

        # Parse TypeScript/JavaScript files
        if any(source_path.rglob("*.ts")) or any(source_path.rglob("*.js")):
            ts_results = self.parsers["typescript"].parse_directory(source_path)
            results["languages"]["typescript"] = ts_results
            for file_result in ts_results:
                results["files"].append(file_result["file"])
                results["functions"].extend(file_result.get("functions", []))
                results["classes"].extend(file_result.get("classes", []))

        # Parse Rust files
        if any(source_path.rglob("*.rs")):
            rust_results = self.parsers["rust"].parse_directory(source_path)
            results["languages"]["rust"] = rust_results
            for file_result in rust_results:
                results["files"].append(file_result["file"])
                results["functions"].extend(file_result.get("functions", []))
                results["structs"] = results.get("structs", [])
                results["structs"].extend(file_result.get("structs", []))

        # Parse Go files
        if any(source_path.rglob("*.go")):
            go_results = self.parsers["go"].parse_directory(source_path)
            results["languages"]["go"] = go_results
            for file_result in go_results:
                results["files"].append(file_result["file"])
                results["functions"].extend(file_result.get("functions", []))

        # Parse Java files
        if any(source_path.rglob("*.java")):
            java_results = self.parsers["java"].parse_directory(source_path)
            results["languages"]["java"] = java_results
            for file_result in java_results:
                results["files"].append(file_result["file"])
                results["classes"].extend(file_result.get("classes", []))

        return results


