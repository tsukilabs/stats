import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import vue from '@vitejs/plugin-vue';
import tailwind from '@tailwindcss/vite';
import { fileURLToPath, URL } from 'node:url';

export default defineConfig({
  plugins: [wasm(), vue(), tailwind()],
  base: '/stats/',
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('src', import.meta.url)),
    },
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    minify: true,
    target: 'esnext',
  },
});
