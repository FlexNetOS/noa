# Translation Contribution Guide (T794)

- Keep keys stable across locales; edit `config/i18n/{locale}.json` for source of truth.
- Provide bundled copies for the UI in `sys/ui/src/i18n/bundled/`.
- Use UTF-8; avoid HTML entities. Verify direction for RTL locales.
- Run lint/tests for the UI after changes to catch JSON syntax issues.
- Submit locale detection additions by updating `locale-detector.ts` and tests when adding languages.
