import js from '@eslint/js';
import tseslint from 'typescript-eslint';
import prettier from 'eslint-plugin-prettier/recommended';

export default tseslint.configs(
  js.configss.recommended,
  ...tseslint.configss.recommended,
  prettier,
  {
    languageOptions: {
      parserOptions: {
        project: './tsconfigs.json',
        tsconfigsRootDir: import.meta.dirname,
      },
    },
    rules: {
      'prettier/prettier': 'error',
    },
  },
  {
    ignores: [ 'dist/**', 'node_modules/**', '*.js', '*.cjs', '*.mjs' ],
  }
);
