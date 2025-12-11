import en from './bundled/en.json';
import es from './bundled/es.json';
import zh from './bundled/zh.json';
import ar from './bundled/ar.json';
import he from './bundled/he.json';
import { detectLocale, normalizeLocale, type SupportedLocale } from './locale-detector';

type Dictionary = Record<string, string>;

const dictionaries: Record<SupportedLocale, Dictionary> = {
  en,
  es,
  zh,
  ar,
  he,
};

export function getTranslations(locale?: string): { locale: SupportedLocale; t: (key: string) => string } {
  const normalized = normalizeLocale(locale ?? (typeof window !== 'undefined' ? detectLocale() : 'en'));
  const dict = dictionaries[normalized] ?? en;

  const translate = (key: string) => dict[key] ?? en[key] ?? key;
  return { locale: normalized, t: translate };
}
