# TUI Debug Screen Implementation

## Overview

The TUI Debug Screen provides comprehensive real-time event monitoring and system diagnostics for the @dcversus/prp project. This implementation follows the specifications outlined in `PRPs/tui-implementation.md` and offers full visibility into internal system operations.

## Features

### 🎯 Real-Time Event Monitoring
- **Live Event Streaming**: Captures and displays events from all system components
- **Priority-Based Color Coding**: Events are color-coded by priority (low, medium, high, critical)
- **Source Identification**: Events are tagged by source (system, scanner, inspector, orchestrator, agent)
- **JSON Syntax Highlighting**: Full JSON syntax highlighting for structured data

### 📊 System Status Display
- **Orchestrator Status**: Current PRP, status, and chain-of-thought (CoT) display
- **Agent Tracking**: Active agents with status, progress, token usage, and task information
- **Signal Monitoring**: Real-time signal display with role-based colors
- **Resource Metrics**: System resource usage and performance indicators

### 🎮 Interactive Controls
- **Keyboard Navigation**: Full keyboard control for all debug functions
- **Toggle Full JSON**: Switch between compact and full JSON views
- **Pause/Resume**: Control event flow during analysis
- **Event Export**: Export debug events to JSON files
- **Clear Events**: Clear event history for fresh monitoring

### 🎨 Visual Design
- **Color-Coded Sources**: Each system source has a distinct color scheme
- **Priority Indicators**: Visual priority indicators for critical events
- **Progress Indicators**: Real-time progress tracking for agents
- **Status Icons**: Musical note icons for agent states (♪ ♩ ♬ ♫)

## Installation & Usage

### Basic Usage

```bash
# Test debug screen functionality
npm run debug:test

# Run interactive debug demo
npm run debug:demo
```

### Integration with TUI

The debug screen is integrated into the main TUI system:

```typescript
import { TabbedTUI } from './src/tmux/tui';
import { createDebugConfig } from './src/tui/debug-config';

// Create TUI with debug capabilities
const tui = new TabbedTUI(config, eventBus);
await tui.start();

// Switch to debug tab using Ctrl+D or Tab navigation
```

### Programmatic Usage

```typescript
import { TuiDebugScreen, createDebugConfig } from './src/tui/debug-screen';

// Create debug screen
const debugScreen = new TuiDebugScreen(createDebugConfig(), eventBus);

// Activate debug mode
debugScreen.activate();

// Add events programmatically
debugScreen.addEvent({
  id: 'custom-event',
  timestamp: new Date(),
  source: 'system',
  priority: 'high',
  type: 'custom',
  data: { message: 'Custom event data' },
  raw: 'system · Custom event occurred'
});
```

## Configuration

### Default Configuration

```typescript
const debugConfig = createDebugConfig({
  maxEvents: 100,           // Maximum events to keep in memory
  refreshInterval: 1000,    // Refresh interval in milliseconds
  showFullJson: false,      // Show full JSON or compact format
  colorScheme: {
    // Custom color schemes
    system: '\x1b[38;5;208m',    // Brand orange
    scanner: '\x1b[38;5;214m',   // Light orange
    inspector: '\x1b[38;5;208m', // Brand orange
    orchestrator: '\x1b[38;5;208m', // Brand orange
    // ... more colors
  },
  keyBindings: {
    toggleFullJson: 'j',     // Toggle JSON format
    clearEvents: 'c',        // Clear event history
    exportLogs: 'e',         // Export to file
    backToMain: 'q',         // Return to main screen
    pauseUpdates: 'p',       // Pause/resume updates
  }
});
```

### Theme Variants

```typescript
// Dark theme (default)
const darkConfig = createDarkThemeDebugConfig();

// Light theme
const lightConfig = createLightThemeDebugConfig();

// High contrast theme
const highContrastConfig = createHighContrastDebugConfig();

// Minimal theme
const minimalConfig = createMinimalDebugConfig();

// Role-specific configuration
const roleConfig = getRoleColorConfig('robo-aqa');
```

## Event Types

### System Events
- **startup**: System initialization
- **heartbeat**: System health checks
- **error**: Error conditions and failures
- **debug_mode_enabled**: Debug screen activation

### Scanner Events
- **scan_completed**: File system scan results
- **detection**: Changes detected in repository
- **file_change**: Individual file modifications

### Inspector Events
- **inspection_complete**: Code inspection results
- **risk_assessment**: Risk analysis outcomes
- **quality_check**: Quality gate validation

### Orchestrator Events
- **decision**: Agent spawning decisions
- **agent_spawn**: New agent creation
- **resource_allocation**: Budget and resource management

### Agent Events
- **progress**: Agent progress updates
- **status**: Agent status changes
- **completion**: Task completion notifications

## Keyboard Controls

### Global Controls
- **Ctrl+D**: Toggle debug mode
- **Tab**: Switch between tabs
- **1-3**: Jump to specific tabs
- **q**: Quit application

### Debug Screen Controls
- **j**: Toggle full JSON view
- **c**: Clear event history
- **e**: Export events to file
- **p**: Pause/resume updates
- **q**: Return to main screen

## Color Scheme

### Source Colors
- **System**: Brand orange (#FF9A38)
- **Scanner**: Light orange
- **Inspector**: Brand orange (priority)
- **Orchestrator**: Brand orange
- **Agent**: Role-specific colors

### Role Colors
- **robo-aqa**: Purple (#B48EAD)
- **robo-quality-control**: Red (#E06C75)
- **robo-system-analyst**: Brown (#C7A16B)
- **robo-developer**: Blue (#61AFEF)
- **robo-devops-sre**: Green (#98C379)
- **robo-ux-ui**: Pink (#D19A66)
- **robo-legal-compliance**: Light-violet (#C5A3FF)

### Priority Colors
- **Low**: Gray
- **Medium**: Yellow
- **High**: Red
- **Critical**: Bright red

## API Reference

### TuiDebugScreen

#### Constructor
```typescript
constructor(config: DebugConfig, eventBus: EventBus)
```

#### Methods
- **activate()**: Activate debug screen
- **deactivate()**: Deactivate debug screen
- **addEvent(event)**: Add a debug event
- **clearEvents()**: Clear all events
- **togglePause()**: Toggle pause state
- **exportEvents(filePath?)**: Export events to file
- **getDebugContent()**: Get formatted debug content

#### Events
- **debug.activated**: Debug screen activated
- **debug.deactivated**: Debug screen deactivated
- **debug.event**: New event received
- **debug.refresh**: Screen refreshed
- **debug.cleared**: Events cleared
- **debug.exported**: Events exported

### DebugEvent Interface

```typescript
interface DebugEvent {
  id: string;
  timestamp: Date;
  source: 'system' | 'scanner' | 'inspector' | 'orchestrator' | 'guidelines' | 'agent';
  priority: 'low' | 'medium' | 'high' | 'critical';
  type: string;
  data: unknown;
  raw?: string; // Raw log line for display
}
```

## Testing

### Unit Tests
```bash
npm run debug:test
```

### Integration Demo
```bash
npm run debug:demo
```

### Test Coverage
- ✅ Event handling and display
- ✅ JSON syntax highlighting
- ✅ Keyboard navigation
- ✅ Configuration management
- ✅ Export functionality
- ✅ Pause/resume functionality

## Examples

### Basic Event Logging
```typescript
// Add a system event
debugScreen.addEvent({
  id: 'system-status',
  timestamp: new Date(),
  source: 'system',
  priority: 'medium',
  type: 'status_update',
  data: { uptime: 1234, memory: '256MB' },
  raw: 'system · Status: Uptime 1234s, Memory 256MB'
});
```

### Agent Progress Tracking
```typescript
// Track agent progress
debugScreen.addEvent({
  id: 'agent-progress',
  timestamp: new Date(),
  source: 'agent',
  priority: 'medium',
  type: 'progress',
  data: {
    agentId: 'robo-aqa-001',
    progress: 75,
    tokens: '45.2k',
    currentTask: 'Validating cross-links'
  },
  raw: 'agent · robo-aqa-001: Validating cross-links… (75% complete)'
});
```

### Error Reporting
```typescript
// Report critical error
debugScreen.addEvent({
  id: 'critical-error',
  timestamp: new Date(),
  source: 'system',
  priority: 'critical',
  type: 'error',
  data: {
    error: 'Compilation failed',
    details: 'TypeScript errors detected',
    count: 42
  },
  raw: 'system · CRITICAL: Compilation failed - 42 TypeScript errors detected'
});
```

## Troubleshooting

### Common Issues

1. **Events not displaying**: Ensure debug screen is activated with `debugScreen.activate()`
2. **Colors not showing**: Check terminal color support and configuration
3. **Keyboard shortcuts not working**: Verify raw mode is enabled for stdin
4. **Performance issues**: Reduce `maxEvents` or increase `refreshInterval`

### Debug Logging

Enable debug logging by setting log level:
```typescript
const logger = createLayerLogger('tui-debug', { level: 'debug' });
```

## Contributing

When contributing to the debug screen:

1. Follow the color scheme specifications from `PRPs/tui-implementation.md`
2. Ensure all events include proper source and priority information
3. Test keyboard navigation thoroughly
4. Verify compatibility with different terminal sizes
5. Update documentation for new features

## License

This implementation is part of the @dcversus/prp project and follows the same license terms.