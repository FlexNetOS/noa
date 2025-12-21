/**
 * Jest Test Setup
 * 
 * Global setup for all tests following Universal Task Execution Policy.
 */

// Mock environment variables (use Object.defineProperty to avoid readonly error)
Object.defineProperty(process.env, 'DATABASE_URL', { value: 'postgresql://test:test@localhost:5432/test' });
Object.defineProperty(process.env, 'APP_ROOT', { value: '/tmp/test-app' });

// Silence console during tests unless DEBUG=true
if (!process.env.DEBUG) {
  jest.spyOn(console, 'log').mockImplementation(() => {});
  jest.spyOn(console, 'debug').mockImplementation(() => {});
  jest.spyOn(console, 'info').mockImplementation(() => {});
  jest.spyOn(console, 'warn').mockImplementation(() => {});
}

export {};
