#!/usr/bin/env python3
"""
FlexNetOS GitHub App Authentication
Creates and manages GitHub App for organization-wide automation

References:
- https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/about-creating-github-apps
- https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
"""

import os
import json
import time
import jwt
import logging
from typing import Optional, Dict, Any, List
from dataclasses import dataclass
from datetime import datetime, timedelta
from pathlib import Path

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


@dataclass
class GitHubAppConfig:
    """Configuration for GitHub App"""
    app_id: str
    app_name: str = "FlexNetOS Automation"
    private_key_path: Optional[str] = None
    private_key: Optional[str] = None
    client_id: Optional[str] = None
    client_secret: Optional[str] = None
    webhook_secret: Optional[str] = None
    homepage_url: str = "https://flexnetos.dev"
    callback_url: str = "https://flexnetos.dev/auth/callback"

    # Permissions
    permissions: Dict[str, str] = None

    def __post_init__(self):
        if self.permissions is None:
            self.permissions = {
                "contents": "write",
                "issues": "write",
                "pull_requests": "write",
                "workflows": "write",
                "checks": "write",
                "actions": "read",
                "metadata": "read",
                "security_events": "read"
            }


class GitHubApp:
    """
    GitHub App for FlexNetOS Organization Automation

    This provides:
    - Organization-wide access without personal tokens
    - Fine-grained permissions
    - Higher rate limits (5000 + 5000 per installation)
    - Webhook event handling
    - JWT-based authentication

    Reference: https://docs.github.com/en/apps/creating-github-apps/about-creating-github-apps/about-creating-github-apps
    """

    def __init__(self, config: GitHubAppConfig):
        self.config = config
        self.jwt_token: Optional[str] = None
        self.jwt_expiry: Optional[datetime] = None
        self.installation_tokens: Dict[int, Dict[str, Any]] = {}

        # Load private key
        if config.private_key_path:
            with open(config.private_key_path) as f:
                self._private_key = f.read()
        elif config.private_key:
            self._private_key = config.private_key
        else:
            self._private_key = os.environ.get('GITHUB_APP_PRIVATE_KEY', '')

    def generate_jwt(self) -> str:
        """Generate JWT for GitHub App authentication"""
        now = datetime.utcnow()

        # Check if we have a valid JWT
        if self.jwt_token and self.jwt_expiry and now < self.jwt_expiry:
            return self.jwt_token

        # Generate new JWT (valid for 10 minutes, request 9 to be safe)
        payload = {
            'iat': int(now.timestamp()),
            'exp': int((now + timedelta(minutes=9)).timestamp()),
            'iss': self.config.app_id
        }

        self.jwt_token = jwt.encode(
            payload,
            self._private_key,
            algorithm='RS256'
        )
        self.jwt_expiry = now + timedelta(minutes=9)

        logger.info("Generated new GitHub App JWT")
        return self.jwt_token

    def get_installation_token(self, installation_id: int) -> str:
        """Get installation access token for a specific installation"""
        import requests

        # Check cache
        cached = self.installation_tokens.get(installation_id)
        if cached and datetime.fromisoformat(cached['expires_at'].replace('Z', '')) > datetime.utcnow():
            return cached['token']

        # Request new token
        jwt_token = self.generate_jwt()

        response = requests.post(
            f"https://api.github.com/app/installations/{installation_id}/access_tokens",
            headers={
                'Authorization': f'Bearer {jwt_token}',
                'Accept': 'application/vnd.github+json',
                'X-GitHub-Api-Version': '2022-11-28'
            }
        )

        if response.status_code == 201:
            data = response.json()
            self.installation_tokens[installation_id] = data
            logger.info(f"Got installation token for {installation_id}")
            return data['token']
        else:
            raise Exception(f"Failed to get installation token: {response.text}")

    def list_installations(self) -> List[Dict[str, Any]]:
        """List all installations of this GitHub App"""
        import requests

        jwt_token = self.generate_jwt()

        response = requests.get(
            "https://api.github.com/app/installations",
            headers={
                'Authorization': f'Bearer {jwt_token}',
                'Accept': 'application/vnd.github+json',
                'X-GitHub-Api-Version': '2022-11-28'
            }
        )

        if response.status_code == 200:
            return response.json()
        else:
            raise Exception(f"Failed to list installations: {response.text}")

    def get_installation_for_org(self, org: str) -> Optional[int]:
        """Get installation ID for an organization"""
        installations = self.list_installations()

        for installation in installations:
            if installation.get('account', {}).get('login') == org:
                return installation['id']

        return None

    @staticmethod
    def generate_app_manifest(config: GitHubAppConfig) -> Dict[str, Any]:
        """
        Generate GitHub App manifest for creation

        Use this manifest at: https://github.com/settings/apps/new
        """
        return {
            "name": config.app_name,
            "url": config.homepage_url,
            "hook_attributes": {
                "url": f"{config.homepage_url}/webhooks/github"
            },
            "redirect_url": config.callback_url,
            "callback_urls": [config.callback_url],
            "setup_url": f"{config.homepage_url}/setup",
            "public": False,
            "default_permissions": config.permissions,
            "default_events": [
                "pull_request",
                "pull_request_review",
                "pull_request_review_comment",
                "issues",
                "issue_comment",
                "push",
                "check_run",
                "check_suite",
                "workflow_run"
            ]
        }


class GitHubOAuth:
    """
    GitHub OAuth App for User Authentication

    Use this for:
    - User-level authentication
    - Web application login
    - Personal access to repositories

    Reference: https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps
    """

    AUTHORIZE_URL = "https://github.com/login/oauth/authorize"
    TOKEN_URL = "https://github.com/login/oauth/access_token"
    API_URL = "https://api.github.com"

    def __init__(self, client_id: str, client_secret: str, redirect_uri: str):
        self.client_id = client_id
        self.client_secret = client_secret
        self.redirect_uri = redirect_uri

    def get_authorization_url(
        self,
        scopes: List[str] = None,
        state: Optional[str] = None
    ) -> str:
        """Generate OAuth authorization URL"""
        from urllib.parse import urlencode

        if scopes is None:
            scopes = [
                "repo",
                "workflow",
                "read:org",
                "read:user",
                "user:email"
            ]

        params = {
            'client_id': self.client_id,
            'redirect_uri': self.redirect_uri,
            'scope': ' '.join(scopes),
        }

        if state:
            params['state'] = state

        return f"{self.AUTHORIZE_URL}?{urlencode(params)}"

    def exchange_code_for_token(self, code: str) -> Dict[str, Any]:
        """Exchange authorization code for access token"""
        import requests

        response = requests.post(
            self.TOKEN_URL,
            data={
                'client_id': self.client_id,
                'client_secret': self.client_secret,
                'code': code,
                'redirect_uri': self.redirect_uri
            },
            headers={
                'Accept': 'application/json'
            }
        )

        if response.status_code == 200:
            return response.json()
        else:
            raise Exception(f"Token exchange failed: {response.text}")

    def get_user(self, access_token: str) -> Dict[str, Any]:
        """Get authenticated user information"""
        import requests

        response = requests.get(
            f"{self.API_URL}/user",
            headers={
                'Authorization': f'token {access_token}',
                'Accept': 'application/vnd.github+json'
            }
        )

        if response.status_code == 200:
            return response.json()
        else:
            raise Exception(f"Failed to get user: {response.text}")

    def refresh_token(self, refresh_token: str) -> Dict[str, Any]:
        """Refresh an access token (if enabled for the app)"""
        import requests

        response = requests.post(
            self.TOKEN_URL,
            data={
                'client_id': self.client_id,
                'client_secret': self.client_secret,
                'grant_type': 'refresh_token',
                'refresh_token': refresh_token
            },
            headers={
                'Accept': 'application/json'
            }
        )

        if response.status_code == 200:
            return response.json()
        else:
            raise Exception(f"Token refresh failed: {response.text}")


def generate_setup_instructions() -> str:
    """Generate instructions for setting up GitHub App"""
    return """
# FlexNetOS GitHub App Setup Guide

## Step 1: Create the GitHub App

1. Go to https://github.com/organizations/FlexNetOS/settings/apps/new
2. Fill in the following details:

**Basic Information:**
- GitHub App name: `FlexNetOS Automation`
- Homepage URL: `https://flexnetos.dev`
- Description: `Automated CI/CD for FlexNetOS organization`

**Webhook:**
- Webhook URL: `https://flexnetos.dev/webhooks/github`
- Webhook secret: [Generate a secure random string]

**Callback URLs:**
- `https://flexnetos.dev/auth/callback`

**Permissions:**
Repository permissions:
- Contents: Read and write
- Issues: Read and write
- Pull requests: Read and write
- Workflows: Read and write
- Checks: Read and write
- Actions: Read
- Metadata: Read
- Security events: Read

Organization permissions:
- Members: Read

**Subscribe to events:**
- Pull request
- Pull request review
- Pull request review comment
- Issues
- Issue comment
- Push
- Check run
- Check suite
- Workflow run

## Step 2: Generate Private Key

1. After creating the app, go to the app settings
2. Scroll to "Private keys"
3. Click "Generate a private key"
4. Save the downloaded .pem file securely

## Step 3: Install the App

1. Go to your app's page: `https://github.com/apps/flexnetos-automation`
2. Click "Install"
3. Select "FlexNetOS" organization
4. Choose "All repositories" or select specific repos
5. Click "Install"

## Step 4: Configure Secrets

Add these secrets to your organization:

```bash
# App ID (from app settings page)
gh secret set GITHUB_APP_ID --org FlexNetOS --body "your-app-id"

# Private key (contents of .pem file)
gh secret set GITHUB_APP_PRIVATE_KEY --org FlexNetOS --body "$(cat path/to/private-key.pem)"

# Client ID (for OAuth, from app settings)
gh secret set GITHUB_APP_CLIENT_ID --org FlexNetOS --body "your-client-id"

# Client secret (generate one in app settings)
gh secret set GITHUB_APP_CLIENT_SECRET --org FlexNetOS --body "your-client-secret"

# Webhook secret
gh secret set GITHUB_WEBHOOK_SECRET --org FlexNetOS --body "your-webhook-secret"
```

## Step 5: Update Workflows

Your workflows can now use the app for authentication:

```yaml
- name: Generate GitHub App Token
  id: app-token
  uses: actions/create-github-app-token@v1
  with:
    app-id: ${{ secrets.GITHUB_APP_ID }}
    private-key: ${{ secrets.GITHUB_APP_PRIVATE_KEY }}
    owner: FlexNetOS

- name: Use Token
  env:
    GH_TOKEN: ${{ steps.app-token.outputs.token }}
  run: |
    gh pr list
```

## Benefits of Using GitHub App

1. **Higher Rate Limits**: 5,000 requests/hour + 5,000 per installation
2. **Fine-grained Permissions**: Only request what you need
3. **Organization-wide**: Works across all repos without personal tokens
4. **Audit Trail**: Clear logging of app actions
5. **Revocable**: Easy to revoke access without affecting users
"""


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='GitHub App Management')
    parser.add_argument('--setup', action='store_true', help='Show setup instructions')
    parser.add_argument('--manifest', action='store_true', help='Generate app manifest')
    parser.add_argument('--test', action='store_true', help='Test authentication')
    parser.add_argument('--org', type=str, default='FlexNetOS', help='Organization name')

    args = parser.parse_args()

    if args.setup:
        print(generate_setup_instructions())
    elif args.manifest:
        config = GitHubAppConfig(
            app_id="",
            app_name="FlexNetOS Automation"
        )
        manifest = GitHubApp.generate_app_manifest(config)
        print(json.dumps(manifest, indent=2))
    elif args.test:
        app_id = os.environ.get('GITHUB_APP_ID')
        if not app_id:
            print("GITHUB_APP_ID not set")
            exit(1)

        config = GitHubAppConfig(app_id=app_id)
        app = GitHubApp(config)

        try:
            installations = app.list_installations()
            print(f"Found {len(installations)} installations")

            for inst in installations:
                print(f"  - {inst['account']['login']} (ID: {inst['id']})")

        except Exception as e:
            print(f"Error: {e}")

