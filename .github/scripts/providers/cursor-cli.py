#!/usr/bin/env python3
"""
FlexNetOS Cursor CLI Provider
Integration with Cursor CLI for AI-powered code operations in CI/CD

References:
- https://cursor.com/docs/cli/headless
- https://cursor.com/docs/cli/github-actions
- https://cursor.com/docs/cli/cookbook/fix-ci
"""

import os
import json
import subprocess
import logging
from typing import Optional, Dict, Any, List
from dataclasses import dataclass
from pathlib import Path
from enum import Enum

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class OutputFormat(Enum):
    TEXT = "text"
    JSON = "json"
    STREAM_JSON = "stream-json"


@dataclass
class CursorConfig:
    """Configuration for Cursor CLI provider"""
    api_key: Optional[str] = None
    model: str = "gpt-4"  # or gpt-5, claude-3.5-sonnet, etc.
    output_format: OutputFormat = OutputFormat.JSON
    force_apply: bool = False  # --force flag for file modifications
    timeout_seconds: int = 300
    working_directory: Optional[str] = None


class CursorCLIProvider:
    """
    Cursor CLI Provider for FlexNetOS Automation

    Features:
    - Headless code analysis and generation
    - Automated code review
    - CI fix automation
    - File modification in scripts

    References:
    - https://cursor.com/docs/cli/headless
    - https://cursor.com/docs/cli/github-actions
    """

    def __init__(self, config: Optional[CursorConfig] = None):
        self.config = config or CursorConfig()
        self.authenticated = False
        self.cli_path = "cursor-agent"

        # Load API key from environment if not provided
        if not self.config.api_key:
            self.config.api_key = os.environ.get('CURSOR_API_KEY')

    def authenticate(self) -> bool:
        """Verify Cursor CLI is installed and API key is set"""
        if not self.config.api_key:
            logger.error("CURSOR_API_KEY not set")
            return False

        try:
            # Check if cursor-agent is installed
            result = subprocess.run(
                [self.cli_path, '--version'],
                capture_output=True,
                text=True,
                timeout=10
            )

            if result.returncode == 0:
                self.authenticated = True
                logger.info(f"✅ Cursor CLI authenticated: {result.stdout.strip()}")
                return True
            else:
                logger.warning("Cursor CLI not found, attempting installation...")
                return self._install_cli()

        except FileNotFoundError:
            logger.warning("Cursor CLI not found, attempting installation...")
            return self._install_cli()
        except Exception as e:
            logger.error(f"Cursor CLI check failed: {e}")
            return False

    def _install_cli(self) -> bool:
        """Install Cursor CLI"""
        try:
            # Install Cursor CLI
            result = subprocess.run(
                ['bash', '-c', 'curl https://cursor.com/install -fsS | bash'],
                capture_output=True,
                text=True,
                timeout=120
            )

            if result.returncode == 0:
                # Add to PATH
                home = os.path.expanduser("~")
                cli_dir = os.path.join(home, ".cursor", "bin")
                os.environ["PATH"] = f"{cli_dir}:{os.environ.get('PATH', '')}"

                self.authenticated = True
                logger.info("✅ Cursor CLI installed successfully")
                return True
            else:
                logger.error(f"Cursor CLI installation failed: {result.stderr}")
                return False

        except Exception as e:
            logger.error(f"Cursor CLI installation error: {e}")
            return False

    def _build_command(
        self,
        prompt: str,
        print_mode: bool = True,
        force: bool = False,
        output_format: Optional[OutputFormat] = None
    ) -> List[str]:
        """Build cursor-agent command"""
        cmd = [self.cli_path]

        if print_mode:
            cmd.append('-p')  # Print mode for non-interactive

        if force or self.config.force_apply:
            cmd.append('--force')  # Allow file modifications

        fmt = output_format or self.config.output_format
        cmd.extend(['--output-format', fmt.value])

        if self.config.model:
            cmd.extend(['--model', self.config.model])

        cmd.append(prompt)

        return cmd

    async def analyze_code(self, prompt: str) -> Dict[str, Any]:
        """Run code analysis using Cursor CLI"""
        import asyncio

        if not self.authenticated:
            raise Exception("Not authenticated")

        try:
            cmd = self._build_command(
                prompt,
                print_mode=True,
                output_format=OutputFormat.JSON
            )

            env = os.environ.copy()
            env['CURSOR_API_KEY'] = self.config.api_key

            # Run subprocess in thread pool to avoid blocking event loop
            result = await asyncio.to_thread(
                subprocess.run,
                cmd,
                capture_output=True,
                text=True,
                timeout=self.config.timeout_seconds,
                env=env,
                cwd=self.config.working_directory
            )

            if result.returncode == 0:
                try:
                    return {
                        "success": True,
                        "result": json.loads(result.stdout),
                        "raw": result.stdout
                    }
                except json.JSONDecodeError:
                    return {
                        "success": True,
                        "result": result.stdout,
                        "raw": result.stdout
                    }
            else:
                return {
                    "success": False,
                    "error": result.stderr,
                    "exit_code": result.returncode
                }

        except subprocess.TimeoutExpired:
            return {
                "success": False,
                "error": "Command timed out"
            }
        except Exception as e:
            logger.error(f"Cursor CLI execution failed: {e}")
            return {
                "success": False,
                "error": str(e)
            }

    async def fix_code(self, issue: Dict[str, Any], apply: bool = True) -> Dict[str, Any]:
        """Generate and optionally apply a code fix"""
        if not self.authenticated:
            raise Exception("Not authenticated")

        prompt = f"""Fix this issue:

Category: {issue.get('category', 'unknown')}
Severity: {issue.get('severity', 'unknown')}
File: {issue.get('file', 'unknown')}
Line: {issue.get('line', 'unknown')}
Message: {issue.get('message', '')}

Apply the fix directly to the file."""

        try:
            cmd = self._build_command(
                prompt,
                print_mode=True,
                force=apply,
                output_format=OutputFormat.JSON
            )

            env = os.environ.copy()
            env['CURSOR_API_KEY'] = self.config.api_key

            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=self.config.timeout_seconds,
                env=env,
                cwd=self.config.working_directory
            )

            return {
                "success": result.returncode == 0,
                "applied": apply and result.returncode == 0,
                "output": result.stdout,
                "error": result.stderr if result.returncode != 0 else None
            }

        except Exception as e:
            logger.error(f"Fix application failed: {e}")
            return {
                "success": False,
                "error": str(e)
            }

    async def review_pr(self, pr_number: int) -> Dict[str, Any]:
        """Review a pull request using Cursor CLI"""
        if not self.authenticated:
            raise Exception("Not authenticated")

        prompt = f"""Review the changes in PR #{pr_number}. Analyze:
1. Code quality and readability
2. Potential bugs or issues
3. Security considerations
4. Best practices compliance

Provide specific suggestions for improvement."""

        return await self.analyze_code(prompt)

    async def fix_ci(self, error_log: str) -> Dict[str, Any]:
        """
        Fix CI failures using Cursor CLI

        Reference: https://cursor.com/docs/cli/cookbook/fix-ci
        """
        if not self.authenticated:
            raise Exception("Not authenticated")

        prompt = f"""A CI workflow has failed with the following error:

```
{error_log}
```

Analyze the error and fix the issue. Apply the fix directly to the relevant files."""

        return await self.fix_code(
            {"category": "ci", "severity": "high", "message": error_log},
            apply=True
        )

    async def batch_process(self, files: List[str], operation: str) -> List[Dict[str, Any]]:
        """Process multiple files with Cursor CLI"""
        results = []

        for file in files:
            prompt = f"{operation}: {file}"
            result = await self.analyze_code(prompt)
            result["file"] = file
            results.append(result)

        return results

    def stream_analysis(self, prompt: str):
        """Stream analysis results in real-time"""
        if not self.authenticated:
            raise Exception("Not authenticated")

        cmd = self._build_command(
            prompt,
            print_mode=True,
            output_format=OutputFormat.STREAM_JSON
        )
        cmd.append('--stream-partial-output')

        env = os.environ.copy()
        env['CURSOR_API_KEY'] = self.config.api_key

        process = subprocess.Popen(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env=env,
            cwd=self.config.working_directory
        )

        for line in process.stdout:
            try:
                data = json.loads(line)
                yield data
            except json.JSONDecodeError:
                yield {"type": "raw", "content": line}

        process.wait()


def create_github_workflow() -> str:
    """
    Generate GitHub Actions workflow for Cursor CLI integration

    Reference: https://cursor.com/docs/cli/github-actions
    """
    return """# Cursor CLI Integration for FlexNetOS
# References:
# - https://cursor.com/docs/cli/github-actions
# - https://cursor.com/docs/cli/cookbook/fix-ci

name: Cursor AI Code Operations

on:
  pull_request:
    types: [opened, synchronize]
  workflow_run:
    workflows: ["CI"]
    types: [completed]

permissions:
  contents: write
  pull-requests: write
  issues: write

jobs:
  # Code Review on PR
  review:
    if: github.event_name == 'pull_request'
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install Cursor CLI
        run: |
          curl https://cursor.com/install -fsS | bash
          echo "$HOME/.cursor/bin" >> $GITHUB_PATH

      - name: Run Code Review
        env:
          CURSOR_API_KEY: ${{ secrets.CURSOR_API_KEY }}
        run: |
          cursor-agent -p --output-format json \\
            "Review the changes in this PR and provide feedback on code quality, potential bugs, and best practices" \\
            > review.json

      - name: Post Review Comment
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const review = JSON.parse(fs.readFileSync('review.json', 'utf8'));

            await github.rest.issues.createComment({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: context.issue.number,
              body: `## 🤖 AI Code Review\\n\\n${review.result || review}`
            });

  # Fix CI Failures
  fix-ci:
    if: github.event_name == 'workflow_run' && github.event.workflow_run.conclusion == 'failure'
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          ref: ${{ github.event.workflow_run.head_branch }}
          token: ${{ secrets.FLEXNETOS_BOT_TOKEN }}

      - name: Install Cursor CLI
        run: |
          curl https://cursor.com/install -fsS | bash
          echo "$HOME/.cursor/bin" >> $GITHUB_PATH

      - name: Get Failure Logs
        id: logs
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          gh run view ${{ github.event.workflow_run.id }} --log-failed > failure.log

      - name: Fix CI Issues
        env:
          CURSOR_API_KEY: ${{ secrets.CURSOR_API_KEY }}
        run: |
          cursor-agent -p --force --output-format text \\
            "A CI workflow has failed. Here are the error logs:

            $(cat failure.log)

            Analyze the errors and fix the issues in the codebase."

      - name: Commit Fixes
        run: |
          git config user.name "FlexNetOS Bot"
          git config user.email "bot@flexnetos.dev"

          if git diff --quiet; then
            echo "No changes to commit"
          else
            git add -A
            git commit -m "🤖 Auto-fix: CI failure resolution [skip ci]"
            git push
          fi
"""


def create_permissions_config() -> str:
    """
    Generate permissions configuration for Cursor CLI

    Reference: https://cursor.com/docs/cli/reference/permissions
    """
    return json.dumps({
        "permissions": {
            "allow": [
                "Read(**/*)",
                "Write(src/**/*)",
                "Write(tests/**/*)",
                "Write(docs/**/*)",
                "Shell(npm)",
                "Shell(npx)",
                "Shell(git status)",
                "Shell(git diff)",
                "Shell(eslint)",
                "Shell(prettier)"
            ],
            "deny": [
                "Shell(rm -rf)",
                "Shell(git push --force)",
                "Write(.env*)",
                "Write(*.key)",
                "Write(*.pem)",
                "Write(secrets/**/*)"
            ]
        }
    }, indent=2)


if __name__ == "__main__":
    import argparse
    import asyncio

    parser = argparse.ArgumentParser(description='Cursor CLI Provider')
    parser.add_argument('--analyze', type=str, help='Run analysis with prompt')
    parser.add_argument('--fix-ci', type=str, help='Fix CI with error log file')
    parser.add_argument('--workflow', action='store_true', help='Generate GitHub workflow')
    parser.add_argument('--permissions', action='store_true', help='Generate permissions config')

    args = parser.parse_args()

    if args.workflow:
        print(create_github_workflow())
    elif args.permissions:
        print(create_permissions_config())
    else:
        provider = CursorCLIProvider()
        if provider.authenticate():
            if args.analyze:
                result = asyncio.run(provider.analyze_code(args.analyze))
                print(json.dumps(result, indent=2))
            elif args.fix_ci:
                with open(args.fix_ci) as f:
                    error_log = f.read()
                result = asyncio.run(provider.fix_ci(error_log))
                print(json.dumps(result, indent=2))

