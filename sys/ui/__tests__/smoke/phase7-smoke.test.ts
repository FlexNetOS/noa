/**
 * Phase 7 Smoke Tests
 *
 * Basic smoke tests to verify Phase 7 implementation is functional.
 * These tests verify that components can render and basic functionality works.
 */

import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';

// Test that all critical components can be imported
describe('Phase 7 Smoke Tests', () => {
  describe('Component Imports', () => {
    it('should import MainLayout without errors', async () => {
      const MainLayout = (await import('@/components/layout/MainLayout')).default;
      expect(MainLayout).toBeDefined();
    });

    it('should import Navigation without errors', async () => {
      const Navigation = (await import('@/components/layout/Navigation')).default;
      expect(Navigation).toBeDefined();
    });

    it('should import ActivityLog without errors', async () => {
      const ActivityLog = (await import('@/components/ActivityLog')).default;
      expect(ActivityLog).toBeDefined();
    });

    it('should import Chat component without errors', async () => {
      const Chat = (await import('@/components/Chat')).default;
      expect(Chat).toBeDefined();
    });
  });

  describe('Service Imports', () => {
    it('should import API client without errors', async () => {
      const { apiClient } = await import('@/lib/api');
      expect(apiClient).toBeDefined();
      expect(apiClient.getHealth).toBeDefined();
    });

    it('should import WebSocket client without errors', async () => {
      const { wsClient } = await import('@/lib/websocket');
      expect(wsClient).toBeDefined();
      expect(wsClient.connect).toBeDefined();
    });

    it('should import context detector without errors', async () => {
      const { contextDetector } = await import('@/services/contextDetector');
      expect(contextDetector).toBeDefined();
      expect(contextDetector.detectContext).toBeDefined();
    });

    it('should import settings sync without errors', async () => {
      const { settingsSync } = await import('@/services/settingsSync');
      expect(settingsSync).toBeDefined();
      expect(settingsSync.setScope).toBeDefined();
    });
  });

  describe('Utility Functions', () => {
    it('should have cn utility function', async () => {
      const { cn } = await import('@/lib/utils');
      expect(cn).toBeDefined();
      expect(typeof cn).toBe('function');
    });

    it('should merge class names correctly', async () => {
      const { cn } = await import('@/lib/utils');
      const result = cn('class1', 'class2');
      expect(result).toContain('class1');
      expect(result).toContain('class2');
    });
  });

  describe('Widget Registry', () => {
    it('should have widget registry', async () => {
      const { widgetRegistry } = await import('@/components/widgets/WidgetRegistry');
      expect(widgetRegistry).toBeDefined();
      expect(widgetRegistry.getAll).toBeDefined();
    });

    it('should have default widgets registered', async () => {
      const { widgetRegistry } = await import('@/components/widgets/WidgetRegistry');
      const widgets = widgetRegistry.getAll();
      expect(widgets.length).toBeGreaterThan(0);
    });
  });
});


