'use client';

import { ReactNode } from 'react';
import { cn } from '@/lib/utils';

interface EmptyStateProps {
  title: string;
  description?: string;
  action?: ReactNode;
  className?: string;
}

/**
 * Meaningful empty state with suggested action (T800).
 */
export function EmptyState({ title, description, action, className }: EmptyStateProps) {
  return (
    <div
      className={cn(
        'text-center rounded-2xl border border-dashed border-slate-700 p-8 bg-slate-900/40',
        className
      )}
      role="status"
      aria-live="polite"
    >
      <p className="text-lg font-semibold text-slate-100">{title}</p>
      {description && <p className="text-sm text-slate-400 mt-2">{description}</p>}
      {action && <div className="mt-4 flex justify-center">{action}</div>}
    </div>
  );
}
