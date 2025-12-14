"""NOA Digest CLI.

Command-line interface for the digest pipeline.
"""

import typer
from pathlib import Path
from rich.console import Console

app = typer.Typer(
    name="noa-digest",
    help="NOA Digest Everything Pipeline",
    no_args_is_help=True,
)
console = Console()


@app.command()
def analyze(
    path: Path = typer.Argument(..., help="Path to repository or directory"),
    output: Path = typer.Option(Path("./digest-output"), help="Output directory"),
    format: str = typer.Option("all", help="Output format: all, json, markdown"),
) -> None:
    """Analyze a codebase and generate digest artifacts."""
    console.print(f"[bold blue]NOA Digest[/bold blue] v0.1.0")
    console.print(f"Analyzing: {path}")
    console.print(f"Output: {output}")

    # TODO: Implement digest pipeline
    console.print("[yellow]Pipeline not yet implemented[/yellow]")


@app.command()
def sbom(
    path: Path = typer.Argument(..., help="Path to repository"),
    format: str = typer.Option("cyclonedx", help="SBOM format: cyclonedx, spdx"),
) -> None:
    """Generate Software Bill of Materials."""
    console.print(f"Generating SBOM for: {path}")
    console.print(f"Format: {format}")

    # TODO: Implement SBOM generation
    console.print("[yellow]SBOM generation not yet implemented[/yellow]")


@app.command()
def security(
    path: Path = typer.Argument(..., help="Path to repository"),
) -> None:
    """Run security analysis on codebase."""
    console.print(f"Running security scan on: {path}")

    # TODO: Implement security scanning
    console.print("[yellow]Security scanning not yet implemented[/yellow]")


if __name__ == "__main__":
    app()

