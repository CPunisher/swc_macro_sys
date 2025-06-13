import { defineConfig } from '@rsbuild/core';

export default defineConfig({
  output: {
    // Disable minification for both JS and CSS
    minify: false,
    // Set hardcoded filenames without hashes for reliable testing
    filename: {
      html: '[name].html',
      js: '[name].js',
      css: '[name].css',
      svg: '[name].svg',
      font: '[name][ext]',
      image: '[name][ext]',
      media: '[name][ext]',
      assets: '[name][ext]',
    },
  },
  performance: {
    // Generate stats.json file without the HTML report
    bundleAnalyze: {
      analyzerMode: 'disabled',
      generateStatsFile: true,
    },
    // Set chunk splitting strategy to all-in-one
    chunkSplit: {
      strategy: 'all-in-one',
    },
  },
  tools: {
    // Configure rspack to disable module concatenation
    rspack: {
      optimization: {
        concatenateModules: false,
      },
    },
  },
});
