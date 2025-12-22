'use client';

/**
 * MessageList - Displays chat messages with streaming support
 * 
 * Subscribes to MESSAGE_SENT, TOKEN_STREAMED, and MESSAGE_COMPLETED events
 * 
 * Rust/Dioxus equivalent:
 * - use_signal for message state
 * - use_effect to subscribe to event channel
 */

import { useEffect, useState } from 'react';
import { useEventStream } from '@/lib/hooks/use-event-stream';
import { motion } from 'framer-motion';
import { User, Bot } from 'lucide-react';

interface Message {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  isStreaming?: boolean;
}

export function MessageList() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [, stream] = useEventStream();

  useEffect(() => {
    const handler = (event: any) => {
      if (event.type === 'MESSAGE_SENT') {
        setMessages(prev => [
          ...prev,
          {
            id: event.messageId,
            role: event.role,
            content: event.content,
            isStreaming: event.role === 'assistant' && !event.content,
          },
        ]);
      } else if (event.type === 'TOKEN_STREAMED') {
        setMessages(prev =>
          prev.map(msg =>
            msg.id === event.messageId
              ? { ...msg, content: msg.content + event.token }
              : msg
          )
        );
      } else if (event.type === 'MESSAGE_COMPLETED') {
        setMessages(prev =>
          prev.map(msg =>
            msg.id === event.messageId
              ? { ...msg, content: event.content, isStreaming: false }
              : msg
          )
        );
      }
    };

    return stream.subscribe(handler);
  }, [stream]);

  if (messages.length === 0) {
    return (
      <div className="flex items-center justify-center h-64 text-gray-400">
        <p>No messages yet. Start a conversation!</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {messages.map((message, index) => (
        <motion.div
          key={message.id}
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.3 }}
          className={`flex gap-3 ${
            message.role === 'user' ? 'justify-end' : 'justify-start'
          }`}
        >
          {message.role === 'assistant' && (
            <div className="flex-shrink-0 w-8 h-8 rounded-full bg-gradient-to-br from-blue-500 to-purple-500 flex items-center justify-center">
              <Bot className="w-5 h-5 text-white" />
            </div>
          )}
          <div
            className={`max-w-[70%] rounded-lg px-4 py-2 ${
              message.role === 'user'
                ? 'bg-blue-500 text-white'
                : 'bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100'
            }`}
          >
            <p className="text-sm whitespace-pre-wrap break-words">{message.content}</p>
            {message.isStreaming && (
              <motion.span
                animate={{ opacity: [0, 1, 0] }}
                transition={{ duration: 1, repeat: Infinity }}
                className="inline-block ml-1"
              >
                ▋
              </motion.span>
            )}
          </div>
          {message.role === 'user' && (
            <div className="flex-shrink-0 w-8 h-8 rounded-full bg-gradient-to-br from-green-500 to-teal-500 flex items-center justify-center">
              <User className="w-5 h-5 text-white" />
            </div>
          )}
        </motion.div>
      ))}
    </div>
  );
}
