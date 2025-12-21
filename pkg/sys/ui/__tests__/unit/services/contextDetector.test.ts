/**
 * Context Detector Service Unit Tests
 *
 * Tests for the context detection service.
 */

import { contextDetector, } from '@/services/contextDetector';

describe('Context Detector Service', () => {
  beforeEach(() => {
    contextDetector.reset();
  });

  describe('Context Detection', () => {
    it('should detect coding context from file path', () => {
      const context = contextDetector.detectContext({
        type: 'file_edit',
        path: 'src/components/Button.tsx',
      });

      expect(context.type).toBe('coding');
      expect(context.confidence).toBeGreaterThan(0.7);
    });

    it('should detect project management context', () => {
      const context = contextDetector.detectContext({
        type: 'task_created',
        keywords: ['task', 'deadline', 'milestone'],
      });

      expect(context.type).toBe('project_management');
      expect(context.confidence).toBeGreaterThan(0.7);
    });

    it('should detect research context', () => {
      const context = contextDetector.detectContext({
        type: 'search',
        keywords: ['research', 'analyze', 'study'],
      });

      expect(context.type).toBe('research');
      expect(context.confidence).toBeGreaterThan(0.7);
    });

    it('should default to general context', () => {
      const context = contextDetector.detectContext({
        type: 'unknown',
      });

      expect(context.type).toBe('general');
    });
  });

  describe('Context History', () => {
    it('should maintain context history', () => {
      contextDetector.detectContext({ type: 'coding', path: 'test.ts' });
      contextDetector.detectContext({ type: 'project_management', keywords: ['task'] });

      const history = contextDetector.getHistory();
      expect(history.length).toBe(2);
    });

    it('should limit history to 100 entries', () => {
      for (let i = 0; i < 150; i++) {
        contextDetector.detectContext({ type: 'test', data: { index: i } });
      }

      const history = contextDetector.getHistory();
      expect(history.length).toBe(100);
    });
  });

  describe('UI Adaptation Signals', () => {
    it('should generate signals for coding context', () => {
      const context = contextDetector.detectContext({
        type: 'coding',
        path: 'test.ts',
      });

      const signals = contextDetector.generateSignals(context);
      expect(signals.suggestedTools).toContain('Code Editor');
      expect(signals.uiAdaptations.length).toBeGreaterThan(0);
    });

    it('should generate signals for project management context', () => {
      const context = contextDetector.detectContext({
        type: 'project_management',
        keywords: ['task'],
      });

      const signals = contextDetector.generateSignals(context);
      expect(signals.suggestedTools).toContain('Task Board');
    });
  });
});


