'use client';

import { cn } from '@/lib/utils';
import { Activity, AlertTriangle, CheckCircle2, Info, RefreshCw } from 'lucide-react';

export type StatusLevel = 'info' | 'success' | 'warning' | 'error';

export interface StatusItem {
  id: string;
  label: string;
  detail?: string;
  level: StatusLevel;
  loading?: boolean;
}

interface StatusBarProps {
  items: StatusItem[];
}

/**
 * Persistent status bar for background operations (T797).
 */
export default function StatusBar({ items }: StatusBarProps) {
  return (
    <div
      className="sticky bottom-0 z-20 flex flex-wrap gap-2 p-3 bg-slate-900/80 border-t border-slate-800 backdrop-blur"
      role="status"
      aria-label="Background status"
    >
      {items.length === 0 ? (
        <span className="text-xs text-slate-400">No background operations</span>
      ) : (
        items.map((item) => (
          <span
            key={item.id}
            className={cn(
              'inline-flex items-center gap-2 px-3 py-2 rounded-lg text-xs border',
              pillClasses(item.level)
            )}
          >
            {item.loading ? <RefreshCw className="w-3 h-3 animate-spin" /> : iconFor(item.level)}
            <span className="font-medium">{item.label}</span>
            {item.detail && <span className="text-slate-400">{item.detail}</span>}
          </span>
        ))
      )}
    </div>
  );
}

function iconFor(level: StatusLevel) {
  switch (level) {
    case 'success':
      return <CheckCircle2 className="w-4 h-4" />;
    case 'warning':
      return <AlertTriangle className="w-4 h-4" />;
    case 'error':
      return <Activity className="w-4 h-4" />;
    default:
      return <Info className="w-4 h-4" />;
  }
}

function pillClasses(level: StatusLevel) {
  switch (level) {
    case 'success':
      return 'bg-emerald-500/10 border-emerald-500/50 text-emerald-200';
    case 'warning':
      return 'bg-amber-500/10 border-amber-500/50 text-amber-200';
    case 'error':
      return 'bg-red-500/10 border-red-500/50 text-red-200';
    default:
      return 'bg-slate-800 border-slate-700 text-slate-200';
  }
}
