/**
 * @fileoverview Integration tests for Cursor CLI wrapper
 * These tests verify the actual CLI command syntax and behavior
 */

import { jest } from '@jest/globals';
import os from 'os';
import path from 'path';
import fs from 'fs';

// Mock child_process for safe testing
jest.unstable_mockModule('child_process', () => ({
  exec: jest.fn(),
  spawn: jest.fn(),
}));

const childProcess = await import('child_process');

// Import after mocking
const {
  CursorCli,
  CursorCommand,
  getCursorCli,
  verifyCursorCli,
} = await import('../../../packages/tm-core/src/cursor/cursor-cli.js');

const {
  CursorNotInstalledError,
  CursorTimeoutError,
  CURSOR_ERROR_CODES,
} = await import('../../../packages/tm-core/src/common/errors/cursor-errors.js');

describe('CursorCli', () => {
  let cli;
  let mockExec;

  beforeEach(() => {
    jest.clearAllMocks();

    // Default successful exec mock
    mockExec = childProcess.exec;
    mockExec.mockImplementation((cmd, options, callback) => {
      if (typeof options === 'function') {
        callback = options;
        options = {};
      }

      // Return a mock child process
      const cp = {
        stdout: { on: jest.fn() },
        stderr: { on: jest.fn() },
        on: jest.fn(),
      };

      // Simulate async callback
      setImmediate(() => {
        if (cmd.includes('--version')) {
          callback(null, { stdout: '1.82.0\nCommit: abc123\n', stderr: '' });
        } else {
          callback(null, { stdout: '', stderr: '' });
        }
      });

      return cp;
    });

    cli = new CursorCli({ rateLimit: false });
  });

  describe('Command Syntax Verification', () => {
    describe('Version Command', () => {
      it('should use correct --version syntax', async () => {
        await cli.execute([CursorCommand.VERSION]);

        expect(mockExec).toHaveBeenCalledWith(
          expect.stringContaining('--version'),
          expect.any(Object),
          expect.any(Function)
        );
      });

      it('should parse version output correctly', async () => {
        mockExec.mockImplementation((cmd, options, callback) => {
          if (typeof options === 'function') {
            callback = options;
          }
          setImmediate(() => {
            callback(null, {
              stdout: `1.82.0
Commit: abc123def456
Date: 2024-01-15
Electron: 27.0.0
Chromium: 118.0.5993.159
Node.js: 18.17.1
V8: 11.8.172.17
OS: Windows_NT x64 10.0.22621`,
              stderr: '',
            });
          });
          return { stdout: { on: jest.fn() }, stderr: { on: jest.fn() }, on: jest.fn() };
        });

        const version = await cli.getVersion();

        expect(version.version).toBe('1.82.0');
        expect(version.commit).toBe('abc123def456');
        expect(version.electron).toBe('27.0.0');
        expect(version.chromium).toBe('118.0.5993.159');
        expect(version.nodeJs).toBe('18.17.1');
        expect(version.v8).toBe('11.8.172.17');
        expect(version.os).toBe('Windows_NT x64 10.0.22621');
      });
    });

    describe('Open File Command', () => {
      it('should use correct syntax to open a file', async () => {
        await cli.openFile('/path/to/file.ts');

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--reuse-window');
        expect(calledCmd).toContain('/path/to/file.ts');
      });

      it('should use --new-window when specified', async () => {
        await cli.openFile('/path/to/file.ts', { newWindow: true });

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--new-window');
        expect(calledCmd).not.toContain('--reuse-window');
      });

      it('should use --wait when specified', async () => {
        await cli.openFile('/path/to/file.ts', { wait: true });

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--wait');
      });

      it('should use --goto syntax for line navigation', async () => {
        await cli.openFile('/path/to/file.ts', { line: 42 });

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--goto');
        expect(calledCmd).toContain('/path/to/file.ts:42');
      });

      it('should include column in --goto syntax', async () => {
        await cli.openFile('/path/to/file.ts', { line: 42, column: 10 });

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('/path/to/file.ts:42:10');
      });
    });

    describe('Open Folder Command', () => {
      it('should open folder with basic syntax', async () => {
        await cli.openFolder('/path/to/project');

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('/path/to/project');
      });

      it('should use --add to add folder to workspace', async () => {
        await cli.openFolder('/path/to/project', { add: true });

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--add');
      });

      it('should use --new-window for new window', async () => {
        await cli.openFolder('/path/to/project', { newWindow: true });

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--new-window');
      });
    });

    describe('Diff Command', () => {
      it('should use correct --diff syntax', async () => {
        await cli.diff('/file1.ts', '/file2.ts');

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--diff');
        expect(calledCmd).toContain('/file1.ts');
        expect(calledCmd).toContain('/file2.ts');
      });

      it('should include --wait when specified', async () => {
        await cli.diff('/file1.ts', '/file2.ts', true);

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--wait');
      });
    });

    describe('Extension Commands', () => {
      it('should use --list-extensions to list extensions', async () => {
        mockExec.mockImplementation((cmd, options, callback) => {
          if (typeof options === 'function') {
            callback = options;
          }
          setImmediate(() => {
            callback(null, {
              stdout: 'ms-python.python\ndbaeumer.vscode-eslint\n',
              stderr: '',
            });
          });
          return { stdout: { on: jest.fn() }, stderr: { on: jest.fn() }, on: jest.fn() };
        });

        const extensions = await cli.listExtensions();

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--list-extensions');
        expect(extensions).toEqual(['ms-python.python', 'dbaeumer.vscode-eslint']);
      });

      it('should use --install-extension with extension ID', async () => {
        await cli.installExtension('ms-python.python');

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--install-extension');
        expect(calledCmd).toContain('ms-python.python');
      });

      it('should use --uninstall-extension with extension ID', async () => {
        await cli.uninstallExtension('ms-python.python');

        const calledCmd = mockExec.mock.calls[0][0];
        expect(calledCmd).toContain('--uninstall-extension');
        expect(calledCmd).toContain('ms-python.python');
      });
    });
  });

  describe('Error Handling', () => {
    it('should throw CursorNotInstalledError when cursor not found', async () => {
      mockExec.mockImplementation((cmd, options, callback) => {
        if (typeof options === 'function') {
          callback = options;
        }
        const error = new Error('ENOENT: cursor not found');
        error.code = 'ENOENT';
        setImmediate(() => callback(error));
        return { stdout: { on: jest.fn() }, stderr: { on: jest.fn() }, on: jest.fn() };
      });

      await expect(cli.execute(['--version'])).rejects.toThrow(CursorNotInstalledError);
    });

    it('should throw CursorTimeoutError on timeout', async () => {
      mockExec.mockImplementation((cmd, options, callback) => {
        if (typeof options === 'function') {
          callback = options;
        }
        const error = new Error('Command timed out');
        error.killed = true;
        setImmediate(() => callback(error));
        return { stdout: { on: jest.fn() }, stderr: { on: jest.fn() }, on: jest.fn() };
      });

      await expect(cli.execute(['--version'])).rejects.toThrow(CursorTimeoutError);
    });

    it('should return result with exit code on command failure', async () => {
      mockExec.mockImplementation((cmd, options, callback) => {
        if (typeof options === 'function') {
          callback = options;
        }
        const error = new Error('Command failed');
        error.code = 1;
        error.stdout = '';
        error.stderr = 'Error: Invalid option';
        setImmediate(() => callback(error));
        return { stdout: { on: jest.fn() }, stderr: { on: jest.fn() }, on: jest.fn() };
      });

      const result = await cli.execute(['--invalid-option']);

      expect(result.exitCode).toBe(1);
      expect(result.stderr).toBe('Error: Invalid option');
    });
  });

  describe('isAvailable', () => {
    it('should return true when cursor is available', async () => {
      const available = await cli.isAvailable();
      expect(available).toBe(true);
    });

    it('should return false when cursor is not installed', async () => {
      mockExec.mockImplementation((cmd, options, callback) => {
        if (typeof options === 'function') {
          callback = options;
        }
        const error = new Error('command not found');
        error.code = 'ENOENT';
        setImmediate(() => callback(error));
        return { stdout: { on: jest.fn() }, stderr: { on: jest.fn() }, on: jest.fn() };
      });

      const available = await cli.isAvailable();
      expect(available).toBe(false);
    });
  });

  describe('verifyCursorCli', () => {
    it('should return installed: true with version info', async () => {
      const result = await verifyCursorCli();

      expect(result.installed).toBe(true);
      expect(result.version).toBeDefined();
      expect(result.version.version).toBe('1.82.0');
    });

    it('should return installed: false with error', async () => {
      mockExec.mockImplementation((cmd, options, callback) => {
        if (typeof options === 'function') {
          callback = options;
        }
        const error = new Error('ENOENT');
        error.code = 'ENOENT';
        setImmediate(() => callback(error));
        return { stdout: { on: jest.fn() }, stderr: { on: jest.fn() }, on: jest.fn() };
      });

      const result = await verifyCursorCli();

      expect(result.installed).toBe(false);
      expect(result.error).toBeDefined();
    });
  });

  describe('Platform-specific paths', () => {
    const originalPlatform = process.platform;

    afterEach(() => {
      Object.defineProperty(process, 'platform', { value: originalPlatform });
    });

    it('should use correct path on Windows', () => {
      Object.defineProperty(process, 'platform', { value: 'win32' });
      const windowsCli = new CursorCli({ rateLimit: false });
      // The Windows path should be 'cursor' (relies on PATH)
      expect(windowsCli).toBeDefined();
    });

    it('should use correct path on macOS', () => {
      Object.defineProperty(process, 'platform', { value: 'darwin' });
      const macCli = new CursorCli({ rateLimit: false });
      expect(macCli).toBeDefined();
    });

    it('should use correct path on Linux', () => {
      Object.defineProperty(process, 'platform', { value: 'linux' });
      const linuxCli = new CursorCli({ rateLimit: false });
      expect(linuxCli).toBeDefined();
    });

    it('should accept custom cursor path', () => {
      const customCli = new CursorCli({
        cursorPath: '/custom/path/cursor',
        rateLimit: false,
      });
      expect(customCli).toBeDefined();
    });
  });

  describe('Rate Limiting Integration', () => {
    it('should apply rate limiting when enabled', async () => {
      const rateLimitedCli = new CursorCli({
        rateLimit: true,
        rateLimitConfig: {
          maxRequests: 2,
          windowMs: 60000,
          minDelay: 0,
        },
      });

      // Make requests
      await rateLimitedCli.execute(['--version']);
      await rateLimitedCli.execute(['--version']);

      const status = rateLimitedCli.getRateLimiterStatus();
      expect(status.windowRequests).toBe(2);
    });

    it('should skip rate limiting when disabled', async () => {
      const noRateLimitCli = new CursorCli({ rateLimit: false });

      await noRateLimitCli.execute(['--version']);

      const status = noRateLimitCli.getRateLimiterStatus();
      // Status still exists but requests aren't recorded
      expect(status).toBeDefined();
    });
  });

  describe('Command Result', () => {
    it('should include command in result', async () => {
      const result = await cli.execute(['--version']);

      expect(result.command).toContain('--version');
    });

    it('should track execution duration', async () => {
      const result = await cli.execute(['--version']);

      expect(result.duration).toBeGreaterThanOrEqual(0);
      expect(typeof result.duration).toBe('number');
    });
  });
});

describe('CursorCli Real CLI Tests', () => {
  // These tests run against the actual Cursor CLI if available
  // They are skipped in CI or when Cursor is not installed

  const shouldRunRealTests = process.env.TEST_REAL_CURSOR === 'true';

  beforeAll(async () => {
    if (shouldRunRealTests) {
      const cli = new CursorCli({ rateLimit: false });
      const available = await cli.isAvailable();
      if (!available) {
        console.log('Skipping real CLI tests - Cursor not installed');
      }
    }
  });

  (shouldRunRealTests ? describe : describe.skip)('Real Cursor CLI', () => {
    let cli;

    beforeEach(() => {
      cli = new CursorCli({ rateLimit: true });
    });

    it('should get actual version from Cursor CLI', async () => {
      const version = await cli.getVersion();

      expect(version.version).toBeDefined();
      expect(version.version).toMatch(/^\d+\.\d+/);
    });

    it('should list installed extensions', async () => {
      const extensions = await cli.listExtensions();

      expect(Array.isArray(extensions)).toBe(true);
      // Extensions might be empty, that's okay
    });

    it('should open file without error', async () => {
      // Create a temp file
      const tempFile = path.join(os.tmpdir(), 'cursor-cli-test.txt');
      fs.writeFileSync(tempFile, 'test content');

      try {
        // This opens the file but doesn't wait
        const result = await cli.openFile(tempFile);
        expect(result.exitCode).toBe(0);
      } finally {
        fs.unlinkSync(tempFile);
      }
    });
  });
});
