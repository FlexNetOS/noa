'use client';

import { useState } from 'react';
import { Settings } from 'lucide-react';
import MainLayout from '@/components/layout/MainLayout';
import AISettings from '@/components/settings/AISettings';
import ProvidersSettings from '@/components/settings/ProvidersSettings';
import IDESettings from '@/components/settings/IDESettings';
import CLISettings from '@/components/settings/CLISettings';
import SyncSettings from '@/components/settings/SyncSettings';
import MemorySettings from '@/components/settings/MemorySettings';
import SecuritySettings from '@/components/settings/SecuritySettings';
import PrivacySettings from '@/components/settings/PrivacySettings';
import ThemeSettings from '@/components/settings/ThemeSettings';
import { cn } from '@/lib/utils';

type SettingsTab =
  | 'ai'
  | 'providers'
  | 'ide'
  | 'cli'
  | 'sync'
  | 'memory'
  | 'security'
  | 'privacy'
  | 'theme';

const tabs: { id: SettingsTab; label: string }[] = [
  { id: 'ai', label: 'AI/LLM' },
  { id: 'providers', label: 'Providers' },
  { id: 'ide', label: 'IDE/Editor' },
  { id: 'cli', label: 'CLI' },
  { id: 'sync', label: 'Sync' },
  { id: 'memory', label: 'Memory' },
  { id: 'security', label: 'Security' },
  { id: 'privacy', label: 'Privacy' },
  { id: 'theme', label: 'Theme' },
];

export default function SettingsPage() {
  const [activeTab, setActiveTab] = useState<SettingsTab>('ai');

  const renderTabContent = () => {
    switch (activeTab) {
      case 'ai':
        return <AISettings />;
      case 'providers':
        return <ProvidersSettings />;
      case 'ide':
        return <IDESettings />;
      case 'cli':
        return <CLISettings />;
      case 'sync':
        return <SyncSettings />;
      case 'memory':
        return <MemorySettings />;
      case 'security':
        return <SecuritySettings />;
      case 'privacy':
        return <PrivacySettings />;
      case 'theme':
        return <ThemeSettings />;
      default:
        return null;
    }
  };

  return (
    <MainLayout>
      <div className="space-y-6">
        <div className="flex items-center gap-3">
          <Settings className="w-8 h-8 text-blue-400" />
          <h1 className="text-3xl font-bold text-slate-100">Settings</h1>
        </div>

        <div className="bg-slate-800/50 backdrop-blur-sm rounded-2xl border border-slate-700 overflow-hidden">
          <div className="flex border-b border-slate-700">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={cn(
                  'px-6 py-4 text-sm font-medium transition-colors',
                  'hover:bg-slate-700/50',
                  activeTab === tab.id
                    ? 'bg-slate-700/70 text-blue-400 border-b-2 border-blue-400'
                    : 'text-slate-300 hover:text-slate-100'
                )}
              >
                {tab.label}
              </button>
            ))}
          </div>

          <div className="p-6">
            {renderTabContent()}
          </div>
        </div>
      </div>
    </MainLayout>
  );
}

