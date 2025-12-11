'use client';

import { ReactNode, useEffect } from 'react';
import { useAccessibility } from '@/hooks/useAccessibility';

export default function AccessibilityProvider({ children }: { children: ReactNode }) {
  const { highContrast, reducedMotion } = useAccessibility();

  useEffect(() => {
    const body = document.body;
    body.dataset.highContrast = highContrast ? 'true' : 'false';
    body.dataset.reducedMotion = reducedMotion ? 'true' : 'false';
  }, [highContrast, reducedMotion]);

  return <>{children}</>;
}
