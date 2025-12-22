// Learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom';

// Polyfill TextEncoder/TextDecoder for jsdom test environment
// (ProviderClient uses TextDecoder for stream parsing.)
const { TextDecoder, TextEncoder } = require( 'util' );
global.TextDecoder = TextDecoder;
global.TextEncoder = TextEncoder;

// Mock Next.js router
jest.mock( 'next/navigation', () => ( {
  useRouter ()
  {
    return {
      push: jest.fn(),
      replace: jest.fn(),
      prefetch: jest.fn(),
      back: jest.fn(),
    };
  },
  usePathname ()
  {
    return '/';
  },
  useSearchParams ()
  {
    return new URLSearchParams();
  },
} ) );

// Mock WebSocket
global.WebSocket = class MockWebSocket
{
  constructor ( url )
  {
    this.url = url;
    this.readyState = WebSocket.CONNECTING;
    this.onopen = null;
    this.onclose = null;
    this.onmessage = null;
    this.onerror = null;
  }
  send = jest.fn();
  close = jest.fn();
  addEventListener = jest.fn();
  removeEventListener = jest.fn();
};
global.WebSocket.CONNECTING = 0;
global.WebSocket.OPEN = 1;
global.WebSocket.CLOSING = 2;
global.WebSocket.CLOSED = 3;

// Mock window.matchMedia
Object.defineProperty( window, 'matchMedia', {
  writable: true,
  value: jest.fn().mockImplementation( query => ( {
    matches: false,
    media: query,
    onchange: null,
    addListener: jest.fn(),
    removeListener: jest.fn(),
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  } ) ),
} );

// Mock localStorage
const localStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
Object.defineProperty( global, 'localStorage', {
  value: localStorageMock,
  writable: true,
} );
Object.defineProperty( window, 'localStorage', {
  value: localStorageMock,
  writable: true,
} );

// Mock fetch
global.fetch = jest.fn()


