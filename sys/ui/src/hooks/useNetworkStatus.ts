'use client';

import { useEffect, useState } from 'react';

export type NetworkStatus = {
  online: boolean;
  lastChange: number;
};

/**
 * Network status detection with offline indicator (T801).
 */
export function useNetworkStatus(): NetworkStatus {
  const [status, setStatus] = useState<NetworkStatus>({
    online: typeof navigator !== 'undefined' ? navigator.onLine : true,
    lastChange: Date.now(),
  });

  useEffect(() => {
    const handleOnline = () =>
      setStatus({ online: true, lastChange: Date.now() });
    const handleOffline = () =>
      setStatus({ online: false, lastChange: Date.now() });

    window.addEventListener('online', handleOnline);
    window.addEventListener('offline', handleOffline);

    return () => {
      window.removeEventListener('online', handleOnline);
      window.removeEventListener('offline', handleOffline);
    };
  }, []);

  return status;
}
