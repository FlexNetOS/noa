'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { 
  Github, 
  Bot, 
  Cpu, 
  Cloud, 
  Check, 
  X, 
  RefreshCw,
  Zap,
  Shield,
  Code2
} from 'lucide-react';
import { motion } from 'framer-motion';

type ProviderType = 
  | 'github_copilot'
  | 'abacus_deepagent'
  | 'claude_cli'
  | 'openai_codex'
  | 'local_inference';

interface ProviderStatus {
  type: ProviderType;
  name: string;
  connected: boolean;
  authMethod: 'oauth' | 'api_key' | 'builtin';
  capabilities: string[];
}

const PROVIDER_ICONS: Record<ProviderType, React.ReactNode> = {
  github_copilot: <Github className="h-5 w-5" />,
  abacus_deepagent: <Bot className="h-5 w-5" />,
  claude_cli: <Zap className="h-5 w-5" />,
  openai_codex: <Code2 className="h-5 w-5" />,
  local_inference: <Cpu className="h-5 w-5" />
};

const PROVIDER_COLORS: Record<ProviderType, string> = {
  github_copilot: 'bg-gray-900 hover:bg-gray-800',
  abacus_deepagent: 'bg-blue-600 hover:bg-blue-500',
  claude_cli: 'bg-orange-600 hover:bg-orange-500',
  openai_codex: 'bg-green-600 hover:bg-green-500',
  local_inference: 'bg-purple-600 hover:bg-purple-500'
};

export function ProviderSelector() {
  const [providers, setProviders] = useState<ProviderStatus[]>([]);
  const [activeProvider, setActiveProvider] = useState<ProviderType>('abacus_deepagent');
  const [loading, setLoading] = useState(true);
  const [connecting, setConnecting] = useState<ProviderType | null>(null);

  useEffect(() => {
    fetchProviders();
  }, []);

  const fetchProviders = async () => {
    try {
      const response = await fetch('/api/providers');
      const data = await response.json();
      setProviders(data.providers || []);
      setActiveProvider(data.activeProvider || 'abacus_deepagent');
    } catch (error) {
      console.error('Failed to fetch providers:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleConnect = async (type: ProviderType) => {
    setConnecting(type);
    
    // For GitHub, trigger OAuth flow
    if (type === 'github_copilot') {
      // This would trigger the oauth_token_manager flow
      window.open('/api/auth/github', '_blank');
    }
    
    // For Claude/OpenAI, show instructions for CLI auth
    if (type === 'claude_cli' || type === 'openai_codex') {
      alert(`To connect ${type === 'claude_cli' ? 'Claude' : 'OpenAI'} CLI:\n\n1. Open terminal\n2. Run: ${type === 'claude_cli' ? 'claude login' : 'openai auth login'}\n3. Complete browser authentication\n4. Refresh this page`);
    }
    
    setConnecting(null);
  };

  const handleSetActive = async (type: ProviderType) => {
    try {
      const response = await fetch('/api/providers', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action: 'set_active', provider: type })
      });
      const data = await response.json();
      if (data.success) {
        setActiveProvider(data.activeProvider);
      }
    } catch (error) {
      console.error('Failed to set active provider:', error);
    }
  };

  if (loading) {
    return (
      <Card>
        <CardContent className="p-6">
          <div className="flex items-center justify-center gap-2">
            <RefreshCw className="h-4 w-4 animate-spin" />
            <span>Loading providers...</span>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Cloud className="h-5 w-5" />
          AI Providers
        </CardTitle>
        <CardDescription>
          Connect and manage AI providers for code generation and assistance
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {providers.map((provider, index) => (
          <motion.div
            key={provider.type}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: index * 0.1 }}
            className={`p-4 rounded-lg border ${
              activeProvider === provider.type
                ? 'border-primary bg-primary/5'
                : 'border-border'
            }`}
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className={`p-2 rounded-lg text-white ${PROVIDER_COLORS[provider.type]}`}>
                  {PROVIDER_ICONS[provider.type]}
                </div>
                <div>
                  <div className="flex items-center gap-2">
                    <span className="font-medium">{provider.name}</span>
                    {provider.connected ? (
                      <Badge variant="outline" className="text-green-600 border-green-600">
                        <Check className="h-3 w-3 mr-1" /> Connected
                      </Badge>
                    ) : (
                      <Badge variant="outline" className="text-muted-foreground">
                        <X className="h-3 w-3 mr-1" /> Not Connected
                      </Badge>
                    )}
                    {activeProvider === provider.type && (
                      <Badge className="bg-primary">Active</Badge>
                    )}
                  </div>
                  <div className="flex gap-1 mt-1">
                    {provider.capabilities.slice(0, 3).map(cap => (
                      <Badge key={cap} variant="secondary" className="text-xs">
                        {cap.replace('_', ' ')}
                      </Badge>
                    ))}
                  </div>
                </div>
              </div>
              <div className="flex gap-2">
                {!provider.connected && provider.authMethod !== 'builtin' && (
                  <Button
                    size="sm"
                    variant="outline"
                    onClick={() => handleConnect(provider.type)}
                    disabled={connecting === provider.type}
                  >
                    {connecting === provider.type ? (
                      <RefreshCw className="h-4 w-4 animate-spin" />
                    ) : (
                      'Connect'
                    )}
                  </Button>
                )}
                {provider.connected && activeProvider !== provider.type && (
                  <Button
                    size="sm"
                    onClick={() => handleSetActive(provider.type)}
                  >
                    Use
                  </Button>
                )}
              </div>
            </div>
          </motion.div>
        ))}

        <div className="mt-4 p-3 bg-muted rounded-lg text-sm text-muted-foreground">
          <div className="flex items-center gap-2 mb-2">
            <Shield className="h-4 w-4" />
            <span className="font-medium">Authentication Methods</span>
          </div>
          <ul className="space-y-1 ml-6 list-disc">
            <li><strong>GitHub Copilot:</strong> OAuth via browser</li>
            <li><strong>Abacus AI:</strong> Built-in (no setup needed)</li>
            <li><strong>Claude/OpenAI CLI:</strong> Browser OAuth via CLI tools</li>
            <li><strong>Local Inference:</strong> No auth (runs locally)</li>
          </ul>
        </div>
      </CardContent>
    </Card>
  );
}
