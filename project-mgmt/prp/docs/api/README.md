# PRP CLI API Documentation

## Overview

The PRP CLI provides a comprehensive API for programmatic access to all CLI functionality. This documentation covers the complete API surface, including core interfaces, configuration management, build systems, testing frameworks, and extension points.

## Table of Contents

- [Getting Started](#getting-started)
- [Core API](#core-api)
- [Configuration API](#configuration-api)
- [Build API](#build-api)
- [Test API](#test-api)
- [Quality API](#quality-api)
- [CI/CD API](#cicd-api)
- [Debug API](#debug-api)
- [Plugin System](#plugin-system)
- [Event System](#event-system)
- [CLI Commands API](#cli-commands-api)

## Getting Started

### Installation

```bash
npm install @prp/cli
# or
yarn add @prp/cli
# or
pnpm add @prp/cli
```

### Basic Usage

```typescript
import { PRPCli } from '@prp/cli';

// Initialize CLI
const cli = new PRPCli();

// Load configuration
await cli.loadConfig();

// Run command
const result = await cli.run(['build']);

console.log(result);
```

### TypeScript Support

```typescript
import {
  PRPCli,
  PRPConfig,
  BuildOptions,
  TestOptions,
  QualityOptions
} from '@prp/cli';

// Type-safe configuration
const config: PRPConfig = {
  name: 'my-project',
  version: '1.0.0',
  settings: {
    build: {
      mode: 'production',
      output: 'dist'
    }
  }
};
```

## Core API

### PRPCli Class

The main entry point for the PRP CLI API.

```typescript
class PRPCli {
  constructor(options?: CliOptions);

  // Configuration
  async loadConfig(configPath?: string): Promise<PRPConfig>;
  async saveConfig(config: PRPConfig): Promise<void>;
  async validateConfig(config?: PRPConfig): Promise<ValidationResult>;

  // Command execution
  async run(args: string[], options?: RunOptions): Promise<CommandResult>;
  async execute(command: string, options?: ExecuteOptions): Promise<CommandResult>;

  // Utilities
  async status(): Promise<StatusResult>;
  async version(): Promise<string>;
  async help(command?: string): Promise<string>;

  // Events
  on(event: string, handler: EventHandler): void;
  off(event: string, handler: EventHandler): void;
  emit(event: string, data?: any): void;
}
```

### Configuration

```typescript
interface CliOptions {
  configPath?: string;
  cwd?: string;
  debug?: boolean;
  quiet?: boolean;
  verbose?: boolean;
  noColor?: boolean;
}

interface RunOptions {
  cwd?: string;
  env?: Record<string, string>;
  timeout?: number;
  stream?: boolean;
}

interface ExecuteOptions extends RunOptions {
  args?: string[];
  input?: string;
}
```

### Results

```typescript
interface CommandResult {
  success: boolean;
  exitCode: number;
  stdout: string;
  stderr: string;
  duration: number;
  data?: any;
}

interface ValidationResult {
  valid: boolean;
  errors: ValidationError[];
  warnings: ValidationWarning[];
}

interface StatusResult {
  project: ProjectStatus;
  system: SystemStatus;
  quality: QualityStatus;
  ci: CIStatus;
}
```

## Configuration API

### Configuration Manager

```typescript
class ConfigurationManager {
  constructor(options?: ConfigManagerOptions);

  // Loading and saving
  async load(path?: string): Promise<PRPConfig>;
  async save(config: PRPConfig, path?: string): Promise<void>;
  async reload(): Promise<PRPConfig>;

  // Validation
  validate(config: PRPConfig): ValidationResult;
  validateSection(section: string, value: any): ValidationResult;

  // Merging and transformation
  merge(base: PRPConfig, override: Partial<PRPConfig>): PRPConfig;
  transform(config: PRPConfig, transformer: ConfigTransformer): PRPConfig;

  // Environment variables
  resolveEnvironment(config: PRPConfig): PRPConfig;
  substituteVariables(value: string): string;

  // Schema
  getSchema(): JSONSchema;
  validateAgainstSchema(config: PRPConfig): ValidationResult;
}
```

### Configuration Interfaces

```typescript
interface PRPConfig {
  name: string;
  version: string;
  description?: string;
  type?: string;
  author?: string;
  license?: string;
  repository?: string;
  keywords?: string[];
  settings: SettingsConfig;
  scripts?: Record<string, string>;
  dependencies?: Record<string, string>;
  devDependencies?: Record<string, string>;
}

interface SettingsConfig {
  debug?: DebugSettings;
  quality?: QualitySettings;
  build?: BuildSettings;
  test?: TestSettings;
  ci?: CISettings;
  development?: DevelopmentSettings;
  packageManager?: PackageManagerSettings;
  debugging?: DebuggingSettings;
  tokenAccounting?: TokenAccountingSettings;
}
```

### Configuration Builder

```typescript
class ConfigBuilder {
  constructor();

  // Basic configuration
  name(name: string): ConfigBuilder;
  version(version: string): ConfigBuilder;
  description(description: string): ConfigBuilder;
  type(type: string): ConfigBuilder;

  // Settings
  debug(settings: Partial<DebugSettings>): ConfigBuilder;
  quality(settings: Partial<QualitySettings>): ConfigBuilder;
  build(settings: Partial<BuildSettings>): ConfigBuilder;
  test(settings: Partial<TestSettings>): ConfigBuilder;

  // Scripts and dependencies
  script(name: string, command: string): ConfigBuilder;
  dependency(name: string, version: string): ConfigBuilder;
  devDependency(name: string, version: string): ConfigBuilder;

  // Build
  build(): PRPConfig;
  toJSON(): string;
  toYAML(): string;
}

// Usage
const config = new ConfigBuilder()
  .name('my-project')
  .version('1.0.0')
  .type('node-typescript')
  .debug({ enabled: true, level: 'info' })
  .quality({ strict: true, coverage: 80 })
  .script('dev', 'prp dev')
  .script('build', 'prp build')
  .build();
```

## Build API

### Build Manager

```typescript
class BuildManager {
  constructor(config: BuildSettings);

  // Build operations
  async build(options?: BuildOptions): Promise<BuildResult>;
  async watch(options?: WatchOptions): Promise<WatchHandle>;
  async clean(): Promise<void>;

  // Analysis
  async analyze(): Promise<AnalysisResult>;
  async bundle(): Promise<BundleResult>;

  // Incremental builds
  async incremental(): Promise<BuildResult>;
  getBuildHash(): string;

  // Caching
  async cache(): Promise<CacheResult>;
  async invalidateCache(): Promise<void>;

  // Optimization
  async optimize(): Promise<OptimizationResult>;
  async compress(): Promise<CompressionResult>;
}
```

### Build Configuration

```typescript
interface BuildSettings {
  mode: 'development' | 'production';
  target?: string;
  output?: string;
  clean?: boolean;
  sourcemap?: boolean;
  minify?: boolean;
  compression?: boolean;
  analyze?: boolean;
  incremental?: boolean;
  parallel?: boolean;
  cache?: CacheSettings;
  optimization?: OptimizationSettings;
  environment?: EnvironmentSettings;
  assets?: AssetSettings;
  externals?: Record<string, string>;
}

interface BuildOptions {
  mode?: BuildSettings['mode'];
  target?: string;
  output?: string;
  watch?: boolean;
  analyze?: boolean;
  clean?: boolean;
  incremental?: boolean;
}

interface BuildResult {
  success: boolean;
  duration: number;
  artifacts: BuildArtifact[];
  stats: BuildStats;
  warnings: BuildWarning[];
  errors: BuildError[];
}

interface BuildArtifact {
  name: string;
  path: string;
  size: number;
  hash: string;
  type: 'js' | 'css' | 'asset' | 'map';
}

interface BuildStats {
  totalSize: number;
  totalFiles: number;
  buildTime: number;
  cacheHitRate: number;
  compressionRatio: number;
}
```

### Build Pipeline

```typescript
class BuildPipeline {
  constructor(config: PipelineConfig);

  // Pipeline stages
  addStage(stage: PipelineStage): void;
  removeStage(name: string): void;
  getStage(name: string): PipelineStage | undefined;

  // Pipeline execution
  async execute(context: BuildContext): Promise<PipelineResult>;
  async dryRun(context: BuildContext): Promise<PipelineResult>;

  // Pipeline configuration
  setParallel(enabled: boolean): void;
  setCache(enabled: boolean): void;
  setRetry(attempts: number): void;
}

interface PipelineStage {
  name: string;
  execute: (context: BuildContext) => Promise<StageResult>;
  dependencies?: string[];
  condition?: (context: BuildContext) => boolean;
  retry?: number;
  timeout?: number;
}

// Usage example
const pipeline = new BuildPipeline({
  parallel: true,
  cache: true
});

pipeline.addStage({
  name: 'typescript',
  execute: async (context) => {
    // TypeScript compilation
    return { success: true, output: 'compiled files' };
  }
});

pipeline.addStage({
  name: 'babel',
  execute: async (context) => {
    // Babel transformation
    return { success: true, output: 'transformed files' };
  },
  dependencies: ['typescript']
});
```

## Test API

### Test Manager

```typescript
class TestManager {
  constructor(config: TestSettings);

  // Test execution
  async run(options?: TestOptions): Promise<TestResult>;
  async watch(options?: WatchOptions): Promise<WatchHandle>;
  async coverage(options?: CoverageOptions): Promise<CoverageResult>;

  // Test types
  async runUnit(options?: TestOptions): Promise<TestResult>;
  async runIntegration(options?: TestOptions): Promise<TestResult>;
  async runE2E(options?: E2ETestOptions): Promise<TestResult>;

  // Test discovery
  async discover(patterns: string[]): Promise<TestFile[]>;
  async filter(tests: TestFile[], filters: TestFilter[]): TestFile[];

  // Test reports
  async generateReport(format: ReportFormat): Promise<Report>;
  async publishReport(report: Report, destination: string): Promise<void>;

  // Snapshots
  async updateSnapshots(): Promise<void>;
  async compareSnapshots(): Promise<SnapshotResult>;
}

interface TestSettings {
  type: 'unit' | 'integration' | 'e2e' | 'all';
  framework: string;
  coverage?: boolean;
  watch?: boolean;
  parallel?: boolean;
  maxWorkers?: string | number;
  reporters?: string[];
  testEnvironment?: string;
  testMatch?: string[];
  collectCoverageFrom?: string[];
  coverageThreshold?: CoverageThreshold;
  setupFiles?: string[];
  snapshotSerializers?: string[];
}

interface TestOptions {
  type?: TestSettings['type'];
  pattern?: string;
  coverage?: boolean;
  watch?: boolean;
  parallel?: boolean;
  reporter?: string;
  timeout?: number;
  bail?: boolean;
  updateSnapshots?: boolean;
}

interface TestResult {
  success: boolean;
  duration: number;
  total: number;
  passed: number;
  failed: number;
  skipped: number;
  tests: TestSuite[];
  coverage?: CoverageResult;
  errors: TestError[];
}

interface TestSuite {
  name: string;
  tests: TestCase[];
  duration: number;
  passed: number;
  failed: number;
  skipped: number;
}

interface TestCase {
  name: string;
  status: 'passed' | 'failed' | 'skipped' | 'pending';
  duration: number;
  error?: TestError;
  assertions?: Assertion[];
}
```

### Test Framework Adapters

```typescript
interface TestFramework {
  name: string;
  version: string;

  // Configuration
  configure(config: any): void;

  // Test execution
  run(patterns: string[], options: any): Promise<TestResult>;
  watch(patterns: string[], options: any): WatchHandle;

  // Coverage
  coverage(options: any): Promise<CoverageResult>;

  // Snapshots
  updateSnapshots(): Promise<void>;
}

// Jest Adapter
class JestAdapter implements TestFramework {
  name = 'jest';
  version = '29.0.0';

  configure(config: JestConfig): void {
    // Configure Jest
  }

  async run(patterns: string[], options: any): Promise<TestResult> {
    // Run Jest tests
  }
}

// Mocha Adapter
class MochaAdapter implements TestFramework {
  name = 'mocha';
  version = '10.0.0';

  configure(config: MochaConfig): void {
    // Configure Mocha
  }

  async run(patterns: string[], options: any): Promise<TestResult> {
    // Run Mocha tests
  }
}
```

## Quality API

### Quality Manager

```typescript
class QualityManager {
  constructor(config: QualitySettings);

  // Quality gates
  async run(options?: QualityOptions): Promise<QualityResult>;
  async runGate(gate: string, options?: QualityOptions): Promise<GateResult>;

  // Linting
  async lint(options?: LintOptions): Promise<LintResult>;
  async lintFix(options?: LintOptions): Promise<LintResult>;

  // Security
  async security(options?: SecurityOptions): Promise<SecurityResult>;
  async audit(): Promise<AuditResult>;

  // Performance
  async performance(options?: PerformanceOptions): Promise<PerformanceResult>;
  async benchmark(options?: BenchmarkOptions): Promise<BenchmarkResult>;

  // Complexity
  async complexity(options?: ComplexityOptions): Promise<ComplexityResult>;

  // Duplication
  async duplication(options?: DuplicationOptions): Promise<DuplicationResult>;

  // Reports
  async generateReport(format: ReportFormat): Promise<QualityReport>;
}

interface QualitySettings {
  enabled: boolean;
  strict?: boolean;
  gates: QualityGates;
  reporting?: ReportingSettings;
  preCommitHooks?: boolean;
  prePushHooks?: boolean;
}

interface QualityGates {
  lint: LintGate;
  test: TestGate;
  security: SecurityGate;
  performance?: PerformanceGate;
  complexity?: ComplexityGate;
  duplication?: DuplicationGate;
}

interface QualityResult {
  success: boolean;
  duration: number;
  gates: GateResult[];
  score: number;
  issues: QualityIssue[];
  recommendations: Recommendation[];
}

interface GateResult {
  name: string;
  success: boolean;
  score: number;
  issues: Issue[];
  metrics: Record<string, number>;
  duration: number;
}
```

### Quality Gates

```typescript
interface LintGate {
  enabled: boolean;
  tools: string[];
  failOnWarnings?: boolean;
  maxWarnings?: number;
  configFile?: string;
  rules?: Record<string, any>;
}

interface TestGate {
  enabled: boolean;
  coverage: {
    enabled: boolean;
    minimum: number;
    threshold?: number;
    reporters?: string[];
    outputDirectory?: string;
  };
  failures: {
    maximum: number;
    retry?: number;
  };
  timeout?: number;
}

interface SecurityGate {
  enabled: boolean;
  tools: string[];
  failOnHigh: boolean;
  failOnMedium?: boolean;
  failOnLow?: boolean;
  excludeDevDependencies?: boolean;
}

// Usage
const quality = new QualityManager({
  enabled: true,
  strict: true,
  gates: {
    lint: {
      enabled: true,
      tools: ['eslint', 'prettier'],
      failOnWarnings: false,
      maxWarnings: 5
    },
    test: {
      enabled: true,
      coverage: {
        enabled: true,
        minimum: 80,
        threshold: 5
      }
    },
    security: {
      enabled: true,
      tools: ['npm-audit', 'snyk'],
      failOnHigh: true,
      failOnMedium: false
    }
  }
});

const result = await quality.run({
  strict: true,
  report: ['console', 'json'],
  fix: true
});
```

## CI/CD API

### CI/CD Manager

```typescript
class CIManager {
  constructor(config: CISettings);

  // Pipeline management
  async createPipeline(name: string, config: PipelineConfig): Promise<Pipeline>;
  async updatePipeline(id: string, config: PipelineConfig): Promise<Pipeline>;
  async deletePipeline(id: string): Promise<void>;
  async listPipelines(): Promise<Pipeline[]>;

  // Pipeline execution
  async runPipeline(id: string, options?: RunOptions): Promise<PipelineRun>;
  async cancelRun(id: string): Promise<void>;
  async getRunStatus(id: string): Promise<RunStatus>;

  // Validation
  async validatePipeline(config: PipelineConfig): Promise<ValidationResult>;
  async validateConfig(): Promise<ValidationResult>;

  // Artifacts
  async listArtifacts(runId: string): Promise<Artifact[]>;
  async downloadArtifact(artifactId: string): Promise<Buffer>;
  async uploadArtifact(data: Buffer, name: string): Promise<Artifact>;

  // Secrets
  async createSecret(name: string, value: string): Promise<Secret>;
  async updateSecret(name: string, value: string): Promise<Secret>;
  async deleteSecret(name: string): Promise<void>;
  async listSecrets(): Promise<Secret[]>;

  // Notifications
  async sendNotification(message: Notification): Promise<void>;
  async configureNotifications(config: NotificationConfig): Promise<void>;
}

interface CISettings {
  provider: 'github' | 'gitlab' | 'circleci' | 'jenkins';
  enabled: boolean;
  workflows: Record<string, WorkflowConfig>;
  cache?: CacheSettings;
  secrets?: SecretSettings;
  notifications?: NotificationSettings;
  artifacts?: ArtifactSettings;
}

interface WorkflowConfig {
  enabled: boolean;
  triggers: string[];
  branches?: string[];
  nodeVersions?: number[];
  os?: string[];
  cache?: boolean;
  artifacts?: boolean;
  dependsOn?: string[];
  environment?: Record<string, string>;
}

interface PipelineRun {
  id: string;
  status: 'pending' | 'running' | 'success' | 'failure' | 'cancelled';
  startTime: Date;
  endTime?: Date;
  duration?: number;
  logs: LogEntry[];
  artifacts: Artifact[];
  jobs: Job[];
}
```

### Provider Adapters

```typescript
interface CIProvider {
  name: string;

  // Pipeline management
  createPipeline(config: PipelineConfig): Promise<Pipeline>;
  updatePipeline(id: string, config: PipelineConfig): Promise<Pipeline>;
  deletePipeline(id: string): Promise<void>;
  listPipelines(): Promise<Pipeline[]>;

  // Pipeline execution
  runPipeline(id: string, options?: RunOptions): Promise<PipelineRun>;
  getRunStatus(id: string): Promise<RunStatus>;
  cancelRun(id: string): Promise<void>;

  // Authentication
  authenticate(credentials: Credentials): Promise<void>;
  validateAuth(): Promise<boolean>;
}

// GitHub Actions Provider
class GitHubProvider implements CIProvider {
  name = 'github';

  constructor(private octokit: Octokit) {}

  async createPipeline(config: PipelineConfig): Promise<Pipeline> {
    // Create GitHub Actions workflow
    const workflow = {
      name: config.name,
      on: this.buildTriggers(config.triggers),
      jobs: this.buildJobs(config)
    };

    await this.octokit.rest.actions.createOrUpdateRepoFile({
      owner: 'owner',
      repo: 'repo',
      path: `.github/workflows/${config.name}.yml`,
      message: `Add workflow ${config.name}`,
      content: Buffer.from(yaml.dump(workflow)).toString('base64')
    });

    return new Pipeline(config.name, workflow);
  }
}
```

## Debug API

### Debug Manager

```typescript
class DebugManager {
  constructor(config: DebuggingSettings);

  // Debug sessions
  async startSession(type: DebugType, options?: DebugOptions): Promise<DebugSession>;
  async stopSession(sessionId: string): Promise<void>;
  async listSessions(): Promise<DebugSession[]>;

  // Node.js debugging
  async startNodeDebug(options?: NodeDebugOptions): Promise<NodeDebugSession>;
  async attachNodeDebug(options?: NodeAttachOptions): Promise<NodeDebugSession>;

  // Python debugging
  async startPythonDebug(options?: PythonDebugOptions): Promise<PythonDebugSession>;
  async attachPythonDebug(options?: PythonAttachOptions): Promise<PythonDebugSession>;

  // Browser debugging
  async startBrowserDebug(options?: BrowserDebugOptions): Promise<BrowserDebugSession>;

  // Breakpoints
  async setBreakpoint(sessionId: string, location: BreakpointLocation): Promise<Breakpoint>;
  async removeBreakpoint(sessionId: string, breakpointId: string): Promise<void>;
  async listBreakpoints(sessionId: string): Promise<Breakpoint[]>;

  // Evaluation
  async evaluate(sessionId: string, expression: string): Promise<EvaluationResult>;
  async getVariables(sessionId: string, frameId?: number): Promise<Variable[]>;

  // Console
  async sendCommand(sessionId: string, command: string): Promise<CommandResult>;
  async getConsoleOutput(sessionId: string): Promise<ConsoleEntry[]>;
}

interface DebuggingSettings {
  node?: NodeDebugSettings;
  python?: PythonDebugSettings;
  browser?: BrowserDebugSettings;
}

interface DebugSession {
  id: string;
  type: DebugType;
  status: 'starting' | 'running' | 'paused' | 'stopped';
  startTime: Date;
  port?: number;
  process?: any;
  breakpoints: Breakpoint[];
  frames: StackFrame[];
}

interface DebugOptions {
  port?: number;
  host?: string;
  break?: boolean;
  sourceMaps?: boolean;
  console?: boolean;
  timeout?: number;
}
```

### Node.js Debugging

```typescript
interface NodeDebugSettings {
  enabled: boolean;
  port?: number;
  host?: string;
  break?: boolean;
  inspect?: boolean;
  restart?: boolean;
  console?: boolean;
  sourceMaps?: boolean;
  timeout?: number;
}

class NodeDebugSession implements DebugSession {
  id: string;
  type = 'node' as const;
  status: DebugSession['status'] = 'starting';
  startTime = new Date();
  port: number;
  process?: any;
  breakpoints: Breakpoint[] = [];
  frames: StackFrame[] = [];

  constructor(options: NodeDebugOptions) {
    this.port = options.port || 9229;
    this.id = `node-${Date.now()}-${Math.random()}`;
  }

  async start(): Promise<void> {
    const args = ['--inspect-brk'];
    if (this.port !== 9229) {
      args.push(`--inspect-brk=${this.port}`);
    }

    this.process = spawn('node', args, {
      stdio: ['inherit', 'pipe', 'pipe']
    });

    this.status = 'running';
  }

  async setBreakpoint(location: BreakpointLocation): Promise<Breakpoint> {
    // Set breakpoint via Debug Protocol
    const breakpoint = await this.sendRequest('setBreakpoints', {
      source: { path: location.file },
      breakpoints: [{ line: location.line }]
    });

    this.breakpoints.push(breakpoint);
    return breakpoint;
  }

  async evaluate(expression: string): Promise<EvaluationResult> {
    return await this.sendRequest('evaluate', {
      expression,
      context: 'repl'
    });
  }

  private async sendRequest(command: string, arguments?: any): Promise<any> {
    // Send request via Debug Protocol
  }
}
```

## Plugin System

### Plugin Manager

```typescript
class PluginManager {
  constructor(private config: PluginConfig);

  // Plugin lifecycle
  async loadPlugin(name: string, options?: any): Promise<Plugin>;
  async unloadPlugin(name: string): Promise<void>;
  async reloadPlugin(name: string): Promise<Plugin>;

  // Plugin discovery
  async discoverPlugins(): Promise<Plugin[]>;
  async searchPlugins(query: string): Promise<Plugin[]>;

  // Plugin management
  async installPlugin(specifier: string): Promise<Plugin>;
  async uninstallPlugin(name: string): Promise<void>;
  async updatePlugin(name: string): Promise<Plugin>;

  // Plugin execution
  async executeHook(hookName: string, context: any): Promise<HookResult[]>;
  async getPlugin(name: string): Promise<Plugin | undefined>;
  async listPlugins(): Promise<Plugin[]>;

  // Plugin configuration
  configurePlugin(name: string, config: any): Promise<void>;
  getPluginConfig(name: string): any;
}

interface Plugin {
  name: string;
  version: string;
  description: string;
  author: string;
  hooks: Record<string, HookHandler>;
  commands?: Record<string, CommandHandler>;
  config?: any;
  enabled: boolean;
}

interface PluginConfig {
  directory: string;
  autoLoad: boolean;
  registry: string;
  allowExternal: boolean;
}
```

### Plugin Development

```typescript
// Plugin definition
export default class MyPlugin implements Plugin {
  name = 'my-plugin';
  version = '1.0.0';
  description = 'My custom plugin';
  author = 'John Doe';
  enabled = true;

  hooks = {
    'before:build': this.beforeBuild.bind(this),
    'after:build': this.afterBuild.bind(this),
    'on:test:complete': this.onTestComplete.bind(this)
  };

  commands = {
    'my-command': this.myCommand.bind(this)
  };

  private async beforeBuild(context: BuildContext): Promise<void> {
    console.log('Before build hook');
    // Custom logic before build
  }

  private async afterBuild(context: BuildContext): Promise<void> {
    console.log('After build hook');
    // Custom logic after build
  }

  private async onTestComplete(result: TestResult): Promise<void> {
    console.log('Test complete hook');
    // Custom logic after tests
  }

  private async myCommand(args: string[], options: any): Promise<void> {
    console.log('My custom command');
    // Custom command implementation
  }
}

// Plugin registration
import { PluginRegistry } from '@prp/cli';

PluginRegistry.register(MyPlugin);
```

## Event System

### Event Emitter

```typescript
class EventEmitter {
  private listeners: Map<string, EventListener[]> = new Map();

  // Event registration
  on(event: string, listener: EventListener): void;
  once(event: string, listener: EventListener): void;
  off(event: string, listener: EventListener): void;

  // Event emission
  emit(event: string, data?: any): void;
  emitAsync(event: string, data?: any): Promise<void>;

  // Event management
  eventNames(): string[];
  listenerCount(event: string): number;
  removeAllListeners(event?: string): void;

  // Event utilities
  waitFor(event: string, timeout?: number): Promise<any>;
  pipeline(events: string[]): EventPipeline;
}

interface EventListener {
  (data?: any): void | Promise<void>;
}

interface EventPipeline {
  then(listener: EventListener): EventPipeline;
  catch(listener: ErrorListener): EventPipeline;
  finally(listener: EventListener): EventPipeline;
}
```

### Built-in Events

```typescript
// Build events
type BuildEvent =
  | 'build:start'
  | 'build:progress'
  | 'build:success'
  | 'build:error'
  | 'build:complete';

// Test events
type TestEvent =
  | 'test:start'
  | 'test:suite:start'
  | 'test:suite:end'
  | 'test:pass'
  | 'test:fail'
  | 'test:complete';

// Quality events
type QualityEvent =
  | 'quality:start'
  | 'quality:gate:start'
  | 'quality:gate:pass'
  | 'quality:gate:fail'
  | 'quality:complete';

// CI/CD events
type CIEvent =
  | 'pipeline:start'
  | 'pipeline:success'
  | 'pipeline:failure'
  | 'deployment:start'
  | 'deployment:success'
  | 'deployment:failure';

// Usage
cli.on('build:start', (context) => {
  console.log('Build started:', context);
});

cli.on('test:complete', (result) => {
  console.log(`Tests completed: ${result.passed}/${result.total} passed`);
});

cli.on('quality:gate:fail', (gate) => {
  console.error(`Quality gate ${gate.name} failed`);
  // Send notification, create issue, etc.
});
```

## CLI Commands API

### Command System

```typescript
class CommandManager {
  private commands: Map<string, Command> = new Map();

  // Command registration
  register(command: Command): void;
  unregister(name: string): void;

  // Command execution
  async execute(name: string, args: string[], options?: CommandOptions): Promise<CommandResult>;

  // Command discovery
  list(): Command[];
  get(name: string): Command | undefined;
  search(query: string): Command[];

  // Help system
  help(command?: string): string;
  generateHelp(command: Command): string;
}

interface Command {
  name: string;
  description: string;
  usage: string;
  examples: string[];
  options: CommandOption[];
  arguments: CommandArgument[];

  // Command execution
  execute(args: string[], options: any): Promise<CommandResult>;

  // Validation
  validate(args: string[], options: any): ValidationResult;

  // Auto-completion
  complete(args: string[]): Promise<string[]>;
}

interface CommandOption {
  name: string;
  short?: string;
  type: 'string' | 'number' | 'boolean';
  description: string;
  default?: any;
  required?: boolean;
  choices?: string[];
}
```

### Custom Commands

```typescript
// Command definition
export default class DeployCommand implements Command {
  name = 'deploy';
  description = 'Deploy application to specified environment';
  usage = 'prp deploy [options]';
  examples = [
    'prp deploy --env production',
    'prp deploy --env staging --dry-run'
  ];

  options: CommandOption[] = [
    {
      name: 'environment',
      short: 'e',
      type: 'string',
      description: 'Target environment',
      required: true,
      choices: ['development', 'staging', 'production']
    },
    {
      name: 'dry-run',
      type: 'boolean',
      description: 'Perform dry run without actual deployment',
      default: false
    },
    {
      name: 'force',
      type: 'boolean',
      description: 'Force deployment bypassing safety checks',
      default: false
    }
  ];

  arguments: CommandArgument[] = [];

  async execute(args: string[], options: any): Promise<CommandResult> {
    const { environment, dryRun, force } = options;

    try {
      console.log(`Deploying to ${environment}...`);

      if (dryRun) {
        console.log('DRY RUN: No actual deployment performed');
        return { success: true, exitCode: 0, stdout: 'Dry run completed', stderr: '', duration: 100 };
      }

      // Perform actual deployment
      const deployer = new Deployer();
      const result = await deployer.deploy(environment, { force });

      return {
        success: true,
        exitCode: 0,
        stdout: `Successfully deployed to ${environment}`,
        stderr: '',
        duration: result.duration
      };
    } catch (error) {
      return {
        success: false,
        exitCode: 1,
        stdout: '',
        stderr: error.message,
        duration: 0
      };
    }
  }

  validate(args: string[], options: any): ValidationResult {
    const errors: ValidationError[] = [];
    const warnings: ValidationWarning[] = [];

    if (!options.environment) {
      errors.push(new ValidationError('Environment is required'));
    }

    if (options.environment === 'production' && !options.force) {
      warnings.push(new ValidationWarning('Deploying to production without --force'));
    }

    return {
      valid: errors.length === 0,
      errors,
      warnings
    };
  }

  async complete(args: string[]): Promise<string[]> {
    if (args.length === 0) {
      return ['--environment', '--dry-run', '--force', '--help'];
    }

    if (args.includes('--environment') || args.includes('-e')) {
      return ['development', 'staging', 'production'];
    }

    return [];
  }
}

// Command registration
import { CommandRegistry } from '@prp/cli';

CommandRegistry.register(new DeployCommand());
```

## Usage Examples

### Complete Project Setup

```typescript
import { PRPCli, ConfigBuilder } from '@prp/cli';

async function setupProject() {
  // Create configuration
  const config = new ConfigBuilder()
    .name('my-awesome-project')
    .version('1.0.0')
    .type('node-typescript')
    .debug({ enabled: true, level: 'info' })
    .quality({
      strict: true,
      gates: {
        lint: { enabled: true, failOnWarnings: false },
        test: { coverage: { minimum: 80 } },
        security: { enabled: true, failOnHigh: true }
      }
    })
    .ci({
      provider: 'github',
      workflows: {
        test: { enabled: true, triggers: ['push', 'pull_request'] },
        build: { enabled: true, triggers: ['push'] },
        deploy: { enabled: true, triggers: ['push'], environments: ['production'] }
      }
    })
    .build();

  // Initialize CLI
  const cli = new PRPCli({
    debug: true,
    verbose: true
  });

  // Save configuration
  await cli.saveConfig(config);

  // Initialize project
  await cli.run(['init', '--template', 'node-typescript']);

  // Install dependencies
  await cli.run(['deps', 'install']);

  // Run quality checks
  const qualityResult = await cli.run(['quality', '--strict']);

  if (qualityResult.success) {
    console.log('Project setup completed successfully!');
  } else {
    console.error('Quality checks failed:', qualityResult.stderr);
  }
}

setupProject().catch(console.error);
```

### Custom Build Pipeline

```typescript
import { BuildManager, BuildPipeline } from '@prp/cli';

async function customBuild() {
  const buildManager = new BuildManager({
    mode: 'production',
    output: 'dist',
    incremental: true,
    parallel: true
  });

  // Create custom pipeline
  const pipeline = new BuildPipeline({
    parallel: true,
    cache: true
  });

  // Add pipeline stages
  pipeline.addStage({
    name: 'typescript',
    execute: async (context) => {
      // Custom TypeScript compilation
      return { success: true, output: 'compiled' };
    }
  });

  pipeline.addStage({
    name: 'minification',
    execute: async (context) => {
      // Custom minification
      return { success: true, output: 'minified' };
    },
    dependencies: ['typescript']
  });

  // Execute pipeline
  const result = await pipeline.execute({
    inputDir: 'src',
    outputDir: 'dist',
    mode: 'production'
  });

  console.log('Build completed:', result);
}
```

### Testing Framework Integration

```typescript
import { TestManager, JestAdapter } from '@prp/cli';

async function customTesting() {
  // Create custom test manager
  const testManager = new TestManager({
    type: 'all',
    framework: 'jest',
    coverage: true,
    parallel: true
  });

  // Register custom framework adapter
  testManager.registerAdapter(new JestAdapter());

  // Run tests with custom options
  const result = await testManager.run({
    pattern: '**/*.test.ts',
    coverage: true,
    reporter: 'jest-html-reporters'
  });

  // Generate custom report
  if (result.coverage) {
    const report = await testManager.generateReport({
      format: 'html',
      outputPath: 'coverage-report.html',
      includeCoverage: true,
      includeMetrics: true
    });

    console.log('Report generated:', report.path);
  }

  return result;
}
```

This comprehensive API documentation provides complete coverage of the PRP CLI's programmatic interface, enabling developers to integrate PRP functionality into their own tools and workflows.