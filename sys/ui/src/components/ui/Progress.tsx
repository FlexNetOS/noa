'use client';

import { cn } from '@/lib/utils';

interface ProgressProps {
  value: number; // 0-100
  label?: string;
}

/**
 * Progress indicator for long-running operations (>2s) (T803).
 */
export function Progress({ value, label }: ProgressProps) {
  const clamped = Math.max(0, Math.min(100, value));
  return (
    <div className="space-y-1" role="group" aria-label={label ?? 'Progress'}>
      {label && <div className="text-xs text-slate-400">{label}</div>}
      <div
        className="w-full h-3 rounded-full bg-slate-800 border border-slate-700"
        role="progressbar"
        aria-valuemin={0}
        aria-valuemax={100}
        aria-valuenow={clamped}
      >
        <div
          className={cn(
            'h-full rounded-full bg-blue-600 transition-all',
            clamped === 100 && 'bg-emerald-500'
          )}
          style={{ width: `${clamped}%` }}
        />
      </div>
      <div className="text-[11px] text-slate-500">{clamped}%</div>
    </div>
  );
}
