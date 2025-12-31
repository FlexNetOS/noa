/**
 * @fileoverview Cursor CLI wrapper for command execution and verification
 * Provides a type-safe interface for interacting with the Cursor CLI
 */

import { exec, spawn, SpawnOptions, ExecOptions } from 'child_process';
import { promisify } from 'util';
import {
  CursorError,
  CursorNotInstalledError,
  CursorTimeoutError,
  toCursorError,
} from '../common/errors/cursor-errors.js';
import { CursorRateLimiter, getCursorRateLimiter, CursorRateLimiterConfig } from './cursor-rate-limiter.js';

const execAsync = promisify(exec);

/**
 * Cursor Agent CLI command names
 * Based on official Cursor CLI documentation: https://cursor.com/docs/cli/reference/parameters
 */
export enum CursorCommand {
  // Global options
  VERSION = '--version',
  HELP = '--help',
  API_KEY = '--api-key',
  PRINT = '--print',
  OUTPUT_FORMAT = '--output-format',
  BACKGROUND = '--background',
  FULLSCREEN = '--fullscreen',
  RESUME = '--resume',
  MODEL = '--model',
  FORCE = '--force',

  // Core commands
  LOGIN = 'login',
  LOGOUT = 'logout',
  STATUS = 'status',
  UPDATE = 'update',
  UPGRADE = 'upgrade',
  LS = 'ls',
  RESUME_CMD = 'resume',

  // MCP commands
  MCP = 'mcp',
  MCP_LOGIN = 'mcp login',
  MCP_LIST = 'mcp list',
  MCP_LIST_TOOLS = 'mcp list-tools',

  // Legacy VS Code-style commands (for compatibility with Cursor IDE)
  DIFF = '--diff',
  GOTO = '--goto',
  NEW_WINDOW = '--new-window',
  REUSE_WINDOW = '--reuse-window',
  WAIT = '--wait',
  ADD = '--add',
  LIST_EXTENSIONS = '--list-extensions',
  INSTALL_EXTENSION = '--install-extension',
  UNINSTALL_EXTENSION = '--uninstall-extension',
}

/**
 * Options for Cursor CLI execution
 */
export interface CursorCliOptions {
  /** Working directory for command execution */
  cwd?: string;
  /** Environment variables */
  env?: NodeJS.ProcessEnv;
  /** Command timeout in milliseconds */
  timeout?: number;
  /** Enable rate limiting */
  rateLimit?: boolean;
  /** Rate limiter configuration override */
  rateLimitConfig?: Partial<CursorRateLimiterConfig>;
  /** Custom Cursor executable path */
  cursorPath?: string;
}

/**
 * Result of a Cursor CLI command execution
 */
export interface CursorCliResult {
  /** stdout output */
  stdout: string;
  /** stderr output */
  stderr: string;
  /** Exit code */
  exitCode: number;
  /** Execution duration in ms */
  duration: number;
  /** The command that was executed */
  command: string;
}

/**
 * Cursor CLI version information
 */
export interface CursorVersion {
  version: string;
  commit?: string;
  date?: string;
  electron?: string;
  chromium?: string;
  nodeJs?: string;
  v8?: string;
  os?: string;
}

/**
 * Default options for CLI execution
 */
const DEFAULT_OPTIONS: CursorCliOptions = {
  timeout: 30000, // 30 seconds
  rateLimit: true,
};

/**
 * Cursor CLI wrapper class
 *
 * Provides methods for interacting with the Cursor CLI including:
 * - Command execution with proper error handling
 * - Version detection and verification
 * - Rate limiting support
 * - Automatic retry for transient errors
 *
 * @example
 * ```typescript
 * const cli = new CursorCli();
 *
 * // Check if Cursor is available
 * if (await cli.isAvailable()) {
 *   const version = await cli.getVersion();
 *   console.log(`Cursor ${version.version}`);
 *
 *   // Open a file
 *   await cli.openFile('/path/to/file.ts');
 * }
 * ```
 */
export class CursorCli {
  private options: CursorCliOptions;
  private rateLimiter: CursorRateLimiter;
  private cursorPath: string;

  constructor(options: CursorCliOptions = {}) {
    this.options = { ...DEFAULT_OPTIONS, ...options };
    // Create a local rate limiter instance if custom config provided,
    // otherwise use the global singleton
    this.rateLimiter = options.rateLimitConfig
      ? new CursorRateLimiter(options.rateLimitConfig)
      : getCursorRateLimiter();
    this.cursorPath = options.cursorPath || this.detectCursorPath();
  }

  /**
   * Detect the Cursor Agent executable path based on the operating system
   * The CLI command is 'cursor-agent' as per official documentation
   */
  private detectCursorPath(): string {
    const platform = process.platform;

    switch (platform) {
      case 'win32':
        // Windows: cursor-agent should be in PATH after installation
        return 'cursor-agent';
      case 'darwin':
        // macOS: cursor-agent installed via curl script
        return 'cursor-agent';
      case 'linux':
        // Linux: cursor-agent installed via curl script
        return 'cursor-agent';
      default:
        return 'cursor-agent';
    }
  }

  /**
   * Check if Cursor CLI is available
   */
  public async isAvailable(): Promise<boolean> {
    try {
      await this.execute([CursorCommand.VERSION], { rateLimit: false });
      return true;
    } catch (error) {
      if (error instanceof CursorNotInstalledError) {
        return false;
      }
      // Other errors might indicate Cursor is installed but having issues
      return false;
    }
  }

  /**
   * Get Cursor version information
   */
  public async getVersion(): Promise<CursorVersion> {
    const result = await this.execute([CursorCommand.VERSION], { rateLimit: false });

    // Parse version output
    // Example: "1.82.0" or more detailed output
    const lines = result.stdout.trim().split('\n');
    const version: CursorVersion = {
      version: lines[0]?.trim() || 'unknown',
    };

    // Parse additional version details if available
    for (const line of lines) {
      const lower = line.toLowerCase();
      if (lower.includes('commit:')) {
        version.commit = line.split(':')[1]?.trim();
      } else if (lower.includes('date:')) {
        version.date = line.split(':').slice(1).join(':').trim();
      } else if (lower.includes('electron:')) {
        version.electron = line.split(':')[1]?.trim();
      } else if (lower.includes('chromium:')) {
        version.chromium = line.split(':')[1]?.trim();
      } else if (lower.includes('node.js:') || lower.includes('node:')) {
        version.nodeJs = line.split(':')[1]?.trim();
      } else if (lower.includes('v8:')) {
        version.v8 = line.split(':')[1]?.trim();
      } else if (lower.includes('os:')) {
        version.os = line.split(':').slice(1).join(':').trim();
      }
    }

    return version;
  }

  /**
   * Open a file in Cursor
   */
  public async openFile(
    filePath: string,
    options: { line?: number; column?: number; newWindow?: boolean; wait?: boolean } = {}
  ): Promise<CursorCliResult> {
    const args: string[] = [];

    if (options.newWindow) {
      args.push(CursorCommand.NEW_WINDOW);
    } else {
      args.push(CursorCommand.REUSE_WINDOW);
    }

    if (options.wait) {
      args.push(CursorCommand.WAIT);
    }

    // Handle line/column navigation
    if (options.line !== undefined) {
      args.push(CursorCommand.GOTO);
      const position = options.column !== undefined
        ? `${filePath}:${options.line}:${options.column}`
        : `${filePath}:${options.line}`;
      args.push(position);
    } else {
      args.push(filePath);
    }

    return this.execute(args);
  }

  /**
   * Open a folder in Cursor
   */
  public async openFolder(
    folderPath: string,
    options: { newWindow?: boolean; add?: boolean } = {}
  ): Promise<CursorCliResult> {
    const args: string[] = [];

    if (options.add) {
      args.push(CursorCommand.ADD);
    } else if (options.newWindow) {
      args.push(CursorCommand.NEW_WINDOW);
    }

    args.push(folderPath);

    return this.execute(args);
  }

  /**
   * Show diff between two files
   */
  public async diff(file1: string, file2: string, wait = false): Promise<CursorCliResult> {
    const args = [CursorCommand.DIFF, file1, file2];

    if (wait) {
      args.push(CursorCommand.WAIT);
    }

    return this.execute(args);
  }

  /**
   * List installed extensions
   */
  public async listExtensions(): Promise<string[]> {
    const result = await this.execute([CursorCommand.LIST_EXTENSIONS]);
    return result.stdout
      .trim()
      .split('\n')
      .filter(ext => ext.length > 0);
  }

  /**
   * Install an extension
   */
  public async installExtension(extensionId: string): Promise<CursorCliResult> {
    return this.execute([CursorCommand.INSTALL_EXTENSION, extensionId]);
  }

  /**
   * Uninstall an extension
   */
  public async uninstallExtension(extensionId: string): Promise<CursorCliResult> {
    return this.execute([CursorCommand.UNINSTALL_EXTENSION, extensionId]);
  }

  // ============================================
  // Cursor Agent CLI Commands
  // Based on official documentation
  // ============================================

  /**
   * Log in to Cursor Agent
   */
  public async login(): Promise<CursorCliResult> {
    return this.execute([CursorCommand.LOGIN]);
  }

  /**
   * Log out from Cursor Agent
   */
  public async logout(): Promise<CursorCliResult> {
    return this.execute([CursorCommand.LOGOUT]);
  }

  /**
   * Check authentication status
   */
  public async status(): Promise<CursorCliResult> {
    return this.execute([CursorCommand.STATUS]);
  }

  /**
   * Update Cursor Agent to the latest version
   */
  public async update(): Promise<CursorCliResult> {
    return this.execute([CursorCommand.UPDATE]);
  }

  /**
   * List previous chat sessions
   */
  public async listChats(): Promise<CursorCliResult> {
    return this.execute([CursorCommand.LS]);
  }

  /**
   * Resume a previous chat session
   * @param chatId - Optional chat ID to resume. If provided, uses --resume=chatId flag.
   *                 If not provided, uses the 'resume' command to resume last session.
   */
  public async resumeChat(chatId?: string): Promise<CursorCliResult> {
    if (chatId) {
      // Use the --resume flag with the chat ID (global option)
      return this.execute([`${CursorCommand.RESUME}=${chatId}`]);
    }
    // Use the 'resume' command to resume the last session
    return this.execute([CursorCommand.RESUME_CMD]);
  }

  /**
   * Start an interactive agent session with an optional prompt
   */
  public async agent(
    prompt?: string,
    options: { model?: string; print?: boolean; outputFormat?: 'text' | 'json' | 'stream-json' } = {}
  ): Promise<CursorCliResult> {
    const args: string[] = [];

    if (options.model) {
      args.push(CursorCommand.MODEL, options.model);
    }

    if (options.print) {
      args.push(CursorCommand.PRINT);
    }

    if (options.outputFormat) {
      args.push(CursorCommand.OUTPUT_FORMAT, options.outputFormat);
    }

    if (prompt) {
      args.push(`"${prompt.replace(/"/g, '\\"')}"`);
    }

    return this.execute(args);
  }

  /**
   * List configured MCP servers
   */
  public async mcpList(): Promise<CursorCliResult> {
    return this.execute([CursorCommand.MCP, 'list']);
  }

  /**
   * List tools for a specific MCP server
   */
  public async mcpListTools(identifier: string): Promise<CursorCliResult> {
    return this.execute([CursorCommand.MCP, 'list-tools', identifier]);
  }

  /**
   * Authenticate with an MCP server
   */
  public async mcpLogin(identifier: string): Promise<CursorCliResult> {
    return this.execute([CursorCommand.MCP, 'login', identifier]);
  }

  /**
   * Execute a raw Cursor CLI command
   */
  public async execute(
    args: string[],
    options: Partial<CursorCliOptions> = {}
  ): Promise<CursorCliResult> {
    const mergedOptions = { ...this.options, ...options };
    const startTime = Date.now();

    // Apply rate limiting if enabled
    if (mergedOptions.rateLimit) {
      await this.rateLimiter.acquire('cursor-cli');
    }

    const command = `${this.cursorPath} ${args.join(' ')}`;

    const execOptions: ExecOptions = {
      cwd: mergedOptions.cwd,
      env: mergedOptions.env || process.env,
      timeout: mergedOptions.timeout,
      windowsHide: true,
    };

    try {
      const { stdout, stderr } = await execAsync(command, execOptions);
      const duration = Date.now() - startTime;

      return {
        stdout: String(stdout || ''),
        stderr: String(stderr || ''),
        exitCode: 0,
        duration,
        command,
      };
    } catch (error: any) {
      const duration = Date.now() - startTime;

      // Handle specific error types
      if (error.code === 'ENOENT' || error.message?.includes('not found')) {
        throw new CursorNotInstalledError(
          `Cursor CLI not found at path: ${this.cursorPath}`,
          { command: this.cursorPath, args }
        );
      }

      if (error.code === 'ETIMEDOUT' || error.killed) {
        throw new CursorTimeoutError(
          `Cursor CLI command timed out after ${mergedOptions.timeout}ms`,
          mergedOptions.timeout,
          { command: this.cursorPath, args }
        );
      }

      // Handle exec error with stdout/stderr
      if (error.stdout !== undefined || error.stderr !== undefined) {
        return {
          stdout: error.stdout || '',
          stderr: error.stderr || '',
          exitCode: error.code || 1,
          duration,
          command,
        };
      }

      // Convert to CursorError
      throw toCursorError(error, {
        command: this.cursorPath,
        args,
        exitCode: error.code,
        stderr: error.stderr,
      });
    }
  }

  /**
   * Execute a command with streaming output
   */
  public executeStreaming(
    args: string[],
    callbacks: {
      onStdout?: (data: string) => void;
      onStderr?: (data: string) => void;
      onClose?: (code: number) => void;
      onError?: (error: Error) => void;
    },
    options: Partial<CursorCliOptions> = {}
  ): { kill: () => void; pid?: number } {
    const mergedOptions = { ...this.options, ...options };

    const spawnOptions: SpawnOptions = {
      cwd: mergedOptions.cwd,
      env: mergedOptions.env || process.env,
      windowsHide: true,
      shell: true,
    };

    const child = spawn(this.cursorPath, args, spawnOptions);

    if (child.stdout) {
      child.stdout.on('data', (data: Buffer | string) => {
        callbacks.onStdout?.(String(data));
      });
    }

    if (child.stderr) {
      child.stderr.on('data', (data: Buffer | string) => {
        callbacks.onStderr?.(String(data));
      });
    }

    child.on('close', (code) => {
      callbacks.onClose?.(code || 0);
    });

    child.on('error', (error) => {
      callbacks.onError?.(toCursorError(error, { command: this.cursorPath, args }));
    });

    return {
      kill: () => child.kill(),
      pid: child.pid,
    };
  }

  /**
   * Get the current rate limiter status
   */
  public getRateLimiterStatus() {
    return this.rateLimiter.getStatus('cursor-cli');
  }

  /**
   * Reset the rate limiter
   */
  public resetRateLimiter(): void {
    this.rateLimiter.reset('cursor-cli');
  }
}

/**
 * Singleton instance for global CLI access
 */
let globalCli: CursorCli | null = null;

/**
 * Get the global Cursor CLI instance
 * Creates a new instance with default options if none exists
 *
 * Note: This function preserves the singleton - it does NOT replace
 * the existing instance. Use `resetGlobalCli()` first if you need to
 * reconfigure the global instance.
 */
export function getCursorCli(): CursorCli {
  if (!globalCli) {
    globalCli = new CursorCli();
  }
  return globalCli;
}

/**
 * Initialize or replace the global CLI with custom options
 * Use this only during application initialization
 *
 * @param options - Custom options for the CLI
 * @param force - If true, replaces existing instance; if false, only creates if none exists
 */
export function initGlobalCli(options: CursorCliOptions, force = false): CursorCli {
  if (!globalCli || force) {
    globalCli = new CursorCli(options);
  }
  return globalCli;
}

/**
 * Reset the global CLI instance
 */
export function resetGlobalCli(): void {
  globalCli = null;
}

/**
 * Verify Cursor CLI installation and get version info
 */
export async function verifyCursorCli(): Promise<{
  installed: boolean;
  version?: CursorVersion;
  error?: CursorError;
}> {
  // Create a temporary CLI instance for verification to avoid affecting global state
  const cli = new CursorCli({ rateLimit: false });

  try {
    const version = await cli.getVersion();
    return { installed: true, version };
  } catch (error) {
    if (error instanceof CursorError) {
      return { installed: false, error };
    }
    return {
      installed: false,
      error: toCursorError(error),
    };
  }
}
