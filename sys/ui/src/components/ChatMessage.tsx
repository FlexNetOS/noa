'use client';

import { User, Bot } from 'lucide-react';
import MarkdownRenderer from './MarkdownRenderer';
import { cn } from '@/lib/utils';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: string;
}

interface ChatMessageProps {
  message: Message;
}

/**
 * Chat Message Component
 *
 * Displays individual chat messages with markdown rendering support.
 */
export default function ChatMessage({ message }: ChatMessageProps) {
  const isUser = message.role === 'user';

  return (
    <div
      className={cn(
        'flex gap-4',
        isUser ? 'justify-end' : 'justify-start'
      )}
    >
      {!isUser && (
        <div className="w-8 h-8 rounded-full bg-blue-600 flex items-center justify-center flex-shrink-0">
          <Bot className="w-5 h-5 text-white" />
        </div>
      )}

      <div
        className={cn(
          'max-w-[80%] rounded-lg px-4 py-2',
          isUser
            ? 'bg-blue-600 text-white'
            : 'bg-slate-700 text-slate-100'
        )}
      >
        <MarkdownRenderer content={message.content} />
        <div className={cn(
          'text-xs mt-1',
          isUser ? 'text-blue-100' : 'text-slate-400'
        )}>
          {new Date(message.timestamp).toLocaleTimeString()}
        </div>
      </div>

      {isUser && (
        <div className="w-8 h-8 rounded-full bg-slate-600 flex items-center justify-center flex-shrink-0">
          <User className="w-5 h-5 text-white" />
        </div>
      )}
    </div>
  );
}

