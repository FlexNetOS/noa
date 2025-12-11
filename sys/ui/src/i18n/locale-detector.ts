const SUPPORTED = ['en', 'es', 'zh', 'ar', 'he'] as const;
export type SupportedLocale = (typeof SUPPORTED)[number];

export function detectLocale(): SupportedLocale {
  if (typeof navigator === 'undefined') return 'en';
  const langs = navigator.languages ?? [navigator.language];
  for (const lang of langs) {
    const base = lang.toLowerCase().split('-')[0];
    if (SUPPORTED.includes(base as SupportedLocale)) {
      return base as SupportedLocale;
    }
  }
  return 'en';
}

export function normalizeLocale(locale?: string): SupportedLocale {
  if (!locale) return 'en';
  const base = locale.toLowerCase().split('-')[0];
  return SUPPORTED.includes(base as SupportedLocale) ? (base as SupportedLocale) : 'en';
}
