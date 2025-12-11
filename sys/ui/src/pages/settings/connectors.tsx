'use client';

import { useMemo, useState } from 'react';
import { ChevronDown, PlugZap, RefreshCw, Settings2 } from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';
import OAuthFlow from '@/components/connectors/OAuthFlow';
import StatusDashboard, { ConnectorSummary } from '@/components/connectors/StatusDashboard';
import { cn } from '@/lib/utils';

type FeatureFlag = {
  id: string;
  label: string;
  description: string;
  enabled: boolean;
};

const initialFlags: FeatureFlag[] = [
  { id: 'connectors.enabled', label: 'Connectors', description: 'Master switch for all connectors', enabled: true },
  { id: 'connectors.github', label: 'GitHub', description: 'GitHub OAuth + repo access', enabled: true },
  { id: 'connectors.google', label: 'Gmail/Google', description: 'OAuth for Gmail + drive metadata', enabled: true },
  { id: 'connectors.openai', label: 'OpenAI', description: 'API key-based connector', enabled: true },
  { id: 'connectors.claude', label: 'Claude', description: 'Anthropic Claude connector', enabled: true },
  { id: 'connectors.cloud_storage', label: 'Cloud Storage', description: 'S3 / GCS credentials', enabled: true },
  { id: 'connectors.email', label: 'Email', description: 'SMTP/IMAP connector', enabled: false },
];

const initialConnectors: ConnectorSummary[] = [
  { id: 'github', name: 'GitHub', status: 'ready', detail: 'Token cached; repo + org scopes granted', lastChecked: '2m ago', latencyMs: 120 },
  { id: 'google', name: 'Gmail / Google', status: 'degraded', detail: 'Awaiting OAuth approval', lastChecked: '5m ago', latencyMs: 180 },
  { id: 'openai', name: 'OpenAI', status: 'ready', detail: 'API key detected', lastChecked: 'Just now' },
  { id: 'claude', name: 'Claude', status: 'ready', detail: 'Anthropic key loaded', lastChecked: '30s ago' },
  { id: 'cloud', name: 'Cloud Storage', status: 'offline', detail: 'Network unreachable; offline cache active', lastChecked: '8m ago' },
  { id: 'email', name: 'Email (SMTP/IMAP)', status: 'disabled', detail: 'Feature flag disabled', lastChecked: '—' },
];

const oauthProfiles = {
  github: {
    label: 'GitHub',
    redirectUri: 'https://localhost:3000/api/oauth/github/callback',
    scopes: ['repo', 'read:org', 'workflow'],
  },
  google: {
    label: 'Google',
    redirectUri: 'https://localhost:3000/api/oauth/google/callback',
    scopes: ['openid', 'email', 'https://www.googleapis.com/auth/gmail.readonly'],
  },
};

export default function ConnectorsSettingsPage() {
  const [flags, setFlags] = useState<FeatureFlag[]>(initialFlags);
  const [connectors, setConnectors] = useState<ConnectorSummary[]>(initialConnectors);
  const [selectedOAuthProvider, setSelectedOAuthProvider] = useState<'github' | 'google'>('github');

  const activeFlags = useMemo(() => new Set(flags.filter((f) => f.enabled).map((f) => f.id)), [flags]);

  const toggleFlag = (flagId: string) => {
    setFlags((prev) =>
      prev.map((flag) =>
        flag.id === flagId ? { ...flag, enabled: !flag.enabled } : flag
      )
    );

    setConnectors((prev) =>
      prev.map((conn) => {
        const flagName = `connectors.${conn.id === 'cloud' ? 'cloud_storage' : conn.id}`;
        const masterEnabled = activeFlags.has('connectors.enabled') || flagId === 'connectors.enabled';
        if (flagId === flagName || flagId === 'connectors.enabled') {
          const enabled = flagId === 'connectors.enabled'
            ? !activeFlags.has(flagId)
            : !activeFlags.has(flagId);
          if (!enabled || !masterEnabled) {
            return { ...conn, status: 'disabled', detail: 'Disabled via feature flag' };
          }
          return { ...conn, status: conn.status === 'disabled' ? 'degraded' : conn.status };
        }
        return conn;
      })
    );
  };

  const refreshStatuses = () => {
    const timestamp = new Date().toLocaleTimeString();
    setConnectors((prev) =>
      prev.map((conn) => ({
        ...conn,
        lastChecked: timestamp,
        detail:
          conn.status === 'degraded'
            ? 'Still waiting for authorization'
            : conn.detail,
      }))
    );
  };

  const selectedProfile = oauthProfiles[selectedOAuthProvider];

  return (
    <MainLayout>
      <div className="space-y-6">
        <div className="flex items-center gap-3">
          <PlugZap className="w-8 h-8 text-blue-400" />
          <div>
            <h1 className="text-3xl font-bold text-slate-100">Connectors</h1>
            <p className="text-slate-400">
              Manage OAuth flows, feature flags, and status for external services.
            </p>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4">
          <div className="lg:col-span-2 space-y-4">
            <FeatureFlagPanel flags={flags} onToggle={toggleFlag} />
            <StatusDashboard connectors={connectors} onRefresh={refreshStatuses} />
          </div>
          <div className="space-y-4">
            <div className="bg-slate-900/60 border border-slate-700 rounded-xl p-5 space-y-3">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-xs uppercase tracking-wide text-slate-400 mb-1">OAuth</p>
                  <h3 className="text-lg font-semibold text-slate-100">Authorization Flow</h3>
                </div>
                <div className="relative inline-block">
                  <select
                    value={selectedOAuthProvider}
                    onChange={(e) => setSelectedOAuthProvider(e.target.value as 'github' | 'google')}
                    className="appearance-none bg-slate-800 text-slate-100 text-sm px-3 py-2 rounded-lg border border-slate-700 pr-8"
                  >
                    <option value="github">GitHub</option>
                    <option value="google">Google</option>
                  </select>
                  <ChevronDown className="w-4 h-4 text-slate-400 absolute right-2 top-2.5 pointer-events-none" />
                </div>
              </div>
              <OAuthFlow
                provider={selectedProfile.label}
                redirectUri={selectedProfile.redirectUri}
                scopes={selectedProfile.scopes}
              />
            </div>

            <div className="bg-slate-900/60 border border-slate-700 rounded-xl p-5 space-y-2">
              <div className="flex items-center gap-2 text-slate-100 font-semibold">
                <RefreshCw className="w-4 h-4 text-blue-300" />
                Graceful degradation
              </div>
              <p className="text-sm text-slate-400">
                When the network drops, NOA serves cached connector data for up to 10 minutes and
                marks the connector as offline instead of blocking flows.
              </p>
            </div>
          </div>
        </div>
      </div>
    </MainLayout>
  );
}

function FeatureFlagPanel({
  flags,
  onToggle,
}: {
  flags: FeatureFlag[];
  onToggle: (id: string) => void;
}) {
  return (
    <div className="bg-slate-900/60 border border-slate-700 rounded-xl p-5">
      <div className="flex items-center gap-2 mb-4">
        <Settings2 className="w-5 h-5 text-blue-300" />
        <h3 className="text-lg font-semibold text-slate-100">Feature Flags</h3>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
        {flags.map((flag) => (
          <div
            key={flag.id}
            className="flex items-start justify-between border border-slate-700 rounded-lg p-3 bg-slate-800/60"
          >
            <div>
              <p className="text-slate-100 font-medium">{flag.label}</p>
              <p className="text-xs text-slate-400">{flag.description}</p>
            </div>
            <label className="relative inline-flex items-center cursor-pointer">
              <input
                type="checkbox"
                className="sr-only peer"
                checked={flag.enabled}
                onChange={() => onToggle(flag.id)}
              />
              <div
                className={cn(
                  'relative w-11 h-6 rounded-full peer-focus:outline-none transition-colors',
                  flag.enabled ? 'bg-blue-600' : 'bg-slate-700'
                )}
              >
                <div
                  className={cn(
                    'absolute top-[2px] left-[2px] h-5 w-5 rounded-full bg-white transition-transform',
                    flag.enabled && 'translate-x-full'
                  )}
                />
              </div>
            </label>
          </div>
        ))}
      </div>
    </div>
  );
}
