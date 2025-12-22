'use client';

import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { 
  PlayCircle, 
  StopCircle, 
  RefreshCw, 
  Server, 
  AlertCircle, 
  Globe,
  Laptop,
  CheckCircle,
  Info
} from 'lucide-react';
import { motion } from 'framer-motion';
import { 
  startInferenceServer, 
  stopInferenceServer, 
  getInferenceStatus,
  isTauriContext,
  type InferenceStatus 
} from '@/lib/tauri/commands';

export function LocalServerControl() {
  const [status, setStatus] = useState<InferenceStatus>({
    running: false,
    port: 8080,
    url: 'http://127.0.0.1:8080',
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [serverReachable, setServerReachable] = useState(false);
  const isDesktop = isTauriContext();

  // Universal health check (works for both web and desktop)
  const checkServerHealth = async (): Promise<boolean> => {
    try {
      const response = await fetch('http://127.0.0.1:8080/health', { 
        method: 'GET',
        signal: AbortSignal.timeout(2000) 
      });
      return response.ok;
    } catch (e) {
      return false;
    }
  };

  // Poll status every 3 seconds (cross-platform)
  useEffect(() => {
    const checkStatus = async () => {
      if (isDesktop) {
        // Desktop: use Tauri commands for full control
        const currentStatus = await getInferenceStatus();
        setStatus(currentStatus);
        setServerReachable(currentStatus.running);
      } else {
        // Web: check via HTTP health endpoint
        const reachable = await checkServerHealth();
        setServerReachable(reachable);
        if (reachable) {
          setStatus({ running: true, port: 8080, url: 'http://127.0.0.1:8080' });
        } else {
          setStatus({ running: false, port: 8080, url: 'http://127.0.0.1:8080' });
        }
      }
    };

    checkStatus();
    const interval = setInterval(checkStatus, 3000);

    return () => clearInterval(interval);
  }, [isDesktop]);

  const handleStart = async () => {
    setLoading(true);
    setError(null);

    try {
      const serverUrl = await startInferenceServer();
      if (serverUrl) {
        setStatus({ ...status, running: true, url: serverUrl });
        setServerReachable(true);
      } else {
        setError('Failed to start server. Please build the inference server first.');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to start server');
    } finally {
      setLoading(false);
    }
  };

  const handleStop = async () => {
    setLoading(true);
    setError(null);

    try {
      const success = await stopInferenceServer();
      if (success) {
        setStatus({ ...status, running: false });
        setServerReachable(false);
      } else {
        setError('Failed to stop server');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to stop server');
    } finally {
      setLoading(false);
    }
  };

  const handleRefresh = async () => {
    setLoading(true);
    try {
      if (isDesktop) {
        const currentStatus = await getInferenceStatus();
        setStatus(currentStatus);
        setServerReachable(currentStatus.running);
      } else {
        const reachable = await checkServerHealth();
        setServerReachable(reachable);
      }
    } catch (err) {
      console.error('Failed to refresh status:', err);
    } finally {
      setLoading(false);
    }
  };

  // Web browser view (monitor-only mode)
  if (!isDesktop) {
    return (
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Globe className="h-5 w-5" />
              <div>
                <CardTitle>Local Inference Server</CardTitle>
                <CardDescription>Cross-platform ML inference via HTTP</CardDescription>
              </div>
            </div>
            <Badge 
              variant={serverReachable ? "default" : "secondary"}
              className={serverReachable ? "bg-green-500" : ""}
            >
              {serverReachable ? 'Connected' : 'Not Running'}
            </Badge>
          </div>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Status Display */}
          <motion.div 
            className="grid grid-cols-2 gap-4 p-4 bg-muted rounded-lg"
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
          >
            <div>
              <p className="text-sm text-muted-foreground">Endpoint</p>
              <p className="font-mono text-xs font-medium">{status.url}</p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Status</p>
              <p className="font-medium">{serverReachable ? '✅ Reachable' : '❌ Offline'}</p>
            </div>
          </motion.div>

          {serverReachable ? (
            <Alert>
              <CheckCircle className="h-4 w-4" />
              <AlertDescription>
                <strong>Server is running!</strong> Your browser can now use local ML inference.
                <br />
                <span className="text-xs text-muted-foreground mt-1 block">
                  Set <code className="bg-muted px-1 py-0.5 rounded">useLocal: true</code> in <code>config/providers.json</code>
                </span>
              </AlertDescription>
            </Alert>
          ) : (
            <Alert>
              <Info className="h-4 w-4" />
              <AlertDescription>
                <strong>No server detected.</strong> To enable local inference:
                <ol className="list-decimal list-inside space-y-1 mt-2 text-xs">
                  <li>Download the desktop app for one-click management, or</li>
                  <li>Run manually: <code className="bg-muted px-1 py-0.5 rounded block mt-1">./rust_backend/target/release/inference_server --port 8080</code></li>
                </ol>
              </AlertDescription>
            </Alert>
          )}

          {/* Refresh Button */}
          <Button 
            onClick={handleRefresh} 
            disabled={loading}
            variant="outline"
            className="w-full"
          >
            <RefreshCw className={`mr-2 h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
            Refresh Status
          </Button>

          {/* Platform Support */}
          <div className="pt-4 border-t space-y-2">
            <p className="text-sm font-semibold">Platform Support:</p>
            <div className="grid grid-cols-2 gap-2">
              <Badge variant="outline" className="justify-center py-2">
                <Globe className="mr-1 h-3 w-3" />
                Web Browser
              </Badge>
              <Badge variant="outline" className="justify-center py-2">
                <Laptop className="mr-1 h-3 w-3" />
                Desktop (Win/Mac/Linux)
              </Badge>
              <Badge variant="outline" className="justify-center py-2">
                📱 Mobile (via Network)
              </Badge>
              <Badge variant="outline" className="justify-center py-2">
                ☁️ Remote Server
              </Badge>
            </div>
          </div>

          {/* Available Endpoints */}
          {serverReachable && (
            <motion.div 
              className="text-xs space-y-2 pt-4 border-t"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
            >
              <p className="font-semibold text-sm">Available Endpoints:</p>
              <ul className="list-disc list-inside space-y-1 text-muted-foreground ml-2">
                <li><code>/health</code> - Server status & system info</li>
                <li><code>/v1/models</code> - List available models</li>
                <li><code>/v1/chat/completions</code> - Chat API (OpenAI-compatible)</li>
              </ul>
            </motion.div>
          )}
        </CardContent>
      </Card>
    );
  }

  // Desktop view (full control mode)
  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Laptop className="h-5 w-5" />
            <div>
              <CardTitle>Local Inference Server</CardTitle>
              <CardDescription>Privacy-focused local ML with Candle + Qwen3-1.7B</CardDescription>
            </div>
          </div>
          <Badge 
            variant={status.running ? "default" : "secondary"}
            className={status.running ? "bg-green-500" : ""}
          >
            {status.running ? 'Running' : 'Stopped'}
          </Badge>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Status Information */}
        <motion.div 
          className="grid grid-cols-2 gap-4 p-4 bg-muted rounded-lg"
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
        >
          <div>
            <p className="text-sm text-muted-foreground">Port</p>
            <p className="font-mono font-medium">{status.port}</p>
          </div>
          <div>
            <p className="text-sm text-muted-foreground">URL</p>
            <p className="font-mono text-xs font-medium truncate">{status.url}</p>
          </div>
        </motion.div>

        {/* Error Display */}
        {error && (
          <motion.div 
            className="flex items-center gap-2 p-3 bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 rounded-lg"
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
          >
            <AlertCircle className="h-4 w-4" />
            <p className="text-sm">{error}</p>
          </motion.div>
        )}

        {/* Control Buttons */}
        <div className="flex gap-2">
          {!status.running ? (
            <Button 
              onClick={handleStart} 
              disabled={loading}
              className="flex-1"
            >
              {loading ? (
                <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
              ) : (
                <PlayCircle className="mr-2 h-4 w-4" />
              )}
              Start Server
            </Button>
          ) : (
            <Button 
              onClick={handleStop} 
              disabled={loading}
              variant="destructive"
              className="flex-1"
            >
              {loading ? (
                <RefreshCw className="mr-2 h-4 w-4 animate-spin" />
              ) : (
                <StopCircle className="mr-2 h-4 w-4" />
              )}
              Stop Server
            </Button>
          )}
          <Button 
            onClick={handleRefresh} 
            disabled={loading}
            variant="outline"
            size="icon"
          >
            <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
          </Button>
        </div>

        {/* Build Instructions */}
        {!status.running && (
          <div className="text-sm text-muted-foreground space-y-2">
            <p className="font-medium">First time setup:</p>
            <ol className="list-decimal list-inside space-y-1 ml-2">
              <li>Open terminal in project root</li>
              <li><code className="bg-muted px-1 py-0.5 rounded">cd rust_backend</code></li>
              <li><code className="bg-muted px-1 py-0.5 rounded">cargo build --release</code></li>
              <li>Wait ~5-10 minutes for first build</li>
              <li>Click "Start Server" above</li>
            </ol>
          </div>
        )}

        {/* Features List */}
        {status.running && (
          <motion.div 
            className="text-sm space-y-2"
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.2 }}
          >
            <p className="font-medium text-muted-foreground">Active Features:</p>
            <ul className="space-y-1 ml-4">
              <li className="flex items-center gap-2">
                <div className="h-1.5 w-1.5 rounded-full bg-green-500" />
                <span className="text-muted-foreground">Qwen3-1.7B (Q4_K_M quantization)</span>
              </li>
              <li className="flex items-center gap-2">
                <div className="h-1.5 w-1.5 rounded-full bg-green-500" />
                <span className="text-muted-foreground">Candle-powered inference</span>
              </li>
              <li className="flex items-center gap-2">
                <div className="h-1.5 w-1.5 rounded-full bg-green-500" />
                <span className="text-muted-foreground">OpenAI-compatible API</span>
              </li>
              <li className="flex items-center gap-2">
                <div className="h-1.5 w-1.5 rounded-full bg-green-500" />
                <span className="text-muted-foreground">100% privacy (local only)</span>
              </li>
            </ul>
          </motion.div>
        )}

        {/* Platform Info */}
        <div className="pt-4 border-t text-xs text-muted-foreground">
          <p><strong>Desktop Mode:</strong> Full server control with start/stop/restart</p>
          <p className="mt-1"><strong>Web Mode:</strong> Connect to already-running server (read-only)</p>
        </div>
      </CardContent>
    </Card>
  );
}
