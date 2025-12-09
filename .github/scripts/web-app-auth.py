#!/usr/bin/env python3
"""
FlexNetOS Web App Authentication Module
Handles authentication with AI providers via web app sessions (not API keys)

Supported Providers:
- Claude.ai (Anthropic)
- ChatGPT (OpenAI)
- Gemini (Google)
- GitHub Copilot

Note: This module uses browser automation to authenticate with web apps,
which provides access to the latest models without requiring API access.
"""

import os
import json
import time
import logging
from pathlib import Path
from typing import Optional, Dict, Any
from dataclasses import dataclass
from abc import ABC, abstractmethod

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)


@dataclass
class AuthSession:
    """Represents an authenticated session"""
    provider: str
    session_id: str
    access_token: Optional[str]
    refresh_token: Optional[str]
    expires_at: Optional[int]
    cookies: Dict[str, str]
    is_valid: bool


class WebAppAuthProvider(ABC):
    """Base class for web app authentication providers"""

    def __init__(self, provider_name: str):
        self.provider_name = provider_name
        self.session: Optional[AuthSession] = None
        self.browser = None
        self.page = None

    @abstractmethod
    async def authenticate(self, credentials: Dict[str, str]) -> AuthSession:
        """Authenticate with the provider"""
        pass

    @abstractmethod
    async def refresh_session(self) -> bool:
        """Refresh an expired session"""
        pass

    @abstractmethod
    async def send_message(self, message: str, context: Optional[str] = None) -> str:
        """Send a message to the AI and get a response"""
        pass

    async def init_browser(self):
        """Initialize Playwright browser"""
        try:
            from playwright.async_api import async_playwright
            self.playwright = await async_playwright().start()
            self.browser = await self.playwright.chromium.launch(headless=True)
            self.context = await self.browser.new_context(
                user_agent='Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
            )
            self.page = await self.context.new_page()
            logger.info(f"Browser initialized for {self.provider_name}")
        except Exception as e:
            logger.error(f"Failed to initialize browser: {e}")
            raise

    async def close(self):
        """Clean up browser resources"""
        if self.browser:
            await self.browser.close()
        if hasattr(self, 'playwright'):
            await self.playwright.stop()


class ClaudeWebAuth(WebAppAuthProvider):
    """Claude.ai Web Authentication"""

    CLAUDE_URL = "https://claude.ai"

    def __init__(self):
        super().__init__("claude")

    async def authenticate(self, credentials: Dict[str, str]) -> AuthSession:
        """Authenticate with Claude.ai using session cookie"""
        await self.init_browser()

        session_token = credentials.get('session_token')
        if session_token:
            # Use existing session token
            await self.context.add_cookies([{
                'name': 'sessionKey',
                'value': session_token,
                'domain': '.claude.ai',
                'path': '/'
            }])

            await self.page.goto(f"{self.CLAUDE_URL}/chat")
            await self.page.wait_for_load_state('networkidle')

            # Check if authenticated
            if 'login' not in self.page.url:
                self.session = AuthSession(
                    provider='claude',
                    session_id=session_token[:16],
                    access_token=session_token,
                    refresh_token=None,
                    expires_at=None,
                    cookies={'sessionKey': session_token},
                    is_valid=True
                )
                logger.info("Claude authentication successful")
                return self.session

        logger.error("Claude authentication failed")
        return AuthSession(
            provider='claude',
            session_id='',
            access_token=None,
            refresh_token=None,
            expires_at=None,
            cookies={},
            is_valid=False
        )

    async def refresh_session(self) -> bool:
        """Refresh Claude session"""
        if not self.session or not self.session.is_valid:
            return False
        # Claude sessions typically don't need refresh
        return True

    async def send_message(self, message: str, context: Optional[str] = None) -> str:
        """Send a message to Claude and get response"""
        if not self.session or not self.session.is_valid:
            raise Exception("Not authenticated")

        try:
            # Navigate to chat
            await self.page.goto(f"{self.CLAUDE_URL}/chat/new")
            await self.page.wait_for_load_state('networkidle')

            # Find input and send message
            full_message = f"{context}\n\n{message}" if context else message

            # Wait for text input
            await self.page.wait_for_selector('[contenteditable="true"]', timeout=10000)
            await self.page.fill('[contenteditable="true"]', full_message)

            # Send message
            await self.page.keyboard.press('Enter')

            # Wait for response
            await self.page.wait_for_selector('[data-is-streaming="false"]', timeout=120000)

            # Get response text
            response_elements = await self.page.query_selector_all('.prose')
            if response_elements:
                response = await response_elements[-1].inner_text()
                return response

        except Exception as e:
            logger.error(f"Claude message error: {e}")

        return ""


class ChatGPTWebAuth(WebAppAuthProvider):
    """ChatGPT Web Authentication"""

    CHATGPT_URL = "https://chat.openai.com"

    def __init__(self):
        super().__init__("chatgpt")

    async def authenticate(self, credentials: Dict[str, str]) -> AuthSession:
        """Authenticate with ChatGPT using session cookie"""
        await self.init_browser()

        access_token = credentials.get('access_token')
        session_token = credentials.get('session_token')

        if access_token or session_token:
            cookies = []
            if session_token:
                cookies.append({
                    'name': '__Secure-next-auth.session-token',
                    'value': session_token,
                    'domain': '.chat.openai.com',
                    'path': '/'
                })
            if access_token:
                cookies.append({
                    'name': '__Secure-next-auth.callback-url',
                    'value': self.CHATGPT_URL,
                    'domain': '.chat.openai.com',
                    'path': '/'
                })

            await self.context.add_cookies(cookies)
            await self.page.goto(self.CHATGPT_URL)
            await self.page.wait_for_load_state('networkidle')

            if 'auth' not in self.page.url:
                self.session = AuthSession(
                    provider='chatgpt',
                    session_id=session_token[:16] if session_token else '',
                    access_token=access_token,
                    refresh_token=None,
                    expires_at=None,
                    cookies={'session_token': session_token},
                    is_valid=True
                )
                logger.info("ChatGPT authentication successful")
                return self.session

        logger.error("ChatGPT authentication failed")
        return AuthSession(
            provider='chatgpt',
            session_id='',
            access_token=None,
            refresh_token=None,
            expires_at=None,
            cookies={},
            is_valid=False
        )

    async def refresh_session(self) -> bool:
        """Refresh ChatGPT session"""
        if not self.session:
            return False
        # ChatGPT handles refresh automatically
        return self.session.is_valid

    async def send_message(self, message: str, context: Optional[str] = None) -> str:
        """Send a message to ChatGPT and get response"""
        if not self.session or not self.session.is_valid:
            raise Exception("Not authenticated")

        try:
            await self.page.goto(self.CHATGPT_URL)
            await self.page.wait_for_load_state('networkidle')

            full_message = f"{context}\n\n{message}" if context else message

            # Find textarea
            await self.page.wait_for_selector('textarea', timeout=10000)
            await self.page.fill('textarea', full_message)

            # Click send button
            await self.page.click('button[data-testid="send-button"]')

            # Wait for response
            await self.page.wait_for_selector('[data-message-author-role="assistant"]', timeout=120000)
            await asyncio.sleep(2)  # Wait for streaming to complete

            # Get response
            messages = await self.page.query_selector_all('[data-message-author-role="assistant"]')
            if messages:
                return await messages[-1].inner_text()

        except Exception as e:
            logger.error(f"ChatGPT message error: {e}")

        return ""


class GeminiWebAuth(WebAppAuthProvider):
    """Google Gemini Web Authentication"""

    GEMINI_URL = "https://gemini.google.com"

    def __init__(self):
        super().__init__("gemini")

    async def authenticate(self, credentials: Dict[str, str]) -> AuthSession:
        """Authenticate with Gemini using Google cookies"""
        await self.init_browser()

        # Gemini uses Google's standard auth cookies
        cookies_json = credentials.get('cookies_json')
        if cookies_json:
            cookies = json.loads(cookies_json)
            for cookie in cookies:
                await self.context.add_cookies([{
                    'name': cookie['name'],
                    'value': cookie['value'],
                    'domain': cookie.get('domain', '.google.com'),
                    'path': cookie.get('path', '/')
                }])

            await self.page.goto(self.GEMINI_URL)
            await self.page.wait_for_load_state('networkidle')

            if 'accounts.google.com' not in self.page.url:
                self.session = AuthSession(
                    provider='gemini',
                    session_id='google',
                    access_token=None,
                    refresh_token=None,
                    expires_at=None,
                    cookies=cookies,
                    is_valid=True
                )
                logger.info("Gemini authentication successful")
                return self.session

        logger.error("Gemini authentication failed")
        return AuthSession(
            provider='gemini',
            session_id='',
            access_token=None,
            refresh_token=None,
            expires_at=None,
            cookies={},
            is_valid=False
        )

    async def refresh_session(self) -> bool:
        """Refresh Gemini session"""
        return self.session.is_valid if self.session else False

    async def send_message(self, message: str, context: Optional[str] = None) -> str:
        """Send a message to Gemini and get response"""
        if not self.session or not self.session.is_valid:
            raise Exception("Not authenticated")

        try:
            await self.page.goto(f"{self.GEMINI_URL}/app")
            await self.page.wait_for_load_state('networkidle')

            full_message = f"{context}\n\n{message}" if context else message

            # Find input
            await self.page.wait_for_selector('textarea, [contenteditable="true"]', timeout=10000)
            input_el = await self.page.query_selector('textarea') or await self.page.query_selector('[contenteditable="true"]')

            if input_el:
                await input_el.fill(full_message)
                await self.page.keyboard.press('Enter')

                # Wait for response
                await asyncio.sleep(3)
                await self.page.wait_for_selector('[data-response-text]', timeout=120000)

                responses = await self.page.query_selector_all('[data-response-text]')
                if responses:
                    return await responses[-1].inner_text()

        except Exception as e:
            logger.error(f"Gemini message error: {e}")

        return ""


class GitHubCopilotAuth(WebAppAuthProvider):
    """GitHub Copilot Authentication (via GitHub)"""

    GITHUB_URL = "https://github.com"
    COPILOT_URL = "https://copilot.github.com"

    def __init__(self):
        super().__init__("copilot")

    async def authenticate(self, credentials: Dict[str, str]) -> AuthSession:
        """Authenticate with GitHub Copilot"""
        await self.init_browser()

        gh_token = credentials.get('github_token')
        if gh_token:
            # GitHub uses token-based auth
            self.session = AuthSession(
                provider='copilot',
                session_id='github',
                access_token=gh_token,
                refresh_token=None,
                expires_at=None,
                cookies={},
                is_valid=True
            )
            logger.info("GitHub Copilot authentication successful")
            return self.session

        logger.error("GitHub Copilot authentication failed")
        return AuthSession(
            provider='copilot',
            session_id='',
            access_token=None,
            refresh_token=None,
            expires_at=None,
            cookies={},
            is_valid=False
        )

    async def refresh_session(self) -> bool:
        """Refresh GitHub session"""
        return self.session.is_valid if self.session else False

    async def send_message(self, message: str, context: Optional[str] = None) -> str:
        """Use GitHub Copilot for code suggestions"""
        # Copilot works through IDE/CLI integration
        # This would use the GitHub API with Copilot endpoint
        if not self.session or not self.session.is_valid:
            raise Exception("Not authenticated")

        # In practice, Copilot suggestions come through the IDE
        return ""


class WebAppAuthManager:
    """Manages authentication across multiple providers"""

    PROVIDERS = {
        'claude': ClaudeWebAuth,
        'chatgpt': ChatGPTWebAuth,
        'gemini': GeminiWebAuth,
        'copilot': GitHubCopilotAuth,
    }

    def __init__(self):
        self.sessions: Dict[str, AuthSession] = {}
        self.providers: Dict[str, WebAppAuthProvider] = {}
        self.session_cache_path = Path('.github/cache/sessions')
        self.session_cache_path.mkdir(parents=True, exist_ok=True)

    async def authenticate(self, provider_name: str, credentials: Dict[str, str]) -> AuthSession:
        """Authenticate with a provider"""
        if provider_name not in self.PROVIDERS:
            raise ValueError(f"Unknown provider: {provider_name}")

        # Check cache first
        cached = self._load_cached_session(provider_name)
        if cached and cached.is_valid:
            self.sessions[provider_name] = cached
            return cached

        # Authenticate
        provider_class = self.PROVIDERS[provider_name]
        provider = provider_class()
        session = await provider.authenticate(credentials)

        if session.is_valid:
            self.sessions[provider_name] = session
            self.providers[provider_name] = provider
            self._cache_session(provider_name, session)

        return session

    async def get_provider(self, provider_name: str) -> Optional[WebAppAuthProvider]:
        """Get an authenticated provider"""
        return self.providers.get(provider_name)

    async def send_to_provider(self, provider_name: str, message: str, context: Optional[str] = None) -> str:
        """Send a message to a specific provider"""
        provider = self.providers.get(provider_name)
        if not provider:
            raise ValueError(f"Provider {provider_name} not authenticated")
        return await provider.send_message(message, context)

    async def send_to_best_available(self, message: str, context: Optional[str] = None) -> str:
        """Send to the best available provider"""
        # Priority order
        priority = ['claude', 'chatgpt', 'gemini', 'copilot']

        for provider_name in priority:
            if provider_name in self.providers:
                try:
                    return await self.send_to_provider(provider_name, message, context)
                except Exception as e:
                    logger.warning(f"Provider {provider_name} failed: {e}")
                    continue

        raise Exception("No providers available")

    def _load_cached_session(self, provider_name: str) -> Optional[AuthSession]:
        """Load a cached session"""
        cache_file = self.session_cache_path / f"{provider_name}.json"
        if cache_file.exists():
            try:
                with open(cache_file) as f:
                    data = json.load(f)
                return AuthSession(**data)
            except:
                pass
        return None

    def _cache_session(self, provider_name: str, session: AuthSession):
        """Cache a session"""
        cache_file = self.session_cache_path / f"{provider_name}.json"
        try:
            with open(cache_file, 'w') as f:
                json.dump({
                    'provider': session.provider,
                    'session_id': session.session_id,
                    'access_token': session.access_token,
                    'refresh_token': session.refresh_token,
                    'expires_at': session.expires_at,
                    'cookies': session.cookies,
                    'is_valid': session.is_valid
                }, f)
        except Exception as e:
            logger.warning(f"Failed to cache session: {e}")

    async def close_all(self):
        """Close all provider connections"""
        for provider in self.providers.values():
            await provider.close()


# CLI Interface
async def main():
    """CLI for testing web app auth"""
    import argparse

    parser = argparse.ArgumentParser(description='FlexNetOS Web App Auth')
    parser.add_argument('--provider', choices=['claude', 'chatgpt', 'gemini', 'copilot'], required=True)
    parser.add_argument('--session-token', help='Session token for authentication')
    parser.add_argument('--message', help='Test message to send')
    args = parser.parse_args()

    manager = WebAppAuthManager()

    credentials = {}
    if args.session_token:
        credentials['session_token'] = args.session_token

    session = await manager.authenticate(args.provider, credentials)

    if session.is_valid:
        print(f"✅ Authenticated with {args.provider}")

        if args.message:
            response = await manager.send_to_provider(args.provider, args.message)
            print(f"\n📝 Response:\n{response}")
    else:
        print(f"❌ Authentication failed for {args.provider}")

    await manager.close_all()


if __name__ == "__main__":
    import asyncio
    asyncio.run(main())

