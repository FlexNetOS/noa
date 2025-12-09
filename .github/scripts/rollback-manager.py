#!/usr/bin/env python3
"""
FlexNetOS Rollback Manager
Handles automatic rollback of failed automated changes

Features:
- Automatic rollback on deployment failure
- Automatic rollback on test regression
- Automatic rollback on security regression
- Manual rollback triggers
- Rollback PR creation
"""

import os
import json
import subprocess
import logging
from pathlib import Path
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, asdict
from datetime import datetime, timedelta
from enum import Enum

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class RollbackReason(Enum):
    DEPLOYMENT_FAILURE = "deployment_failure"
    TEST_REGRESSION = "test_regression"
    SECURITY_REGRESSION = "security_regression"
    MANUAL_TRIGGER = "manual_trigger"
    PERFORMANCE_REGRESSION = "performance_regression"
    ERROR_RATE_INCREASE = "error_rate_increase"


class RollbackStrategy(Enum):
    REVERT_COMMIT = "revert_commit"
    REVERT_PR = "revert_pr"
    RESTORE_BACKUP = "restore_backup"
    DEPLOY_PREVIOUS = "deploy_previous"


@dataclass
class RollbackConfig:
    """Configuration for rollback behavior"""
    enabled: bool = True
    auto_rollback_triggers: List[str] = None
    rollback_window_hours: int = 24
    create_rollback_pr: bool = True
    notify_on_rollback: bool = True
    require_approval: bool = False
    max_rollback_depth: int = 5  # Max commits to roll back

    def __post_init__(self):
        if self.auto_rollback_triggers is None:
            self.auto_rollback_triggers = [
                RollbackReason.DEPLOYMENT_FAILURE.value,
                RollbackReason.TEST_REGRESSION.value,
                RollbackReason.SECURITY_REGRESSION.value
            ]


@dataclass
class RollbackState:
    """Current rollback state"""
    repo: str
    last_known_good_commit: Optional[str] = None
    last_known_good_timestamp: Optional[str] = None
    pending_rollback: bool = False
    rollback_reason: Optional[str] = None
    rollback_target: Optional[str] = None
    rollback_initiated: Optional[str] = None
    rollback_completed: Optional[str] = None
    rollback_pr_number: Optional[int] = None


@dataclass
class DeploymentRecord:
    """Record of a deployment"""
    commit: str
    timestamp: str
    pr_number: Optional[int]
    status: str  # success, failure
    tests_passed: bool
    security_scan_passed: bool
    deployment_url: Optional[str]


class RollbackManager:
    """
    Manages automatic rollback of failed changes
    """

    def __init__(self, state_dir: str = ".github/state/rollback"):
        self.state_dir = Path(state_dir)
        self.state_dir.mkdir(parents=True, exist_ok=True)
        self.config = RollbackConfig()
        self.deployments: List[DeploymentRecord] = []

    def configure(self, config: RollbackConfig):
        """Configure rollback behavior"""
        self.config = config

    def _get_state_file(self, repo: str) -> Path:
        """Get state file path for a repo"""
        safe_repo = repo.replace("/", "_")
        return self.state_dir / f"{safe_repo}.json"

    def _load_state(self, repo: str) -> RollbackState:
        """Load state from disk"""
        state_file = self._get_state_file(repo)
        if state_file.exists():
            try:
                with open(state_file) as f:
                    data = json.load(f)
                return RollbackState(**data)
            except:
                pass
        return RollbackState(repo=repo)

    def _save_state(self, state: RollbackState):
        """Save state to disk"""
        state_file = self._get_state_file(state.repo)
        with open(state_file, 'w') as f:
            json.dump(asdict(state), f, indent=2)

    def _get_deployments_file(self, repo: str) -> Path:
        """Get deployments history file"""
        safe_repo = repo.replace("/", "_")
        return self.state_dir / f"{safe_repo}_deployments.json"

    def record_deployment(self, repo: str, record: DeploymentRecord):
        """Record a new deployment"""
        deployments_file = self._get_deployments_file(repo)

        # Load existing deployments
        deployments = []
        if deployments_file.exists():
            try:
                with open(deployments_file) as f:
                    deployments = json.load(f)
            except:
                pass

        # Add new deployment
        deployments.append(asdict(record))

        # Keep only recent deployments
        max_records = 100
        if len(deployments) > max_records:
            deployments = deployments[-max_records:]

        # Save
        with open(deployments_file, 'w') as f:
            json.dump(deployments, f, indent=2)

        # Update last known good if successful
        if record.status == "success" and record.tests_passed and record.security_scan_passed:
            state = self._load_state(repo)
            state.last_known_good_commit = record.commit
            state.last_known_good_timestamp = record.timestamp
            self._save_state(state)
            logger.info(f"Updated last known good commit: {record.commit[:8]}")

    def get_last_known_good(self, repo: str) -> Optional[str]:
        """Get last known good commit"""
        state = self._load_state(repo)
        return state.last_known_good_commit

    def should_rollback(self, repo: str, reason: RollbackReason) -> bool:
        """Check if rollback should be triggered"""
        if not self.config.enabled:
            return False

        if reason.value not in self.config.auto_rollback_triggers:
            return False

        state = self._load_state(repo)

        # Check if we have a known good state to roll back to
        if not state.last_known_good_commit:
            logger.warning(f"No known good commit for {repo}, cannot rollback")
            return False

        # Check rollback window
        if state.last_known_good_timestamp:
            last_good = datetime.fromisoformat(state.last_known_good_timestamp)
            window = timedelta(hours=self.config.rollback_window_hours)
            if datetime.utcnow() - last_good > window:
                logger.warning(f"Last known good commit is outside rollback window")
                return False

        return True

    def initiate_rollback(
        self,
        repo: str,
        reason: RollbackReason,
        target_commit: Optional[str] = None,
        strategy: RollbackStrategy = RollbackStrategy.REVERT_COMMIT
    ) -> Dict[str, Any]:
        """
        Initiate a rollback
        Returns rollback status and details
        """
        logger.info(f"Initiating rollback for {repo}")
        logger.info(f"  Reason: {reason.value}")
        logger.info(f"  Strategy: {strategy.value}")

        state = self._load_state(repo)

        # Determine target commit
        if not target_commit:
            target_commit = state.last_known_good_commit

        if not target_commit:
            return {
                "success": False,
                "error": "No target commit for rollback"
            }

        state.pending_rollback = True
        state.rollback_reason = reason.value
        state.rollback_target = target_commit
        state.rollback_initiated = datetime.utcnow().isoformat()
        self._save_state(state)

        # Execute rollback based on strategy
        result = None
        if strategy == RollbackStrategy.REVERT_COMMIT:
            result = self._revert_commit(repo, target_commit)
        elif strategy == RollbackStrategy.REVERT_PR:
            result = self._revert_to_commit(repo, target_commit)
        elif strategy == RollbackStrategy.RESTORE_BACKUP:
            result = self._restore_backup(repo, target_commit)
        elif strategy == RollbackStrategy.DEPLOY_PREVIOUS:
            result = self._deploy_previous(repo, target_commit)

        if result and result.get("success"):
            state.rollback_completed = datetime.utcnow().isoformat()
            state.pending_rollback = False
            if result.get("pr_number"):
                state.rollback_pr_number = result["pr_number"]
            self._save_state(state)

            # Notify if enabled
            if self.config.notify_on_rollback:
                self._notify_rollback(repo, reason, result)

        return result

    def _revert_commit(self, repo: str, target_commit: str) -> Dict[str, Any]:
        """Revert commits since target"""
        try:
            # Get current commit
            result = subprocess.run(
                ['git', 'rev-parse', 'HEAD'],
                capture_output=True, text=True, check=True
            )
            current_commit = result.stdout.strip()

            if current_commit == target_commit:
                return {"success": True, "message": "Already at target commit"}

            # Get commits to revert
            result = subprocess.run(
                ['git', 'rev-list', f'{target_commit}..HEAD'],
                capture_output=True, text=True, check=True
            )
            # Filter out empty strings to handle case where no commits to revert
            commits_to_revert = [c for c in result.stdout.strip().split('\n') if c]

            if len(commits_to_revert) > self.config.max_rollback_depth:
                return {
                    "success": False,
                    "error": f"Too many commits to revert ({len(commits_to_revert)} > {self.config.max_rollback_depth})"
                }

            # Create rollback branch
            rollback_branch = f"rollback/{datetime.utcnow().strftime('%Y%m%d-%H%M%S')}"
            subprocess.run(['git', 'checkout', '-b', rollback_branch], check=True)

            # Revert commits in reverse order
            for commit in commits_to_revert:
                if commit:
                    subprocess.run(
                        ['git', 'revert', '--no-commit', commit],
                        check=True
                    )

            # Commit the reverts
            subprocess.run([
                'git', 'commit', '-m',
                f'🔙 Rollback to {target_commit[:8]}\n\nAutomated rollback by FlexNetOS'
            ], check=True)

            # Push branch
            subprocess.run(['git', 'push', '-u', 'origin', rollback_branch], check=True)

            # Create PR if enabled
            pr_number = None
            if self.config.create_rollback_pr:
                pr_result = subprocess.run(
                    ['gh', 'pr', 'create',
                     '--title', f'🔙 Rollback to {target_commit[:8]}',
                     '--body', self._generate_rollback_pr_body(repo, target_commit, commits_to_revert),
                     '--base', 'main',
                     '--head', rollback_branch],
                    capture_output=True, text=True
                )
                if pr_result.returncode == 0:
                    # Extract PR number from output
                    import re
                    match = re.search(r'/pull/(\d+)', pr_result.stdout)
                    if match:
                        pr_number = int(match.group(1))

            return {
                "success": True,
                "rollback_branch": rollback_branch,
                "commits_reverted": len(commits_to_revert),
                "pr_number": pr_number
            }

        except subprocess.CalledProcessError as e:
            logger.error(f"Git operation failed: {e}")
            return {"success": False, "error": str(e)}
        except Exception as e:
            logger.error(f"Rollback failed: {e}")
            return {"success": False, "error": str(e)}

    def _revert_to_commit(self, repo: str, target_commit: str) -> Dict[str, Any]:
        """Hard revert to a specific commit"""
        try:
            # Create rollback branch from target
            rollback_branch = f"rollback/{datetime.utcnow().strftime('%Y%m%d-%H%M%S')}"
            subprocess.run(['git', 'checkout', '-b', rollback_branch, target_commit], check=True)
            subprocess.run(['git', 'push', '-u', 'origin', rollback_branch], check=True)

            # Create PR
            pr_number = None
            if self.config.create_rollback_pr:
                pr_result = subprocess.run(
                    ['gh', 'pr', 'create',
                     '--title', f'🔙 Restore to {target_commit[:8]}',
                     '--body', self._generate_rollback_pr_body(repo, target_commit, []),
                     '--base', 'main',
                     '--head', rollback_branch],
                    capture_output=True, text=True
                )
                if pr_result.returncode == 0:
                    import re
                    match = re.search(r'/pull/(\d+)', pr_result.stdout)
                    if match:
                        pr_number = int(match.group(1))

            return {
                "success": True,
                "rollback_branch": rollback_branch,
                "target_commit": target_commit,
                "pr_number": pr_number
            }

        except Exception as e:
            logger.error(f"Restore failed: {e}")
            return {"success": False, "error": str(e)}

    def _restore_backup(self, repo: str, target_commit: str) -> Dict[str, Any]:
        """Restore from backup (not implemented)"""
        return {"success": False, "error": "Backup restore not implemented"}

    def _deploy_previous(self, repo: str, target_commit: str) -> Dict[str, Any]:
        """Deploy previous version (not implemented)"""
        return {"success": False, "error": "Deploy previous not implemented"}

    def _generate_rollback_pr_body(
        self,
        repo: str,
        target_commit: str,
        reverted_commits: List[str]
    ) -> str:
        """Generate PR body for rollback"""
        state = self._load_state(repo)

        body = f"""## 🔙 Automated Rollback

**Repository:** {repo}
**Target Commit:** `{target_commit[:8]}`
**Reason:** {state.rollback_reason or 'Unknown'}
**Initiated:** {state.rollback_initiated}

---

### Reverted Changes

"""
        if reverted_commits:
            body += f"The following {len(reverted_commits)} commit(s) are being reverted:\n\n"
            for commit in reverted_commits[:10]:  # Limit display
                if commit:
                    body += f"- `{commit[:8]}`\n"
            if len(reverted_commits) > 10:
                body += f"- ... and {len(reverted_commits) - 10} more\n"
        else:
            body += "Restoring to previous known good state.\n"

        body += """
---

### ⚠️ Action Required

This PR was created automatically by the FlexNetOS rollback system.

- [ ] Review the changes
- [ ] Verify rollback addresses the issue
- [ ] Merge when ready

---

> 🤖 Generated by FlexNetOS Rollback Manager
"""
        return body

    def _notify_rollback(self, repo: str, reason: RollbackReason, result: Dict[str, Any]):
        """Send notification about rollback"""
        try:
            # Create a GitHub issue for notification
            title = f"🔙 Rollback Triggered: {reason.value}"
            body = f"""## Rollback Notification

**Repository:** {repo}
**Reason:** {reason.value}
**Status:** {'✅ Success' if result.get('success') else '❌ Failed'}

### Details

```json
{json.dumps(result, indent=2)}
```

---

> 🤖 Generated by FlexNetOS Rollback Manager
"""
            subprocess.run([
                'gh', 'issue', 'create',
                '--title', title,
                '--body', body,
                '--label', 'rollback,automated'
            ], check=True)

        except Exception as e:
            logger.warning(f"Failed to send rollback notification: {e}")

    def get_rollback_history(self, repo: str, limit: int = 10) -> List[Dict[str, Any]]:
        """Get rollback history for a repo"""
        history_file = self.state_dir / f"{repo.replace('/', '_')}_history.json"

        if history_file.exists():
            try:
                with open(history_file) as f:
                    history = json.load(f)
                return history[-limit:]
            except:
                pass

        return []


# Health check triggers for auto-rollback
class HealthChecker:
    """
    Monitors health metrics and triggers rollback if needed
    """

    def __init__(self, rollback_manager: RollbackManager):
        self.rollback_manager = rollback_manager

    def check_deployment_health(self, repo: str, deployment_url: str) -> bool:
        """Check if deployment is healthy"""
        try:
            import requests
            response = requests.get(f"{deployment_url}/health", timeout=10)
            return response.status_code == 200
        except:
            return False

    def check_test_health(self, repo: str) -> bool:
        """Check if tests are passing"""
        try:
            result = subprocess.run(
                ['npm', 'test', '--', '--passWithNoTests'],
                capture_output=True, check=True
            )
            return True
        except:
            return False

    def check_security_health(self, repo: str) -> bool:
        """Check for security regressions"""
        try:
            result = subprocess.run(
                ['npm', 'audit', '--audit-level=critical'],
                capture_output=True
            )
            return result.returncode == 0
        except:
            return True  # Assume healthy if check fails

    def run_health_checks(self, repo: str, deployment_url: Optional[str] = None) -> Dict[str, bool]:
        """Run all health checks"""
        results = {
            "tests": self.check_test_health(repo),
            "security": self.check_security_health(repo)
        }

        if deployment_url:
            results["deployment"] = self.check_deployment_health(repo, deployment_url)

        return results

    def trigger_rollback_if_needed(self, repo: str, health_results: Dict[str, bool]) -> Optional[Dict[str, Any]]:
        """Trigger rollback if health checks fail"""
        if not health_results.get("tests", True):
            if self.rollback_manager.should_rollback(repo, RollbackReason.TEST_REGRESSION):
                return self.rollback_manager.initiate_rollback(
                    repo, RollbackReason.TEST_REGRESSION
                )

        if not health_results.get("security", True):
            if self.rollback_manager.should_rollback(repo, RollbackReason.SECURITY_REGRESSION):
                return self.rollback_manager.initiate_rollback(
                    repo, RollbackReason.SECURITY_REGRESSION
                )

        if not health_results.get("deployment", True):
            if self.rollback_manager.should_rollback(repo, RollbackReason.DEPLOYMENT_FAILURE):
                return self.rollback_manager.initiate_rollback(
                    repo, RollbackReason.DEPLOYMENT_FAILURE
                )

        return None


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='FlexNetOS Rollback Manager')
    parser.add_argument('--repo', required=True, help='Repository name')
    parser.add_argument('--action', choices=['status', 'rollback', 'health-check'], required=True)
    parser.add_argument('--reason', choices=[r.value for r in RollbackReason], default='manual_trigger')
    parser.add_argument('--target', help='Target commit for rollback')

    args = parser.parse_args()

    manager = RollbackManager()

    if args.action == 'status':
        state = manager._load_state(args.repo)
        print(json.dumps(asdict(state), indent=2))

    elif args.action == 'rollback':
        reason = RollbackReason(args.reason)
        result = manager.initiate_rollback(args.repo, reason, args.target)
        print(json.dumps(result, indent=2))

    elif args.action == 'health-check':
        checker = HealthChecker(manager)
        health = checker.run_health_checks(args.repo)
        print(json.dumps(health, indent=2))

        rollback_result = checker.trigger_rollback_if_needed(args.repo, health)
        if rollback_result:
            print(f"\n⚠️ Rollback triggered!")
            print(json.dumps(rollback_result, indent=2))

