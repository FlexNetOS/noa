'use client';

import { useEffect, useState } from 'react';
import { useNetworkStatus } from './useNetworkStatus';

export interface OfflineState<T> {
  data: T | null;
  stale: boolean;
  syncing: boolean;
  error?: string;
}

/**
 * Offline-aware data hook with cached/partial data display (T799).
 * Accepts async loader + optional cached seed.
 */
export function useOfflineData<T>(
  load: () => Promise<T>,
  cached?: T | null
): OfflineState<T> {
  const { online } = useNetworkStatus();
  const [state, setState] = useState<OfflineState<T>>({
    data: cached ?? null,
    stale: Boolean(cached),
    syncing: false,
  });

  useEffect(() => {
    let cancelled = false;
    const sync = async () => {
      setState((prev) => ({ ...prev, syncing: true }));
      try {
        const fresh = await load();
        if (!cancelled) {
          setState({ data: fresh, stale: false, syncing: false });
        }
      } catch (err) {
        if (!cancelled) {
          setState((prev) => ({
            ...prev,
            syncing: false,
            stale: true,
            error: (err as Error).message,
          }));
        }
      }
    };

    if (online) {
      void sync();
    } else {
      setState((prev) => ({ ...prev, stale: true }));
    }

    return () => {
      cancelled = true;
    };
  }, [load, online]);

  return state;
}
