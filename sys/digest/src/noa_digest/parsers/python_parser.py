"""Python AST Parser.

T172: Implement Python AST parser
§3.4: Digest Everything Pipeline
US4: Digest Everything Pipeline
"""

import ast
from pathlib import Path
from typing import , Any
import logging

logger = logging.getLogger(__name__)


class PythonParser:
    """Parse Python code using AST."""

    def __init__(self):
        """Initialize Python parser."""
        pass

    def parse_file(self, file_path: Path) -> dict[str, Any]:
        """Parse a Python file.

        Args:
            file_path: Path to Python file

        Returns:
            Parsed structure with functions, classes, imports, etc.
        """
        logger.info(f"Parsing Python file: {file_path}")

        try:
            with open(file_path, "r", encoding="utf-8") as f:
                source = f.read()

            tree = ast.parse(source, filename=str(file_path))

            result = {
                "file": str(file_path),
                "functions": [],
                "classes": [],
                "imports": [],
                "variables": [],
            }

            for node in ast.walk(tree):
                if isinstance(node, ast.FunctionDef):
                    result["functions"].append({
                        "name": node.name,
                        "line": node.lineno,
                        "args": [arg.arg for arg in node.args.args],
                        "decorators": [ast.unparse(d) for d in node.decorator_list],
                    })
                elif isinstance(node, ast.ClassDef):
                    result["classes"].append({
                        "name": node.name,
                        "line": node.lineno,
                        "bases": [ast.unparse(b) for b in node.bases],
                        "methods": [n.name for n in node.body if isinstance(n, ast.FunctionDef)],
                    })
                elif isinstance(node, (ast.Import, ast.ImportFrom)):
                    if isinstance(node, ast.Import):
                        result["imports"].extend([alias.name for alias in node.names])
                    else:
                        result["imports"].append(f"from {node.module}")

            return result

        except SyntaxError as e:
            logger.error(f"Syntax error in {file_path}: {e}")
            return {
                "file": str(file_path),
                "error": str(e),
                "functions": [],
                "classes": [],
                "imports": [],
            }
        except Exception as e:
            logger.error(f"Error parsing {file_path}: {e}")
            return {
                "file": str(file_path),
                "error": str(e),
                "functions": [],
                "classes": [],
                "imports": [],
            }

    def parse_directory(self, directory: Path) -> list[dict[str, Any]]:
        """Parse all Python files in a directory.

        Args:
            directory: Directory to parse

        Returns:
            List of parsed file structures
        """
        results = []
        for py_file in directory.rglob("*.py"):
            if "__pycache__" not in str(py_file):
                results.append(self.parse_file(py_file))
        return results

