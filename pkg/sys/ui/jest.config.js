/** @type {import('jest').configs} */
const nextJest = require( 'next/jest' );

const createJestconfigs = nextJest( {
  // Provide the path to your Next.js app to load next.configs.js and .env files in your test environment
  dir: './',
} );

// Add any custom configs to be passed to Jest
const customJestconfigs = {
  setupFilesAfterEnv: [ '<rootDir>/jest.setup.js' ],
  testEnvironment: 'jest-environment-jsdom',
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/src/$1',
  },
  testMatch: [
    '**/__tests__/**/*.[jt]s?(x)',
    '**/?(*.)+(spec|test).[jt]s?(x)',
  ],
  collectCoverageFrom: [
    'src/**/*.{js,jsx,ts,tsx}',
    '!src/**/*.d.ts',
    '!src/**/*.stories.{js,jsx,ts,tsx}',
    '!src/**/__tests__/**',
  ],
  coverageThreshold: {
    global: {
      branches: 70,
      functions: 70,
      lines: 70,
      statements: 70,
    },
  },
};

// createJestconfigs is exported this way to ensure that next/jest can load the Next.js configs which is async
module.exports = createJestconfigs( customJestconfigs )


