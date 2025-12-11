import en from './bundled/en.json';
import es from './bundled/es.json';
import zh from './bundled/zh.json';
import ar from './bundled/ar.json';
import he from './bundled/he.json';
import { detectLocale, normalizeLocale, type SupportedLocale } from './locale-detector';

type Dictionary = Record<string, string>;

const dictionaries: Record<SupportedLocale, Dictionary> = {
  en: en as Dictionary,
  es: es as Dictionary,
  zh: zh as Dictionary,
  ar: ar as Dictionary,
  he: he as Dictionary,
};

export function getTranslations(locale?: string): { locale: SupportedLocale; t: (key: string) => string } {
  const normalized = normalizeLocale(locale ?? (typeof window !== 'undefined' ? detectLocale() : 'en'));
  const dict: Dictionary = dictionaries[normalized] ?? (en as Dictionary);

  const translate = (key: string) =>
    (dict as Dictionary)[key] ?? (en as Dictionary)[key] ?? key;
  return { locale: normalized, t: translate };
}
