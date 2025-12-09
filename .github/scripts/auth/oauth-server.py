#!/usr/bin/env python3
"""
FlexNetOS OAuth Server
Simple web server for handling OAuth callbacks and token management

This provides a lightweight OAuth handler for:
- GitHub App installation
- OAuth provider callbacks
- Token exchange and storage
- User account linking

Usage:
  python oauth-server.py --port 8080

Then navigate to: http://localhost:8080/auth/{provider}
"""

import os
import json
import secrets
import logging
import threading
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs, urlencode
from typing import Dict, Any, Optional
from pathlib import Path

# Import our connectors
try:
    from oauth_connectors import (
        OAuthManager, OAuthConfig, OAuthToken,
        GoogleOAuthConnector, GitHubOAuthConnector,
        OpenAIOAuthConnector, MicrosoftOAuthConnector
    )
except ImportError:
    # Running standalone
    pass

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


class OAuthServerConfig:
    """Configuration for OAuth server"""

    def __init__(self, config_path: str = ".github/config/oauth.json"):
        self.config_path = Path(config_path)
        self.providers: Dict[str, Dict[str, Any]] = {}
        self.base_url = os.environ.get('OAUTH_BASE_URL', 'http://localhost:8080')
        self.load_config()

    def load_config(self):
        """Load configuration from file or environment"""
        if self.config_path.exists():
            with open(self.config_path) as f:
                config = json.load(f)
                self.providers = config.get('providers', {})
        else:
            # Load from environment
            self.providers = {
                'google': {
                    'client_id': os.environ.get('GOOGLE_CLIENT_ID', ''),
                    'client_secret': os.environ.get('GOOGLE_CLIENT_SECRET', ''),
                    'redirect_uri': f"{self.base_url}/auth/google/callback"
                },
                'github': {
                    'client_id': os.environ.get('GITHUB_CLIENT_ID', ''),
                    'client_secret': os.environ.get('GITHUB_CLIENT_SECRET', ''),
                    'redirect_uri': f"{self.base_url}/auth/github/callback"
                },
                'openai': {
                    'client_id': os.environ.get('OPENAI_CLIENT_ID', ''),
                    'client_secret': os.environ.get('OPENAI_CLIENT_SECRET', ''),
                    'redirect_uri': f"{self.base_url}/auth/openai/callback"
                },
                'microsoft': {
                    'client_id': os.environ.get('MICROSOFT_CLIENT_ID', ''),
                    'client_secret': os.environ.get('MICROSOFT_CLIENT_SECRET', ''),
                    'redirect_uri': f"{self.base_url}/auth/microsoft/callback"
                }
            }


class OAuthHandler(BaseHTTPRequestHandler):
    """HTTP handler for OAuth flows"""

    config: OAuthServerConfig = None
    oauth_manager: OAuthManager = None

    def do_GET(self):
        """Handle GET requests"""
        parsed = urlparse(self.path)
        path = parsed.path
        query = parse_qs(parsed.query)

        # Route requests
        if path == '/':
            self.handle_home()
        elif path == '/health':
            self.handle_health()
        elif path.startswith('/auth/') and path.endswith('/callback'):
            parts = path.split('/')
            if len(parts) >= 3:
                provider = parts[2]
                self.handle_callback(provider, query)
            else:
                self.send_error(400, "Invalid auth callback path")
        elif path.startswith('/auth/'):
            parts = path.split('/')
            if len(parts) >= 3:
                provider = parts[2]
                self.handle_auth_start(provider)
            else:
                self.send_error(400, "Invalid auth path")
        elif path == '/tokens':
            self.handle_list_tokens()
        elif path.startswith('/revoke/'):
            parts = path.split('/')
            if len(parts) >= 3:
                provider = parts[2]
                self.handle_revoke(provider)
            else:
                self.send_error(400, "Invalid revoke path")
        else:
            self.send_error(404, "Not Found")

    def do_POST(self):
        """Handle POST requests"""
        parsed = urlparse(self.path)
        path = parsed.path

        # Read body
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length).decode('utf-8')

        if path == '/webhook/github':
            self.handle_github_webhook(body)
        else:
            self.send_error(404, "Not Found")

    def handle_home(self):
        """Render home page with provider links"""
        html = """<!DOCTYPE html>
<html>
<head>
    <title>FlexNetOS OAuth</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
               max-width: 800px; margin: 50px auto; padding: 20px; background: #0d1117; color: #c9d1d9; }
        h1 { color: #58a6ff; }
        .provider { background: #161b22; border: 1px solid #30363d; border-radius: 6px;
                   padding: 20px; margin: 10px 0; }
        .provider h3 { margin-top: 0; color: #f0f6fc; }
        .btn { display: inline-block; padding: 10px 20px; background: #238636; color: white;
               text-decoration: none; border-radius: 6px; margin-right: 10px; }
        .btn:hover { background: #2ea043; }
        .btn-danger { background: #da3633; }
        .btn-danger:hover { background: #f85149; }
        .status { font-size: 12px; color: #8b949e; }
        code { background: #30363d; padding: 2px 6px; border-radius: 3px; }
    </style>
</head>
<body>
    <h1>🔐 FlexNetOS OAuth Server</h1>
    <p>Connect your accounts to enable automated CI/CD operations.</p>

    <div class="provider">
        <h3>🐙 GitHub</h3>
        <p>Connect your GitHub account for repository access and PR automation.</p>
        <a href="/auth/github" class="btn">Connect GitHub</a>
        <a href="/revoke/github" class="btn btn-danger">Revoke</a>
    </div>

    <div class="provider">
        <h3>🔷 Google</h3>
        <p>Connect Google for Gemini AI and GCP services.</p>
        <a href="/auth/google" class="btn">Connect Google</a>
        <a href="/revoke/google" class="btn btn-danger">Revoke</a>
    </div>

    <div class="provider">
        <h3>🤖 OpenAI</h3>
        <p>Connect OpenAI for ChatGPT integration.</p>
        <a href="/auth/openai" class="btn">Connect OpenAI</a>
        <a href="/revoke/openai" class="btn btn-danger">Revoke</a>
    </div>

    <div class="provider">
        <h3>🪟 Microsoft</h3>
        <p>Connect Microsoft for Azure AD and M365 integration.</p>
        <a href="/auth/microsoft" class="btn">Connect Microsoft</a>
        <a href="/revoke/microsoft" class="btn btn-danger">Revoke</a>
    </div>

    <hr style="border-color: #30363d; margin: 30px 0;">

    <h2>API Key Providers</h2>
    <p>These providers use API keys instead of OAuth:</p>

    <div class="provider">
        <h3>🧠 Anthropic (Claude)</h3>
        <p>Set <code>ANTHROPIC_API_KEY</code> environment variable or GitHub secret.</p>
    </div>

    <div class="provider">
        <h3>📊 Abacus AI</h3>
        <p>Set <code>ABACUS_API_KEY</code> environment variable or GitHub secret.</p>
        <p class="status">Reference: <a href="https://abacus.ai/help/python-sdk/github_cicd" style="color: #58a6ff;">Abacus AI CI/CD Guide</a></p>
    </div>

    <div class="provider">
        <h3>✨ Cursor CLI</h3>
        <p>Set <code>CURSOR_API_KEY</code> environment variable or GitHub secret.</p>
        <p class="status">Reference: <a href="https://cursor.com/docs/cli/github-actions" style="color: #58a6ff;">Cursor CLI Guide</a></p>
    </div>

    <hr style="border-color: #30363d; margin: 30px 0;">

    <p class="status">
        <a href="/tokens" style="color: #58a6ff;">View stored tokens</a> |
        <a href="/health" style="color: #58a6ff;">Health check</a>
    </p>
</body>
</html>"""

        self.send_response(200)
        self.send_header('Content-Type', 'text/html')
        self.end_headers()
        self.wfile.write(html.encode())

    def handle_health(self):
        """Health check endpoint"""
        self.send_json({'status': 'healthy', 'providers': list(self.config.providers.keys())})

    def handle_auth_start(self, provider: str):
        """Start OAuth flow for a provider"""
        if provider not in self.config.providers:
            self.send_error(404, f"Unknown provider: {provider}")
            return

        provider_config = self.config.providers[provider]

        if not provider_config.get('client_id'):
            self.send_error(500, f"Provider {provider} not configured")
            return

        # Build authorization URL
        auth_urls = {
            'google': 'https://accounts.google.com/o/oauth2/v2/auth',
            'github': 'https://github.com/login/oauth/authorize',
            'openai': 'https://auth.openai.com/authorize',
            'microsoft': 'https://login.microsoftonline.com/common/oauth2/v2.0/authorize'
        }

        # Validate provider has a known auth URL
        auth_url_base = auth_urls.get(provider)
        if not auth_url_base:
            self.send_error(400, f"Provider {provider} does not have a configured auth URL")
            return

        scopes = {
            'google': ['openid', 'email', 'profile'],
            'github': ['repo', 'workflow', 'read:org', 'read:user', 'user:email'],
            'openai': ['openid', 'profile', 'email'],
            'microsoft': ['openid', 'profile', 'email', 'offline_access']
        }

        state = secrets.token_urlsafe(32)
        with self.server.sessions_lock:
            self.server.sessions[state] = {'provider': provider}

        params = {
            'client_id': provider_config['client_id'],
            'redirect_uri': provider_config['redirect_uri'],
            'response_type': 'code',
            'scope': ' '.join(scopes.get(provider, [])),
            'state': state
        }

        # Google-specific params
        if provider == 'google':
            params['access_type'] = 'offline'
            params['prompt'] = 'consent'

        auth_url = f"{auth_url_base}?{urlencode(params)}"

        self.send_response(302)
        self.send_header('Location', auth_url)
        self.end_headers()

    def handle_callback(self, provider: str, query: Dict[str, list]):
        """Handle OAuth callback"""
        code = query.get('code', [None])[0]
        state = query.get('state', [None])[0]
        error = query.get('error', [None])[0]

        if error:
            self.send_error_page(f"OAuth error: {error}")
            return

        if not code or not state:
            self.send_error_page("Missing code or state")
            return

        with self.server.sessions_lock:
            session = self.server.sessions.pop(state, None)

        if not session:
            self.send_error_page("Invalid state - possible CSRF attack")
            return

        # Exchange code for token
        try:
            token = self.exchange_code(provider, code)
            self.save_token(provider, token)
            self.send_success_page(provider, token)
        except Exception as e:
            logger.error(f"Token exchange failed: {e}")
            self.send_error_page(f"Token exchange failed: {e}")

    def exchange_code(self, provider: str, code: str) -> Dict[str, Any]:
        """Exchange authorization code for tokens"""
        import requests

        token_urls = {
            'google': 'https://oauth2.googleapis.com/token',
            'github': 'https://github.com/login/oauth/access_token',
            'openai': 'https://auth.openai.com/oauth/token',
            'microsoft': 'https://login.microsoftonline.com/common/oauth2/v2.0/token'
        }

        # Validate provider before accessing token URL
        if provider not in token_urls:
            raise ValueError(f"Unknown OAuth provider: {provider}. Supported providers: {', '.join(token_urls.keys())}")

        if provider not in self.config.providers:
            raise ValueError(f"Provider '{provider}' not configured. Check your OAuth configuration.")

        provider_config = self.config.providers[provider]

        data = {
            'client_id': provider_config['client_id'],
            'client_secret': provider_config['client_secret'],
            'code': code,
            'redirect_uri': provider_config['redirect_uri'],
            'grant_type': 'authorization_code'
        }

        response = requests.post(
            token_urls[provider],
            data=data,
            headers={'Accept': 'application/json'}
        )

        if response.status_code != 200:
            raise Exception(f"Token request failed: {response.text}")

        return response.json()

    def save_token(self, provider: str, token: Dict[str, Any]):
        """Save token to storage"""
        storage_path = Path(".github/cache/oauth")
        storage_path.mkdir(parents=True, exist_ok=True)

        token_file = storage_path / f"{provider}_default.json"
        with open(token_file, 'w') as f:
            json.dump(token, f, indent=2)

    def handle_list_tokens(self):
        """List stored tokens"""
        storage_path = Path(".github/cache/oauth")
        tokens = {}

        if storage_path.exists():
            for token_file in storage_path.glob("*.json"):
                provider = token_file.stem.replace("_default", "")
                try:
                    with open(token_file) as f:
                        token = json.load(f)
                    tokens[provider] = {
                        'has_access_token': bool(token.get('access_token')),
                        'has_refresh_token': bool(token.get('refresh_token')),
                        'scope': token.get('scope', 'unknown')
                    }
                except:
                    pass

        self.send_json(tokens)

    def handle_revoke(self, provider: str):
        """Revoke a token"""
        storage_path = Path(".github/cache/oauth")
        token_file = storage_path / f"{provider}_default.json"

        if token_file.exists():
            token_file.unlink()

        self.send_response(302)
        self.send_header('Location', '/')
        self.end_headers()

    def handle_github_webhook(self, body: str):
        """Handle GitHub webhook events"""
        try:
            event = json.loads(body)
            event_type = self.headers.get('X-GitHub-Event', 'unknown')

            logger.info(f"Received GitHub webhook: {event_type}")

            # Process event based on type
            if event_type == 'installation':
                action = event.get('action')
                logger.info(f"GitHub App {action}")

            self.send_json({'status': 'received', 'event': event_type})

        except Exception as e:
            logger.error(f"Webhook processing error: {e}")
            self.send_error(500, str(e))

    def send_json(self, data: Dict[str, Any]):
        """Send JSON response"""
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        self.wfile.write(json.dumps(data, indent=2).encode())

    def send_success_page(self, provider: str, token: Dict[str, Any]):
        """Send success page after OAuth"""
        html = f"""<!DOCTYPE html>
<html>
<head>
    <title>Success - FlexNetOS OAuth</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, sans-serif;
               max-width: 600px; margin: 100px auto; padding: 20px;
               background: #0d1117; color: #c9d1d9; text-align: center; }}
        .success {{ color: #3fb950; font-size: 48px; }}
        h1 {{ color: #f0f6fc; }}
        code {{ background: #30363d; padding: 4px 8px; border-radius: 4px; }}
        a {{ color: #58a6ff; }}
    </style>
</head>
<body>
    <div class="success">✅</div>
    <h1>Connected to {provider.title()}!</h1>
    <p>Your {provider.title()} account has been successfully connected.</p>
    <p>Token scope: <code>{token.get('scope', 'default')}</code></p>
    <p><a href="/">← Back to home</a></p>
</body>
</html>"""

        self.send_response(200)
        self.send_header('Content-Type', 'text/html')
        self.end_headers()
        self.wfile.write(html.encode())

    def send_error_page(self, error: str):
        """Send error page"""
        html = f"""<!DOCTYPE html>
<html>
<head>
    <title>Error - FlexNetOS OAuth</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, sans-serif;
               max-width: 600px; margin: 100px auto; padding: 20px;
               background: #0d1117; color: #c9d1d9; text-align: center; }}
        .error {{ color: #f85149; font-size: 48px; }}
        h1 {{ color: #f0f6fc; }}
        a {{ color: #58a6ff; }}
    </style>
</head>
<body>
    <div class="error">❌</div>
    <h1>Authentication Error</h1>
    <p>{error}</p>
    <p><a href="/">← Try again</a></p>
</body>
</html>"""

        self.send_response(400)
        self.send_header('Content-Type', 'text/html')
        self.end_headers()
        self.wfile.write(html.encode())


def run_server(port: int = 8080):
    """Run the OAuth server"""
    config = OAuthServerConfig()

    OAuthHandler.config = config
    OAuthHandler.oauth_manager = OAuthManager()

    server = HTTPServer(('0.0.0.0', port), OAuthHandler)
    # Per-server session store with a lock to avoid cross-thread leakage
    server.sessions: Dict[str, Dict[str, Any]] = {}
    server.sessions_lock = threading.Lock()

    logger.info(f"🚀 FlexNetOS OAuth Server running on http://localhost:{port}")
    logger.info("Press Ctrl+C to stop")

    try:
        server.serve_forever()
    except KeyboardInterrupt:
        logger.info("Shutting down...")
        server.shutdown()


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description='FlexNetOS OAuth Server')
    parser.add_argument('--port', type=int, default=8080, help='Port to run on')

    args = parser.parse_args()
    run_server(args.port)

