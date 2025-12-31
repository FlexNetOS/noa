'use client';

/**
 * Chat Interface - Main chat component
 * 
 * Handles:
 * - Message display with streaming
 * - User input
 * - Widget rendering
 * 
 * Rust/Dioxus equivalent:
 * - use_coroutine for async message handling
 * - use_signal for local state
 * - Event channel for streaming
 */

import { useState } from 'react';
import { Card } from '@/components/ui/card';
import { MessageList } from './message-list';
import { MessageInput } from './message-input';
import { WidgetRegistry } from '../widgets/widget-registry';
import { useEventEmitter } from '@/lib/hooks/use-event-stream';
import { EventFactory, MessageSentEvent } from '@/lib/events/types';
import { createProvider } from '@/lib/providers/ai-provider';
import { MessageSquare } from 'lucide-react';

export function ChatInterface({ className = '' }: { className?: string }) {
  const { emit } = useEventEmitter();
  const [isProcessing, setIsProcessing] = useState(false);

  const handleSendMessage = async (content: string) => {
    if (!content.trim() || isProcessing) return;

    setIsProcessing(true);

    try {
      // Emit user message event
      const userMessage = EventFactory.createEvent<MessageSentEvent>('MESSAGE_SENT', {
        messageId: `msg_${Date.now()}_user`,
        content,
        role: 'user',
      });
      emit(userMessage);

      // Get AI response
      const provider = createProvider(false); // Use real provider
      const response = await provider.streamChat([
        { role: 'user', content },
      ]);

      // Emit assistant message start event
      const assistantMessage = EventFactory.createEvent<MessageSentEvent>('MESSAGE_SENT', {
        messageId: response.messageId,
        content: '',
        role: 'assistant',
      });
      emit(assistantMessage);

      // Stream tokens
      let fullContent = '';
      for await (const token of response.tokens) {
        fullContent += token;
        emit({
          id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
          type: 'TOKEN_STREAMED',
          timestamp: Date.now(),
          messageId: response.messageId,
          token,
          isComplete: false,
        });
      }

      // Emit completion event
      emit({
        id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type: 'MESSAGE_COMPLETED',
        timestamp: Date.now(),
        messageId: response.messageId,
        content: fullContent,
      });
    } catch (error) {
      console.error('Chat error:', error);
      emit({
        id: `evt_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type: 'STATUS_CHANGED',
        timestamp: Date.now(),
        status: 'error',
        message: 'Failed to process message',
      });
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className={`flex flex-col h-full ${className}`}>
      <Card className="flex-1 overflow-hidden flex flex-col">
        <div className="flex items-center gap-2 p-4 border-b bg-gradient-to-r from-blue-50 to-purple-50 dark:from-blue-900/20 dark:to-purple-900/20">
          <MessageSquare className="w-5 h-5 text-blue-500" />
          <h2 className="text-lg font-semibold">Chat Interface</h2>
        </div>
        
        <div className="flex-1 overflow-y-auto p-4">
          <MessageList />
          <WidgetRegistry className="mt-4" />
        </div>

        <div className="p-4 border-t bg-gray-50 dark:bg-gray-900">
          <MessageInput onSend={handleSendMessage} disabled={isProcessing} />
        </div>
      </Card>
    </div>
  );
}
