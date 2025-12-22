/**
 * Phase 7 Smoke Tests
 *
 * Basic smoke tests to verify Phase 7 implementation is functional.
 * These tests verify that components can render and basic functionality works.
 */

import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';

// Test that all critical components can be imported
describe( 'Phase 7 Smoke Tests', () =>
{
  describe( 'Component Imports', () =>
  {
    it( 'should import MainLayout without errors', () =>
    {
      // Use require() so Jest transforms TS/TSX before execution.
      const MainLayout = require( '@/components/layout/MainLayout' ).default;
      expect( MainLayout ).toBeDefined();
    } );

    it( 'should import Navigation without errors', () =>
    {
      const Navigation = require( '@/components/layout/Navigation' ).default;
      expect( Navigation ).toBeDefined();
    } );

    it( 'should import ActivityLog without errors', () =>
    {
      const ActivityLog = require( '@/components/ActivityLog' ).default;
      expect( ActivityLog ).toBeDefined();
    } );

    it( 'should import Chat component without errors', () =>
    {
      const Chat = require( '@/components/Chat' ).default;
      expect( Chat ).toBeDefined();
    } );
  } );

  describe( 'Service Imports', () =>
  {
    it( 'should import API client without errors', () =>
    {
      const { apiClient } = require( '@/lib/api' );
      expect( apiClient ).toBeDefined();
      expect( apiClient.getHealth ).toBeDefined();
    } );

    it( 'should import WebSocket client without errors', () =>
    {
      const { wsClient } = require( '@/lib/websocket' );
      expect( wsClient ).toBeDefined();
      expect( wsClient.connect ).toBeDefined();
    } );

    it( 'should import context detector without errors', () =>
    {
      const { contextDetector } = require( '@/services/contextDetector' );
      expect( contextDetector ).toBeDefined();
      expect( contextDetector.detectContext ).toBeDefined();
    } );

    it( 'should import settings sync without errors', () =>
    {
      const { settingsSync } = require( '@/services/settingsSync' );
      expect( settingsSync ).toBeDefined();
      expect( settingsSync.setScope ).toBeDefined();
    } );
  } );

  describe( 'Utility Functions', () =>
  {
    it( 'should have cn utility function', () =>
    {
      const { cn } = require( '@/lib/utils' );
      expect( cn ).toBeDefined();
      expect( typeof cn ).toBe( 'function' );
    } );

    it( 'should merge class names correctly', () =>
    {
      const { cn } = require( '@/lib/utils' );
      const result = cn( 'class1', 'class2' );
      expect( result ).toContain( 'class1' );
      expect( result ).toContain( 'class2' );
    } );
  } );

  describe( 'Widget Registry', () =>
  {
    it( 'should have widget registry', () =>
    {
      const { widgetRegistry } = require( '@/components/widgets/WidgetRegistry' );
      expect( widgetRegistry ).toBeDefined();
      expect( widgetRegistry.getAll ).toBeDefined();
    } );

    it( 'should have default widgets registered', () =>
    {
      const { widgetRegistry } = require( '@/components/widgets/WidgetRegistry' );
      const widgets = widgetRegistry.getAll();
      expect( widgets.length ).toBeGreaterThan( 0 );
    } );
  } );
} );


