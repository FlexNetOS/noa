'use client';

import { cn } from '@/lib/utils';

interface SkeletonProps {
  className?: string;
}

/**
 * Skeleton loader (T796) for pending UI states.
 */
export function Skeleton({ className }: SkeletonProps) {
  return (
    <div
      className={cn(
        'animate-pulse rounded-md bg-slate-800/70',
        className ?? 'h-4 w-full'
      )}
      role="presentation"
      aria-hidden="true"
    />
  );
}
