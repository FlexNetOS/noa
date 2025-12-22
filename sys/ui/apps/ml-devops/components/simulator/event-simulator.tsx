'use client';

/**
 * Event Simulator - Testing tool for event-driven architecture
 * 
 * Provides UI to manually trigger various event types:
 * - Token streaming
 * - Widget lifecycle (mount/update/unmount)
 * - Chat messages
 * - Status updates
 * 
 * Rust/Dioxus equivalent:
 * - Simple button handlers that emit events to channel
 */

import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { useEventEmitter } from '@/lib/hooks/use-event-stream';
import { EventFactory } from '@/lib/events/types';
import {
  Play,
  Square,
  FileText,
  Code,
  Activity,
  BarChart3,
  MessageSquare,
  AlertCircle,
} from 'lucide-react';
import { useState } from 'react';

export function EventSimulator({ className = '' }: { className?: string }) {
  const { emit } = useEventEmitter();
  const [mountedWidgets, setMountedWidgets] = useState<Set<string>>(new Set());

  const simulateTokenStream = async () => {
    const messageId = `sim_msg_${Date.now()}`;
    
    // Emit message start
    emit({
      id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'MESSAGE_SENT',
      timestamp: Date.now(),
      messageId,
      content: '',
      role: 'assistant' as const,
    });

    // Simulate token streaming
    const tokens = [
      'This ',
      'is ',
      'a ',
      'simulated ',
      'streaming ',
      'message. ',
      'Each ',
      'word ',
      'arrives ',
      'as ',
      'a ',
      'separate ',
      'token ',
      'event.',
    ];

    let fullContent = '';
    for (const token of tokens) {
      await new Promise(resolve => setTimeout(resolve, 100));
      fullContent += token;
      emit({
        id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type: 'TOKEN_STREAMED',
        timestamp: Date.now(),
        messageId,
        token,
        isComplete: false,
      });
    }

    // Emit completion
    emit({
      id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'MESSAGE_COMPLETED',
      timestamp: Date.now(),
      messageId,
      content: fullContent,
    });
  };

  const mountWidget = (type: 'TextBlock' | 'CodeBlock' | 'StatusIndicator' | 'SimpleChart') => {
    const widgetId = `sim_widget_${type}_${Date.now()}`;
    
    let config;
    switch (type) {
      case 'TextBlock':
        config = {
          type,
          props: {
            content: '## Simulated Text Widget\n\nThis widget was mounted via the event simulator. It demonstrates **markdown rendering** and `code formatting`.',
            markdown: true,
          },
        };
        break;
      case 'CodeBlock':
        config = {
          type,
          props: {
            language: 'rust',
            code: `// Rust example - future implementation\n#[async_trait]\ntrait EventHandler {\n    async fn handle(&self, event: Event) -> Result<()>;\n}`,
          },
        };
        break;
      case 'StatusIndicator':
        config = {
          type,
          props: {
            status: 'success',
            message: 'Widget mounted successfully',
          },
        };
        break;
      case 'SimpleChart':
        config = {
          type,
          props: {
            title: 'Sample Metrics',
            data: [
              { name: 'Events', value: 120 },
              { name: 'Widgets', value: 45 },
              { name: 'Messages', value: 80 },
              { name: 'Replays', value: 15 },
            ],
            type: 'bar',
          },
        };
        break;
    }

    emit({
      id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'WIDGET_MOUNTED',
      timestamp: Date.now(),
      widgetId,
      config,
    });

    setMountedWidgets(prev => new Set(prev).add(widgetId));
  };

  const updateWidget = (widgetId: string) => {
    emit({
      id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'WIDGET_UPDATED',
      timestamp: Date.now(),
      widgetId,
      updates: {
        props: {
          message: `Updated at ${new Date().toLocaleTimeString()}`,
        },
      },
    });
  };

  const unmountLastWidget = () => {
    const lastWidget = Array.from(mountedWidgets).pop();
    if (lastWidget) {
      emit({
        id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type: 'WIDGET_UNMOUNTED',
        timestamp: Date.now(),
        widgetId: lastWidget,
      });
      setMountedWidgets(prev => {
        const next = new Set(prev);
        next.delete(lastWidget);
        return next;
      });
    }
  };

  const changeStatus = (status: 'idle' | 'processing' | 'success' | 'error') => {
    emit({
      id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      type: 'STATUS_CHANGED',
      timestamp: Date.now(),
      status,
      message: `Status changed to ${status} via simulator`,
    });
  };

  return (
    <Card className={`p-6 ${className}`}>
      <div className="flex items-center gap-2 mb-6">
        <Play className="w-5 h-5 text-green-500" />
        <h2 className="text-xl font-bold">Event Simulator</h2>
      </div>

      <div className="space-y-6">
        {/* Token Streaming */}
        <div>
          <h3 className="text-sm font-semibold mb-2 flex items-center gap-2">
            <MessageSquare className="w-4 h-4" />
            Token Streaming
          </h3>
          <Button onClick={simulateTokenStream} className="w-full">
            Simulate Streaming Message
          </Button>
        </div>

        {/* Widget Mounting */}
        <div>
          <h3 className="text-sm font-semibold mb-2 flex items-center gap-2">
            <Square className="w-4 h-4" />
            Widget Lifecycle
          </h3>
          <div className="grid grid-cols-2 gap-2">
            <Button
              onClick={() => mountWidget('TextBlock')}
              variant="outline"
              className="flex items-center gap-2"
            >
              <FileText className="w-4 h-4" />
              Mount Text
            </Button>
            <Button
              onClick={() => mountWidget('CodeBlock')}
              variant="outline"
              className="flex items-center gap-2"
            >
              <Code className="w-4 h-4" />
              Mount Code
            </Button>
            <Button
              onClick={() => mountWidget('StatusIndicator')}
              variant="outline"
              className="flex items-center gap-2"
            >
              <Activity className="w-4 h-4" />
              Mount Status
            </Button>
            <Button
              onClick={() => mountWidget('SimpleChart')}
              variant="outline"
              className="flex items-center gap-2"
            >
              <BarChart3 className="w-4 h-4" />
              Mount Chart
            </Button>
          </div>
          <Button
            onClick={unmountLastWidget}
            variant="destructive"
            className="w-full mt-2"
            disabled={mountedWidgets.size === 0}
          >
            Unmount Last Widget ({mountedWidgets.size} mounted)
          </Button>
        </div>

        {/* Status Changes */}
        <div>
          <h3 className="text-sm font-semibold mb-2 flex items-center gap-2">
            <AlertCircle className="w-4 h-4" />
            Status Updates
          </h3>
          <div className="grid grid-cols-2 gap-2">
            <Button
              onClick={() => changeStatus('idle')}
              variant="outline"
              className="text-gray-600"
            >
              Idle
            </Button>
            <Button
              onClick={() => changeStatus('processing')}
              variant="outline"
              className="text-blue-600"
            >
              Processing
            </Button>
            <Button
              onClick={() => changeStatus('success')}
              variant="outline"
              className="text-green-600"
            >
              Success
            </Button>
            <Button
              onClick={() => changeStatus('error')}
              variant="outline"
              className="text-red-600"
            >
              Error
            </Button>
          </div>
        </div>
      </div>
    </Card>
  );
}
