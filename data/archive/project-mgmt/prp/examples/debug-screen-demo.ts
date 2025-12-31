#!/usr/bin/env ts-node

/**
 * Debug Screen Demo
 *
 * Demonstrates the TUI debug screen functionality with real-time events
 * and system monitoring based on tui-implementation.md specifications
 */

import { EventBus } from '../src/shared/events';
import { TabbedTUI, type TUIConfig } from '../src/tmux/tui';
import { createDebugConfig } from '../src/tui/debug-config';

async function debugScreenDemo(): Promise<void> {
  console.log('🎵 ♫ @dcversus/prp - Debug Screen Demo');
  console.log('═'.repeat(50));

  // Create event bus
  const eventBus = new EventBus();

  // Create TUI configuration
  const tuiConfig: TUIConfig = {
    enabled: true,
    refreshInterval: 1000,
    maxTabs: 10,
    keyBindings: {
      nextTab: '\x09',        // Tab
      prevTab: '\x1b[Z',     // Shift+Tab
      closeTab: 'w',
      switchToMain: '1',
      switchToOrchestrator: '2',
      switchToInfo: '3',
      refresh: 'r',
      quit: 'q'
    },
    colors: {
      active: '\x1b[7m',
      inactive: '\x1b[90m',
      error: '\x1b[91m',
      warning: '\x1b[93m',
      success: '\x1b[92m',
      text: '\x1b[97m',
      border: '\x1b[90m'
    },
    layout: {
      tabBar: {
        height: 1,
        position: 'top'
      },
      content: {
        padding: 1,
        showLineNumbers: false
      },
      status: {
        height: 1,
        position: 'bottom'
      }
    }
  };

  // Create and start TUI
  const tui = new TabbedTUI(tuiConfig, eventBus);

  try {
    await tui.start();
    console.log('✅ TUI started successfully');

    // Simulate real-time events for demonstration
    let eventCount = 0;

    const simulateEvents = setInterval(() => {
      eventCount++;

      // System events
      if (eventCount % 5 === 0) {
        eventBus.emit('system', {
          type: 'heartbeat',
          data: {
            uptime: process.uptime(),
            memory: process.memoryUsage(),
            activeAgents: Math.floor(Math.random() * 5) + 1
          }
        });
      }

      // Scanner events
      if (eventCount % 7 === 0) {
        eventBus.emit('scanner', {
          type: 'scan_completed',
          data: {
            detected: ['file-change', 'branch-update', 'config-modification'],
            count: Math.floor(Math.random() * 10) + 1,
            risk: Math.floor(Math.random() * 10)
          }
        });
      }

      // Inspector events
      if (eventCount % 8 === 0) {
        eventBus.emit('inspector', {
          type: 'inspection_complete',
          data: {
            impact: ['low', 'medium', 'high'][Math.floor(Math.random() * 3)],
            risk: Math.floor(Math.random() * 10),
            files: [`PRPs/file-${Math.floor(Math.random() * 10)}.md`],
            why: 'quality validation required'
          }
        });
      }

      // Orchestrator events
      if (eventCount % 6 === 0) {
        eventBus.emit('orchestrator', {
          type: 'decision',
          data: {
            currentPrp: `prp-demo-${Math.floor(Math.random() * 5)}`,
            decision: 'spawn_agent',
            role: ['robo-aqa', 'robo-developer', 'robo-system-analyst'][Math.floor(Math.random() * 3)],
            budget: { tokens: Math.floor(Math.random() * 50000) + 10000 }
          }
        });
      }

      // Stop after 30 events
      if (eventCount >= 30) {
        clearInterval(simulateEvents);
        console.log('🏁 Event simulation completed');

        // Keep TUI running for user interaction
        setTimeout(() => {
          console.log('\n📋 Demo completed! Press Ctrl+C to exit.');
        }, 2000);
      }
    }, 2000);

    // Handle graceful shutdown
    process.on('SIGINT', async () => {
      console.log('\n🛑 Shutting down demo...');
      clearInterval(simulateEvents);
      await tui.stop();
      process.exit(0);
    });

    console.log('🎮 Controls:');
    console.log('  • Tab: Switch between tabs');
    console.log('  • Ctrl+D: Toggle debug mode');
    console.log('  • 1: Main screen');
    console.log('  • 2: Orchestrator');
    console.log('  • 3: Debug screen');
    console.log('  • q: Quit');
    console.log('\n🐛 Debug screen controls:');
    console.log('  • j: Toggle full JSON view');
    console.log('  • c: Clear events');
    console.log('  • e: Export events');
    console.log('  • p: Pause/resume updates');
    console.log('  • q: Back to main');
    console.log('\n🚀 Demo running... Simulating real-time events.');

  } catch (error) {
    console.error('❌ Failed to start TUI:', error);
    process.exit(1);
  }
}

// Run demo if this file is executed directly
if (import.meta.url === `file://${process.argv[1]}`) {
  debugScreenDemo().catch(console.error);
}

export { debugScreenDemo };