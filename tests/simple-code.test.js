import { describe, it, expect, beforeAll } from 'vitest';
import { optimizer } from './utils/optimizer.js';
import { 
  loadTestCase, 
  TEST_CONFIGS, 
  saveSnapshot
} from './utils/test-helpers.js';

describe('Simple Code Optimization', () => {
  beforeAll(async () => {
    await optimizer.initialize();
  });

  describe('Conditional Compilation', () => {
    it('should keep all features when all feature flags are enabled', async () => {
      const source = loadTestCase('simple-code', 'conditional-compilation.js');
      const config = TEST_CONFIGS.simple.allFeatures;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('simple-all-features', source, optimized, analysis);
      
      // Should contain all conditional blocks
      expect(optimized).toContain('useExpensiveFeature');
      expect(optimized).toContain('useDebugFeature');
      expect(optimized).toContain('useExperimentalFeature');
      expect(optimized).toContain('getWelcomeMessage');
      
      // Inline defines should be replaced with config values
      expect(optimized).toContain('"development"'); // build.mode
      expect(optimized).toContain('"http://localhost:3000"'); // api.url
    });

    it('should remove all conditional features in production build', async () => {
      const source = loadTestCase('simple-code', 'conditional-compilation.js');
      const config = TEST_CONFIGS.simple.production;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('simple-production', source, optimized, analysis);
      
      // Should have reduced size
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      // Should not contain conditional features
      expect(optimized).not.toContain('useExpensiveFeature');
      expect(optimized).not.toContain('useDebugFeature');
      expect(optimized).not.toContain('useExperimentalFeature');
      expect(optimized).not.toContain('getWelcomeMessage');
      
      // Should keep base functionality
      expect(optimized).toContain('baseFeature');
      
      // Inline defines should be replaced with production values
      expect(optimized).toContain('"production"');
      expect(optimized).toContain('"https://api.production.com"');
    });

    it('should keep only debug features when only debug mode is enabled', async () => {
      const source = loadTestCase('simple-code', 'conditional-compilation.js');
      const config = TEST_CONFIGS.simple.debugOnly;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('simple-debug-only', source, optimized, analysis);
      
      // Should have some size reduction
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      // Should contain only debug features
      expect(optimized).toContain('useDebugFeature');
      expect(optimized).toContain('baseFeature');
      
      // Should not contain other features
      expect(optimized).not.toContain('useExpensiveFeature');
      expect(optimized).not.toContain('useExperimentalFeature');
    });
  });

  describe('Feature Flag Variations', () => {
    it('should handle feature flags test case', async () => {
      const source = loadTestCase('simple-code', 'feature-flags.js');
      const config = TEST_CONFIGS.simple.allFeatures;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('feature-flags-all', source, optimized, analysis);
      
      // Should contain all feature flag functions
      expect(optimized).toContain('useExpensiveFeature');
      expect(optimized).toContain('useDebugFeature');
      expect(optimized).toContain('useExperimentalFeature');
      expect(optimized).toContain('baseFeature');
    });

    it('should remove all features when none are enabled', async () => {
      const source = loadTestCase('simple-code', 'feature-flags.js');
      const config = {
        featureFlags: {
          enableExpensiveFeature: false,
          enableDebugMode: false,
          enableExperimentalFeature: false
        },
        build: {
          mode: 'production'
        },
        api: {
          url: 'https://api.production.com'
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('feature-flags-none', source, optimized, analysis);
      
      // Should have significant size reduction
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      // Should not contain any conditional features
      expect(optimized).not.toContain('useExpensiveFeature');
      expect(optimized).not.toContain('useDebugFeature');
      expect(optimized).not.toContain('useExperimentalFeature');
      
      // Should keep base functionality
      expect(optimized).toContain('baseFeature');
      
      // Should have production inline defines
      expect(optimized).toContain('"production"');
      expect(optimized).toContain('"https://api.production.com"');
    });
  });

  describe('Inline Define Replacement', () => {
    it('should replace inline defines with configured values', async () => {
      const source = loadTestCase('simple-code', 'conditional-compilation.js');
      const customConfig = {
        featureFlags: {
          enableExpensiveFeature: false,
          enableDebugMode: false,
          enableExperimentalFeature: false
        },
        build: {
          target: 'production',
          mode: 'staging'
        },
        api: {
          url: 'https://staging.api.com'
        },
        user: {
          isLoggedIn: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, customConfig);
      const analysis = optimizer.analyzeOptimization(source, optimized, customConfig);
      
      // Save snapshot
      saveSnapshot('simple-inline-defines', source, optimized, analysis);
      
      // Should replace inline defines with custom values
      expect(optimized).toContain('"staging"'); // build.mode
      expect(optimized).toContain('"https://staging.api.com"'); // api.url
    });
  });

  describe('Performance and Size Analysis', () => {
    it('should measure optimization effectiveness across configurations', async () => {
      const source = loadTestCase('simple-code', 'conditional-compilation.js');
      const configs = [
        { name: 'allFeatures', config: TEST_CONFIGS.simple.allFeatures },
        { name: 'production', config: TEST_CONFIGS.simple.production },
        { name: 'debugOnly', config: TEST_CONFIGS.simple.debugOnly }
      ];
      
      const results = [];
      
      for (const { name, config } of configs) {
        const startTime = performance.now();
        const optimized = await optimizer.optimizeCode(source, config);
        const endTime = performance.now();
        
        const analysis = optimizer.analyzeOptimization(source, optimized, config);
        analysis.executionTime = endTime - startTime;
        
        results.push({ name, analysis });
      }
      
      // All optimizations should complete quickly
      results.forEach(({ name, analysis }) => {
        expect(analysis.executionTime).toBeLessThan(1000); // Less than 1 second
      });
      
      // Production build should have the most size reduction
      const production = results.find(r => r.name === 'production');
      const allFeatures = results.find(r => r.name === 'allFeatures');
      
      expect(production.analysis.sizes.reduction).toBeGreaterThan(allFeatures.analysis.sizes.reduction);
      
      // Verify size reductions are meaningful
      expect(production.analysis.sizes.reductionPercent).toBeGreaterThan(0);
    });
  });
}); 