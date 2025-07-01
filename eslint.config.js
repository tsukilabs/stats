import { defineConfig } from '@tb-dev/eslint-config';

export default defineConfig({
  project: ['./web/tsconfig.json'],
  ignores: ['core/pkg/*'],
  features: {
    vue: true,
  },
});
