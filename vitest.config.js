import { defineConfig } from 'vitest/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait()
  ],
  test: {
    name: 'SWC Macro System Tests',
    environment: 'node',
    globals: true,
    include: ['tests/**/*.{test,spec}.{js,mjs,ts}'],
    exclude: ['node_modules', 'dist', 'build', 'target'],
    testTimeout: 10000,
    hookTimeout: 10000,
    teardownTimeout: 10000,
    isolate: true,
    pool: 'forks',
    poolOptions: {
      forks: {
        singleFork: true
      }
    },
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['tests/**/*.{js,ts}'],
      exclude: ['tests/**/*.{test,spec}.{js,ts}']
    },
    outputFile: {
      html: './test-results/vitest-report.html',
      json: './test-results/vitest-results.json'
    },
    silent: false,
    verbose: true,
    watch: false
  },
  esbuild: {
    target: 'node18'
  },
  optimizeDeps: false
}); 