import { defineConfig } from 'vitest/config';
import path from 'path';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ],
  test: {
    globals: true,
    environment: process.env.E2E === '1' ? 'node' : 'jsdom',
    setupFiles: './test/setup.js',
    include: process.env.E2E === '1' ? ['test/e2e/**/*.test.js'] : ['test/**/*.test.js'],
    exclude: process.env.E2E === '1' ? [] : ['test/e2e/**'],
    watch: false,
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: [
        'node_modules/',
        'test/',
        '**/*.config.js',
        'dist/',
        'scripts/'
      ]
    }
  },
  resolve: {
    alias: {
      '@host': path.resolve(__dirname, './host/src'),
      '@remote': path.resolve(__dirname, './remote/src'),
      '@test': path.resolve(__dirname, './test')
    }
  }
});