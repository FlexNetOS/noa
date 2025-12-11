'use client';

import { useEffect } from 'react';
import { X } from 'lucide-react';
import { cn } from '@/lib/utils';

export type ToastVariant = 'info' | 'success' | 'warning' | 'error';

export interface ToastMessage {
  id: string;
  title: string;
  description?: string;
  variant?: ToastVariant;
  actionLabel?: string;
  onAction?: () => void;
  durationMs?: number;
}

interface ToastProps {
  toasts: ToastMessage[];
  onDismiss: (id: string) => void;
}

/**
 * Toast notification system with retry actions (T798).
 */
export function Toast({ toasts, onDismiss }: ToastProps) {
  useEffect(() => {
    const timers = toasts.map((toast) => {
      const duration = toast.durationMs ?? 5000;
      return setTimeout(() => onDismiss(toast.id), duration);
    });
    return () => timers.forEach(clearTimeout);
  }, [toasts, onDismiss]);

  return (
    <div
      className="fixed bottom-4 right-4 z-40 space-y-2"
      role="status"
      aria-live="polite"
      aria-label="Notifications"
    >
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={cn(
            'w-80 rounded-lg border p-3 shadow-lg bg-slate-900/95 backdrop-blur',
            variantClasses(toast.variant ?? 'info')
          )}
        >
          <div className="flex items-start gap-2">
            <div className="flex-1">
              <p className="text-sm font-semibold text-slate-100">{toast.title}</p>
              {toast.description && (
                <p className="text-xs text-slate-400 mt-1">{toast.description}</p>
              )}
              {toast.actionLabel && toast.onAction && (
                <button
                  onClick={toast.onAction}
                  className="mt-2 text-xs text-blue-300 underline"
                  aria-label={toast.actionLabel}
                >
                  {toast.actionLabel}
                </button>
              )}
            </div>
            <button
              onClick={() => onDismiss(toast.id)}
              className="text-slate-400 hover:text-slate-200"
              aria-label="Dismiss notification"
            >
              <X className="w-4 h-4" />
            </button>
          </div>
        </div>
      ))}
    </div>
  );
}

function variantClasses(variant: ToastVariant) {
  switch (variant) {
    case 'success':
      return 'border-emerald-600/60';
    case 'warning':
      return 'border-amber-500/60';
    case 'error':
      return 'border-red-600/60';
    default:
      return 'border-slate-700';
  }
}
