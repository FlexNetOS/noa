#!/usr/bin/env python3
"""
FlexNetOS Unified OAuth Connectors
Simple auth connectors for Google, Claude (Anthropic), OpenAI, GitHub, and more

These connectors enable:
- User account linking
- Service authentication
- Token management
- Cross-platform access
"""

import os
import json
import time
import secrets
import hashlib
import base64
import logging
from typing import Optional, Dict, Any, List, Callable
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from abc import ABC, abstractmethod
from urllib.parse import urlencode, parse_qs, urlparse
from pathlib import Path

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


@dataclass
class OAuthToken:
    """Represents an OAuth token"""
    access_token: str
    token_type: str = "Bearer"
    expires_in: Optional[int] = None
    refresh_token: Optional[str] = None
    scope: Optional[str] = None
    id_token: Optional[str] = None  # For OIDC
    created_at: str = field(default_factory=lambda: datetime.utcnow().isoformat())

    @property
    def is_expired(self) -> bool:
        if not self.expires_in:
            return False
        created = datetime.fromisoformat(self.created_at)
        return datetime.utcnow() > created + timedelta(seconds=self.expires_in - 60)

    def to_dict(self) -> Dict[str, Any]:
        return {
            'access_token': self.access_token,
            'token_type': self.token_type,
            'expires_in': self.expires_in,
            'refresh_token': self.refresh_token,
            'scope': self.scope,
            'id_token': self.id_token,
            'created_at': self.created_at
        }


@dataclass
class OAuthConfig:
    """Base OAuth configuration"""
    client_id: str
    client_secret: str
    redirect_uri: str
    scopes: List[str] = field(default_factory=list)


class OAuthConnector(ABC):
    """Base class for OAuth connectors"""

    PROVIDER_NAME = "base"
    AUTHORIZE_URL = ""
    TOKEN_URL = ""
    USERINFO_URL = ""

    def __init__(self, config: OAuthConfig):
        self.config = config
        self._state_store: Dict[str, Dict] = {}

    def generate_state(self, metadata: Optional[Dict] = None) -> str:
        """Generate CSRF state token"""
        state = secrets.token_urlsafe(32)
        self._state_store[state] = {
            'created_at': datetime.utcnow().isoformat(),
            'metadata': metadata or {}
        }
        return state

    def validate_state(self, state: str) -> bool:
        """Validate and consume state token"""
        if state in self._state_store:
            stored = self._state_store.pop(state)
            created = datetime.fromisoformat(stored['created_at'])
            # State valid for 10 minutes
            return datetime.utcnow() < created + timedelta(minutes=10)
        return False

    @abstractmethod
    def get_authorization_url(self, state: Optional[str] = None) -> str:
        """Generate authorization URL"""
        pass

    @abstractmethod
    def exchange_code(self, code: str) -> OAuthToken:
        """Exchange authorization code for tokens"""
        pass

    @abstractmethod
    def refresh_token(self, refresh_token: str) -> OAuthToken:
        """Refresh an access token"""
        pass

    @abstractmethod
    def get_user_info(self, token: OAuthToken) -> Dict[str, Any]:
        """Get authenticated user information"""
        pass

    def _make_request(
        self,
        method: str,
        url: str,
        headers: Dict = None,
        data: Dict = None,
        json_data: Dict = None
    ) -> Dict[str, Any]:
        """Make HTTP request"""
        import requests

        response = requests.request(
            method,
            url,
            headers=headers,
            data=data,
            json=json_data
        )

        if response.status_code >= 400:
            raise Exception(f"Request failed: {response.status_code} - {response.text}")

        return response.json()


class GoogleOAuthConnector(OAuthConnector):
    """
    Google OAuth 2.0 Connector

    Scopes:
    - openid: Basic authentication
    - email: Access email address
    - profile: Access profile information
    - https://www.googleapis.com/auth/cloud-platform: GCP access
    """

    PROVIDER_NAME = "google"
    AUTHORIZE_URL = "https://accounts.google.com/o/oauth2/v2/auth"
    TOKEN_URL = "https://oauth2.googleapis.com/token"
    USERINFO_URL = "https://www.googleapis.com/oauth2/v3/userinfo"

    DEFAULT_SCOPES = [
        "openid",
        "email",
        "profile"
    ]

    def __init__(self, config: OAuthConfig):
        super().__init__(config)
        if not config.scopes:
            config.scopes = self.DEFAULT_SCOPES

    def get_authorization_url(self, state: Optional[str] = None) -> str:
        if not state:
            state = self.generate_state()

        params = {
            'client_id': self.config.client_id,
            'redirect_uri': self.config.redirect_uri,
            'response_type': 'code',
            'scope': ' '.join(self.config.scopes),
            'state': state,
            'access_type': 'offline',  # Get refresh token
            'prompt': 'consent'  # Always show consent screen
        }

        return f"{self.AUTHORIZE_URL}?{urlencode(params)}"

    def exchange_code(self, code: str) -> OAuthToken:
        data = self._make_request(
            'POST',
            self.TOKEN_URL,
            data={
                'client_id': self.config.client_id,
                'client_secret': self.config.client_secret,
                'code': code,
                'redirect_uri': self.config.redirect_uri,
                'grant_type': 'authorization_code'
            }
        )

        return OAuthToken(
            access_token=data['access_token'],
            token_type=data.get('token_type', 'Bearer'),
            expires_in=data.get('expires_in'),
            refresh_token=data.get('refresh_token'),
            scope=data.get('scope'),
            id_token=data.get('id_token')
        )

    def refresh_token(self, refresh_token: str) -> OAuthToken:
        data = self._make_request(
            'POST',
            self.TOKEN_URL,
            data={
                'client_id': self.config.client_id,
                'client_secret': self.config.client_secret,
                'refresh_token': refresh_token,
                'grant_type': 'refresh_token'
            }
        )

        return OAuthToken(
            access_token=data['access_token'],
            token_type=data.get('token_type', 'Bearer'),
            expires_in=data.get('expires_in'),
            refresh_token=refresh_token,  # Google doesn't return new refresh token
            scope=data.get('scope')
        )

    def get_user_info(self, token: OAuthToken) -> Dict[str, Any]:
        return self._make_request(
            'GET',
            self.USERINFO_URL,
            headers={'Authorization': f'{token.token_type} {token.access_token}'}
        )


class OpenAIOAuthConnector(OAuthConnector):
    """
    OpenAI OAuth Connector

    Note: OpenAI uses API keys primarily, but this supports their OAuth flow
    for platform integrations.
    """

    PROVIDER_NAME = "openai"
    AUTHORIZE_URL = "https://auth.openai.com/authorize"
    TOKEN_URL = "https://auth.openai.com/oauth/token"
    USERINFO_URL = "https://api.openai.com/v1/me"

    DEFAULT_SCOPES = [
        "openid",
        "profile",
        "email"
    ]

    def get_authorization_url(self, state: Optional[str] = None) -> str:
        if not state:
            state = self.generate_state()

        params = {
            'client_id': self.config.client_id,
            'redirect_uri': self.config.redirect_uri,
            'response_type': 'code',
            'scope': ' '.join(self.config.scopes or self.DEFAULT_SCOPES),
            'state': state
        }

        return f"{self.AUTHORIZE_URL}?{urlencode(params)}"

    def exchange_code(self, code: str) -> OAuthToken:
        data = self._make_request(
            'POST',
            self.TOKEN_URL,
            headers={'Content-Type': 'application/x-www-form-urlencoded'},
            data={
                'client_id': self.config.client_id,
                'client_secret': self.config.client_secret,
                'code': code,
                'redirect_uri': self.config.redirect_uri,
                'grant_type': 'authorization_code'
            }
        )

        return OAuthToken(
            access_token=data['access_token'],
            token_type=data.get('token_type', 'Bearer'),
            expires_in=data.get('expires_in'),
            refresh_token=data.get('refresh_token'),
            scope=data.get('scope')
        )

    def refresh_token(self, refresh_token: str) -> OAuthToken:
        data = self._make_request(
            'POST',
            self.TOKEN_URL,
            data={
                'client_id': self.config.client_id,
                'client_secret': self.config.client_secret,
                'refresh_token': refresh_token,
                'grant_type': 'refresh_token'
            }
        )

        return OAuthToken(
            access_token=data['access_token'],
            token_type=data.get('token_type', 'Bearer'),
            expires_in=data.get('expires_in'),
            refresh_token=data.get('refresh_token', refresh_token)
        )

    def get_user_info(self, token: OAuthToken) -> Dict[str, Any]:
        return self._make_request(
            'GET',
            self.USERINFO_URL,
            headers={'Authorization': f'{token.token_type} {token.access_token}'}
        )


class AnthropicConnector:
    """
    Anthropic (Claude) API Connector

    Note: Anthropic uses API keys, not OAuth. This connector manages
    API key authentication and provides a consistent interface.

    For web app session auth, see web-app-auth.py
    """

    PROVIDER_NAME = "anthropic"
    API_URL = "https://api.anthropic.com/v1"

    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key or os.environ.get('ANTHROPIC_API_KEY')

    def is_authenticated(self) -> bool:
        return bool(self.api_key)

    def get_user_info(self) -> Dict[str, Any]:
        """Get account information (if available)"""
        # Anthropic API doesn't have a user info endpoint
        # Return basic info based on API key validity
        if not self.api_key:
            return {"authenticated": False}

        return {
            "authenticated": True,
            "provider": "anthropic",
            "api_key_prefix": self.api_key[:8] + "..." if self.api_key else None
        }

    def test_connection(self) -> bool:
        """Test API key validity"""
        if not self.api_key:
            return False

        try:
            import requests
            response = requests.get(
                f"{self.API_URL}/models",
                headers={
                    'x-api-key': self.api_key,
                    'anthropic-version': '2023-06-01'
                }
            )
            return response.status_code == 200
        except:
            return False


class GitHubOAuthConnector(OAuthConnector):
    """
    GitHub OAuth Connector

    Scopes:
    - repo: Full repository access
    - workflow: Workflow management
    - read:org: Read organization membership
    - read:user: Read user profile
    - user:email: Access email addresses
    """

    PROVIDER_NAME = "github"
    AUTHORIZE_URL = "https://github.com/login/oauth/authorize"
    TOKEN_URL = "https://github.com/login/oauth/access_token"
    USERINFO_URL = "https://api.github.com/user"

    DEFAULT_SCOPES = [
        "repo",
        "workflow",
        "read:org",
        "read:user",
        "user:email"
    ]

    def get_authorization_url(self, state: Optional[str] = None) -> str:
        if not state:
            state = self.generate_state()

        params = {
            'client_id': self.config.client_id,
            'redirect_uri': self.config.redirect_uri,
            'scope': ' '.join(self.config.scopes or self.DEFAULT_SCOPES),
            'state': state
        }

        return f"{self.AUTHORIZE_URL}?{urlencode(params)}"

    def exchange_code(self, code: str) -> OAuthToken:
        data = self._make_request(
            'POST',
            self.TOKEN_URL,
            headers={'Accept': 'application/json'},
            data={
                'client_id': self.config.client_id,
                'client_secret': self.config.client_secret,
                'code': code,
                'redirect_uri': self.config.redirect_uri
            }
        )

        return OAuthToken(
            access_token=data['access_token'],
            token_type=data.get('token_type', 'Bearer'),
            scope=data.get('scope'),
            # GitHub doesn't return refresh tokens by default
            refresh_token=data.get('refresh_token')
        )

    def refresh_token(self, refresh_token: str) -> OAuthToken:
        # GitHub tokens don't expire by default
        # This is for GitHub Apps with token expiration enabled
        data = self._make_request(
            'POST',
            self.TOKEN_URL,
            headers={'Accept': 'application/json'},
            data={
                'client_id': self.config.client_id,
                'client_secret': self.config.client_secret,
                'refresh_token': refresh_token,
                'grant_type': 'refresh_token'
            }
        )

        return OAuthToken(
            access_token=data['access_token'],
            token_type=data.get('token_type', 'Bearer'),
            expires_in=data.get('expires_in'),
            refresh_token=data.get('refresh_token', refresh_token)
        )

    def get_user_info(self, token: OAuthToken) -> Dict[str, Any]:
        return self._make_request(
            'GET',
            self.USERINFO_URL,
            headers={
                'Authorization': f'{token.token_type} {token.access_token}',
                'Accept': 'application/vnd.github+json'
            }
        )


class MicrosoftOAuthConnector(OAuthConnector):
    """
    Microsoft OAuth 2.0 Connector (Azure AD / Microsoft Identity Platform)

    Useful for:
    - Microsoft 365 integration
    - Azure services
    - GitHub Enterprise (Azure AD)
    """

    PROVIDER_NAME = "microsoft"
    AUTHORIZE_URL = "https://login.microsoftonline.com/common/oauth2/v2.0/authorize"
    TOKEN_URL = "https://login.microsoftonline.com/common/oauth2/v2.0/token"
    USERINFO_URL = "https://graph.microsoft.com/v1.0/me"

    DEFAULT_SCOPES = [
        "openid",
        "profile",
        "email",
        "offline_access"
    ]

    def get_authorization_url(self, state: Optional[str] = None) -> str:
        if not state:
            state = self.generate_state()

        params = {
            'client_id': self.config.client_id,
            'redirect_uri': self.config.redirect_uri,
            'response_type': 'code',
            'scope': ' '.join(self.config.scopes or self.DEFAULT_SCOPES),
            'state': state,
            'response_mode': 'query'
        }

        return f"{self.AUTHORIZE_URL}?{urlencode(params)}"

    def exchange_code(self, code: str) -> OAuthToken:
        data = self._make_request(
            'POST',
            self.TOKEN_URL,
            headers={'Content-Type': 'application/x-www-form-urlencoded'},
            data={
                'client_id': self.config.client_id,
                'client_secret': self.config.client_secret,
                'code': code,
                'redirect_uri': self.config.redirect_uri,
                'grant_type': 'authorization_code'
            }
        )

        return OAuthToken(
            access_token=data['access_token'],
            token_type=data.get('token_type', 'Bearer'),
            expires_in=data.get('expires_in'),
            refresh_token=data.get('refresh_token'),
            scope=data.get('scope'),
            id_token=data.get('id_token')
        )

    def refresh_token(self, refresh_token: str) -> OAuthToken:
        data = self._make_request(
            'POST',
            self.TOKEN_URL,
            headers={'Content-Type': 'application/x-www-form-urlencoded'},
            data={
                'client_id': self.config.client_id,
                'client_secret': self.config.client_secret,
                'refresh_token': refresh_token,
                'grant_type': 'refresh_token'
            }
        )

        return OAuthToken(
            access_token=data['access_token'],
            token_type=data.get('token_type', 'Bearer'),
            expires_in=data.get('expires_in'),
            refresh_token=data.get('refresh_token', refresh_token),
            scope=data.get('scope')
        )

    def get_user_info(self, token: OAuthToken) -> Dict[str, Any]:
        return self._make_request(
            'GET',
            self.USERINFO_URL,
            headers={'Authorization': f'{token.token_type} {token.access_token}'}
        )


class OAuthManager:
    """
    Unified OAuth Manager for FlexNetOS

    Manages multiple OAuth providers and token storage
    """

    CONNECTORS = {
        'google': GoogleOAuthConnector,
        'github': GitHubOAuthConnector,
        'openai': OpenAIOAuthConnector,
        'microsoft': MicrosoftOAuthConnector,
    }

    def __init__(self, token_storage_path: str = ".github/cache/oauth"):
        self.storage_path = Path(token_storage_path)
        self.storage_path.mkdir(parents=True, exist_ok=True)
        self.connectors: Dict[str, OAuthConnector] = {}

    def configure_provider(self, provider: str, config: OAuthConfig):
        """Configure an OAuth provider"""
        if provider not in self.CONNECTORS:
            raise ValueError(f"Unknown provider: {provider}")

        connector_class = self.CONNECTORS[provider]
        self.connectors[provider] = connector_class(config)

    def get_authorization_url(self, provider: str, metadata: Optional[Dict] = None) -> str:
        """Get authorization URL for a provider"""
        if provider not in self.connectors:
            raise ValueError(f"Provider {provider} not configured")

        connector = self.connectors[provider]
        state = connector.generate_state(metadata)
        return connector.get_authorization_url(state)

    def handle_callback(self, provider: str, code: str, state: str) -> OAuthToken:
        """Handle OAuth callback"""
        if provider not in self.connectors:
            raise ValueError(f"Provider {provider} not configured")

        connector = self.connectors[provider]

        if not connector.validate_state(state):
            raise ValueError("Invalid or expired state")

        token = connector.exchange_code(code)
        self._save_token(provider, token)
        return token

    def get_token(self, provider: str, user_id: str = "default") -> Optional[OAuthToken]:
        """Get stored token, refreshing if needed"""
        token = self._load_token(provider, user_id)

        if not token:
            return None

        if token.is_expired:
            if token.refresh_token:
                connector = self.connectors.get(provider)
                if connector:
                    token = connector.refresh_token(token.refresh_token)
                    self._save_token(provider, token, user_id)
                else:
                    # Has refresh_token but connector not found -> cannot refresh
                    # Return None instead of expired token to maintain API contract
                    return None
            else:
                # Expired and no refresh_token -> treat as missing
                return None

        return token

    def get_user_info(self, provider: str, user_id: str = "default") -> Optional[Dict[str, Any]]:
        """Get user info for a provider"""
        token = self.get_token(provider, user_id)
        if not token:
            return None

        connector = self.connectors.get(provider)
        if not connector:
            return None

        return connector.get_user_info(token)

    def _save_token(self, provider: str, token: OAuthToken, user_id: str = "default"):
        """Save token to storage"""
        token_file = self.storage_path / f"{provider}_{user_id}.json"
        with open(token_file, 'w') as f:
            json.dump(token.to_dict(), f, indent=2)

    def _load_token(self, provider: str, user_id: str = "default") -> Optional[OAuthToken]:
        """Load token from storage"""
        token_file = self.storage_path / f"{provider}_{user_id}.json"
        if token_file.exists():
            try:
                with open(token_file) as f:
                    data = json.load(f)
                return OAuthToken(**data)
            except:
                pass
        return None

    def revoke_token(self, provider: str, user_id: str = "default"):
        """Revoke and delete a token"""
        token_file = self.storage_path / f"{provider}_{user_id}.json"
        if token_file.exists():
            token_file.unlink()


def generate_config_template() -> str:
    """Generate configuration template for all providers"""
    return json.dumps({
        "providers": {
            "google": {
                "client_id": "${GOOGLE_CLIENT_ID}",
                "client_secret": "${GOOGLE_CLIENT_SECRET}",
                "redirect_uri": "https://flexnetos.dev/auth/google/callback",
                "scopes": ["openid", "email", "profile"]
            },
            "github": {
                "client_id": "${GITHUB_CLIENT_ID}",
                "client_secret": "${GITHUB_CLIENT_SECRET}",
                "redirect_uri": "https://flexnetos.dev/auth/github/callback",
                "scopes": ["repo", "workflow", "read:org", "read:user", "user:email"]
            },
            "openai": {
                "client_id": "${OPENAI_CLIENT_ID}",
                "client_secret": "${OPENAI_CLIENT_SECRET}",
                "redirect_uri": "https://flexnetos.dev/auth/openai/callback",
                "scopes": ["openid", "profile", "email"]
            },
            "microsoft": {
                "client_id": "${MICROSOFT_CLIENT_ID}",
                "client_secret": "${MICROSOFT_CLIENT_SECRET}",
                "redirect_uri": "https://flexnetos.dev/auth/microsoft/callback",
                "scopes": ["openid", "profile", "email", "offline_access"]
            }
        },
        "api_keys": {
            "anthropic": "${ANTHROPIC_API_KEY}",
            "abacus": "${ABACUS_API_KEY}",
            "cursor": "${CURSOR_API_KEY}"
        }
    }, indent=2)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='OAuth Connectors')
    parser.add_argument('--config', action='store_true', help='Generate config template')
    parser.add_argument('--test', type=str, help='Test a provider')

    args = parser.parse_args()

    if args.config:
        print(generate_config_template())
    elif args.test:
        print(f"Testing {args.test} connector...")
        # Would implement actual test

