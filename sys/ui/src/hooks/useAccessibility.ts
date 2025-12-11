'use client';

import { useEffect, useState } from 'react';

/**
 * Accessibility hook for detecting high contrast preference and enabling focus rings.
 * - High contrast: prefers-contrast: more OR forced-colors: active
 * - Reduced motion: prefers-reduced-motion
 */
export function useAccessibility() {
  const [highContrast, setHighContrast] = useState(false);
  const [reducedMotion, setReducedMotion] = useState(false);

  useEffect(() => {
    const contrastQuery = window.matchMedia('(prefers-contrast: more)');
    const forcedColors = window.matchMedia('(forced-colors: active)');
    const motionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');

    const update = () => {
      setHighContrast(contrastQuery.matches || forcedColors.matches);
      setReducedMotion(motionQuery.matches);
    };

    update();

    contrastQuery.addEventListener('change', update);
    forcedColors.addEventListener('change', update);
    motionQuery.addEventListener('change', update);

    return () => {
      contrastQuery.removeEventListener('change', update);
      forcedColors.removeEventListener('change', update);
      motionQuery.removeEventListener('change', update);
    };
  }, []);

  return { highContrast, reducedMotion };
}
