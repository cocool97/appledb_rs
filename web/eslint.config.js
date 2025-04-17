import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  { ignores: ['dist'] },
  /* eslint.configs.all, */
  /* tseslint.configs.strict */
  tseslint.configs.stylistic,
  tseslint.configs.eslintRecommended
);