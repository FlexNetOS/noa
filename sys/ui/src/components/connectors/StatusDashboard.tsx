'use client';

import { useMemo } from 'react';
import { AlertTriangle, CheckCircle2, Cloud, Mail, PlugZap, WifiOff } from 'lucide-react';
import { cn } from '@/lib/utils';

type ConnectorStatus = 'ready' | 'degraded' | 'offline' | 'disabled';

export interface ConnectorSummary {
  id: string;
  name: string;
  status: ConnectorStatus;
  detail: string;
  lastChecked: string;
  latencyMs?: number;
}

interface StatusDashboardProps {
  connectors?: ConnectorSummary[];
  onRefresh?: () => void;
}

const defaultConnectors: ConnectorSummary[] = [
  {
    id: 'github',
    name: 'GitHub',
    status: 'ready',
    detail: 'Token healthy, repo + org scopes present',
    lastChecked: '2m ago',
    latencyMs: 120,
  },
  {
    id: 'google',
    name: 'Gmail / Google',
    status: 'degraded',
    detail: 'Awaiting OAuth code; offline cache active',
    lastChecked: '5m ago',
    latencyMs: 180,
  },
  {
    id: 'openai',
    name: 'OpenAI',
    status: 'ready',
    detail: 'API key detected via environment',
    lastChecked: 'Just now',
  },
  {
    id: 'claude',
    name: 'Claude',
    status: 'ready',
    detail: 'Anthropic key loaded, rate limits normal',
    lastChecked: '30s ago',
  },
  {
    id: 'cloud',
    name: 'Cloud Storage',
    status: 'offline',
    detail: 'Network unreachable; serving from cache',
    lastChecked: '8m ago',
  },
  {
    id: 'email',
    name: 'Email (SMTP/IMAP)',
    status: 'disabled',
    detail: 'Feature flag disabled',
    lastChecked: '—',
  },
];

export default function StatusDashboard({ connectors, onRefresh }: StatusDashboardProps) {
  const items = useMemo(() => connectors ?? defaultConnectors, [connectors]);

  return (
    <div className="bg-slate-900/60 border border-slate-700 rounded-xl p-5">
      <div className="flex items-center justify-between mb-4">
        <div>
          <p className="text-xs uppercase tracking-wide text-slate-400 mb-1">Connector Health</p>
          <h3 className="text-xl font-semibold text-slate-100">Status Dashboard</h3>
        </div>
        <button
          onClick={onRefresh}
          aria-label="Refresh connector statuses"
          className="px-3 py-2 text-sm rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-100 border border-slate-700"
        >
          Refresh
        </button>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-3">
        {items.map((connector) => (
          <div
            key={connector.id}
            className="rounded-lg border border-slate-700 bg-slate-800/60 p-4 space-y-2"
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                {iconFor(connector.id)}
                <div>
                  <p className="text-sm text-slate-300">{connector.name}</p>
                  <p className="text-xs text-slate-500">Last check {connector.lastChecked}</p>
                </div>
              </div>
              <StatusPill status={connector.status} />
            </div>
            <p className="text-sm text-slate-300">{connector.detail}</p>
            {connector.latencyMs !== undefined && (
              <p className="text-xs text-slate-500">
                Latency: {connector.latencyMs}ms
              </p>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}

function StatusPill({ status }: { status: ConnectorStatus }) {
  const styles = {
    ready: 'bg-emerald-500/20 text-emerald-300 border-emerald-500/40',
    degraded: 'bg-amber-500/20 text-amber-200 border-amber-500/40',
    offline: 'bg-red-500/15 text-red-200 border-red-500/40',
    disabled: 'bg-slate-700 text-slate-300 border-slate-600',
  };

  const labels = {
    ready: 'Ready',
    degraded: 'Degraded',
    offline: 'Offline',
    disabled: 'Disabled',
  };

  return (
    <span className={cn('px-3 py-1 text-xs rounded-full border font-semibold', styles[status])}>
      {labels[status]}
    </span>
  );
}

function iconFor(id: string) {
  switch (id) {
    case 'github':
      return <PlugZap className="w-5 h-5 text-blue-300" />;
    case 'google':
      return <WifiOff className="w-5 h-5 text-amber-300" />;
    case 'openai':
      return <CheckCircle2 className="w-5 h-5 text-emerald-300" />;
    case 'claude':
      return <AlertTriangle className="w-5 h-5 text-purple-300" />;
    case 'cloud':
      return <Cloud className="w-5 h-5 text-cyan-300" />;
    case 'email':
      return <Mail className="w-5 h-5 text-slate-300" />;
    default:
      return <PlugZap className="w-5 h-5 text-slate-300" />;
  }
}
