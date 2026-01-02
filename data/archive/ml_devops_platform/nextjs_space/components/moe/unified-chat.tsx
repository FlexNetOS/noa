'use client';

import React, { useState, useRef, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Textarea } from '@/components/ui/textarea';
import { ScrollArea } from '@/components/ui/scroll-area';
import { 
  Send, 
  Bot, 
  User, 
  Zap, 
  RefreshCw,
  ChevronDown,
  Cpu,
  Cloud,
  Github,
  Code2,
  Settings,
  Sparkles
} from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import { useMOE, useMOEStats, useSharedResources, RoutingDecision, TaskAnalysis } from '@/lib/hooks/use-moe';
import { ProviderType } from '@/lib/providers/provider-manager';
import { ChatMessage } from '@/lib/providers/types';

interface Message extends ChatMessage {
  id: string;
  timestamp: Date;
  provider?: ProviderType;
  fromCache?: boolean;
  analysis?: TaskAnalysis;
}

const PROVIDER_ICONS: Record<ProviderType, React.ReactNode> = {
  github_copilot: <Github className="h-3 w-3" />,
  abacus_deepagent: <Bot className="h-3 w-3" />,
  claude_cli: <Zap className="h-3 w-3" />,
  openai_codex: <Code2 className="h-3 w-3" />,
  local_inference: <Cpu className="h-3 w-3" />
};

const PROVIDER_NAMES: Record<ProviderType, string> = {
  github_copilot: 'GitHub Copilot',
  abacus_deepagent: 'Abacus AI',
  claude_cli: 'Claude',
  openai_codex: 'OpenAI',
  local_inference: 'Local (Qwen3)'
};

export function UnifiedChat() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [showRouting, setShowRouting] = useState(false);
  const [preferProvider, setPreferProvider] = useState<ProviderType | undefined>();
  const scrollRef = useRef<HTMLDivElement>(null);

  const { execute, isRouting, lastDecision, lastAnalysis } = useMOE();
  const { stats } = useMOEStats();
  const { resources } = useSharedResources();

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || isLoading) return;

    const userMessage: Message = {
      id: `msg_${Date.now()}`,
      role: 'user',
      content: input.trim(),
      timestamp: new Date()
    };

    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setIsLoading(true);

    try {
      // Build message history for context
      const history: ChatMessage[] = messages.map(m => ({ role: m.role, content: m.content }));
      history.push({ role: 'user', content: userMessage.content });

      // Execute with MOE routing
      const result = await execute(history, { preferProvider });

      const assistantMessage: Message = {
        id: `msg_${Date.now()}_response`,
        role: 'assistant',
        content: result.result,
        timestamp: new Date(),
        provider: result.provider,
        fromCache: result.fromCache,
        analysis: lastAnalysis || undefined
      };

      setMessages(prev => [...prev, assistantMessage]);
    } catch (error) {
      const errorMessage: Message = {
        id: `msg_${Date.now()}_error`,
        role: 'assistant',
        content: `Error: ${(error as Error).message}`,
        timestamp: new Date()
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="flex flex-col h-full">
      {/* Header with routing info */}
      <div className="flex items-center justify-between p-3 border-b bg-muted/30">
        <div className="flex items-center gap-2">
          <Sparkles className="h-4 w-4 text-primary" />
          <span className="text-sm font-medium">MOE Unified Chat</span>
          {stats && (
            <Badge variant="secondary" className="text-xs">
              {stats.totalRoutes} routes
            </Badge>
          )}
        </div>
        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setShowRouting(!showRouting)}
            className="text-xs"
          >
            <Settings className="h-3 w-3 mr-1" />
            {showRouting ? 'Hide' : 'Routing'}
          </Button>
        </div>
      </div>

      {/* Routing panel */}
      <AnimatePresence>
        {showRouting && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            className="border-b bg-muted/20 overflow-hidden"
          >
            <div className="p-3 space-y-2">
              <div className="text-xs text-muted-foreground">Prefer Provider:</div>
              <div className="flex flex-wrap gap-1">
                <Button
                  size="sm"
                  variant={!preferProvider ? 'default' : 'outline'}
                  onClick={() => setPreferProvider(undefined)}
                  className="text-xs h-7"
                >
                  Auto
                </Button>
                {(['abacus_deepagent', 'github_copilot', 'claude_cli', 'openai_codex', 'local_inference'] as ProviderType[]).map(p => (
                  <Button
                    key={p}
                    size="sm"
                    variant={preferProvider === p ? 'default' : 'outline'}
                    onClick={() => setPreferProvider(p)}
                    className="text-xs h-7"
                  >
                    {PROVIDER_ICONS[p]}
                    <span className="ml-1">{PROVIDER_NAMES[p]}</span>
                  </Button>
                ))}
              </div>
              {lastDecision && (
                <div className="text-xs text-muted-foreground mt-2">
                  Last route: <span className="text-foreground">{PROVIDER_NAMES[lastDecision.provider]}</span>
                  {' · '}{lastDecision.reason}
                </div>
              )}
              {resources && (
                <div className="flex gap-2 text-xs text-muted-foreground">
                  <span>{resources.skills.length} skills</span>
                  <span>·</span>
                  <span>{resources.agents.length} agents</span>
                  <span>·</span>
                  <span>{resources.tools.length} tools</span>
                </div>
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>

      {/* Messages */}
      <ScrollArea ref={scrollRef} className="flex-1 p-4">
        <div className="space-y-4">
          {messages.length === 0 && (
            <div className="text-center text-muted-foreground py-8">
              <Bot className="h-12 w-12 mx-auto mb-4 opacity-50" />
              <p className="text-sm">Start a conversation. MOE will automatically route to the best provider.</p>
              <div className="flex justify-center gap-2 mt-4">
                {resources?.commands.slice(0, 3).map(cmd => (
                  <Badge key={cmd.id} variant="outline" className="text-xs cursor-pointer" onClick={() => setInput(cmd.name)}>
                    {cmd.name}
                  </Badge>
                ))}
              </div>
            </div>
          )}

          {messages.map((message, index) => (
            <motion.div
              key={message.id}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.05 }}
              className={`flex gap-3 ${message.role === 'user' ? 'justify-end' : 'justify-start'}`}
            >
              {message.role === 'assistant' && (
                <div className="flex-shrink-0 w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center">
                  {message.provider ? PROVIDER_ICONS[message.provider] : <Bot className="h-4 w-4" />}
                </div>
              )}
              <div className={`max-w-[80%] ${message.role === 'user' ? 'order-first' : ''}`}>
                <div
                  className={`rounded-lg p-3 ${
                    message.role === 'user'
                      ? 'bg-primary text-primary-foreground'
                      : 'bg-muted'
                  }`}
                >
                  <p className="text-sm whitespace-pre-wrap">{message.content}</p>
                </div>
                {message.role === 'assistant' && message.provider && (
                  <div className="flex items-center gap-2 mt-1 text-xs text-muted-foreground">
                    <span>{PROVIDER_NAMES[message.provider]}</span>
                    {message.fromCache && <Badge variant="outline" className="text-[10px] h-4">cached</Badge>}
                    {message.analysis && (
                      <Badge variant="secondary" className="text-[10px] h-4">
                        {message.analysis.taskType} · {message.analysis.complexity}
                      </Badge>
                    )}
                  </div>
                )}
              </div>
              {message.role === 'user' && (
                <div className="flex-shrink-0 w-8 h-8 rounded-full bg-primary flex items-center justify-center">
                  <User className="h-4 w-4 text-primary-foreground" />
                </div>
              )}
            </motion.div>
          ))}

          {isLoading && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="flex gap-3"
            >
              <div className="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center">
                <RefreshCw className="h-4 w-4 animate-spin" />
              </div>
              <div className="bg-muted rounded-lg p-3">
                <p className="text-sm text-muted-foreground">
                  {isRouting ? 'Routing to best provider...' : 'Thinking...'}
                </p>
              </div>
            </motion.div>
          )}
        </div>
      </ScrollArea>

      {/* Input */}
      <div className="p-4 border-t">
        <div className="flex gap-2">
          <Textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Type a message... (Enter to send, Shift+Enter for newline)"
            className="min-h-[60px] max-h-[120px] resize-none"
            disabled={isLoading}
          />
          <Button
            onClick={handleSend}
            disabled={!input.trim() || isLoading}
            className="self-end"
          >
            <Send className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  );
}
