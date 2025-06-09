import { describe, it, expect, beforeAll } from 'vitest';
import { optimizer } from './utils/optimizer.js';
import { 
  loadTestCase, 
  TEST_CONFIGS, 
  saveSnapshot, 
  validateOptimization,
  EXPECTED_WEBPACK_MODULES 
} from './utils/test-helpers.js';

describe('Webpack Bundle Optimization', () => {
  beforeAll(async () => {
    await optimizer.initialize();
  });

  describe('Feature Flag Tree Shaking', () => {
    it('should keep all modules when all features are enabled', async () => {
      const source = loadTestCase('webpack-bundles', 'bundle-all-features.js');
      const config = TEST_CONFIGS.webpack.allFeatures;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('webpack-all-features', source, optimized, analysis);
      
      // Validate results
      const validation = validateOptimization('allFeatures', analysis, true);
      
      expect(validation.passed).toBe(true);
      expect(analysis.modules.removed).toBe(0);
      expect(analysis.sizes.reductionPercent).toBe(0);
      
      // All 7 modules should be present
      expect(analysis.modules.optimized).toBe(7);
    });

    it('should remove Feature B and Debug modules when only Feature A is enabled', async () => {
      const source = loadTestCase('webpack-bundles', 'bundle-all-features.js');
      const config = TEST_CONFIGS.webpack.featureAOnly;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('webpack-feature-a-only', source, optimized, analysis);
      
      // Validate results
      const validation = validateOptimization('featureAOnly', analysis, true);
      
      expect(validation.passed).toBe(true);
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      expect(analysis.modules.removed).toBeGreaterThan(0);
      
      // Should have only Feature A modules (3 modules: 153, 418, 78)
      expect(analysis.modules.optimized).toBe(3);
      
      // Verify specific modules are present in optimized code
      expect(optimized).toContain('153:'); // featureA
      expect(optimized).toContain('418:'); // dataProcessor
      expect(optimized).toContain('78:');  // heavyMathUtils
      
      // Verify removed modules are not present
      expect(optimized).not.toContain('722:'); // featureB
      expect(optimized).not.toContain('803:'); // expensiveUIUtils
      expect(optimized).not.toContain('812:'); // networkUtils
      expect(optimized).not.toContain('422:'); // debugUtils
    });

    it('should remove Feature A and Debug modules when only Feature B is enabled', async () => {
      const source = loadTestCase('webpack-bundles', 'bundle-all-features.js');
      const config = TEST_CONFIGS.webpack.featureBOnly;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('webpack-feature-b-only', source, optimized, analysis);
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      expect(analysis.modules.optimized).toBe(3); // Feature B modules
      
      // Verify Feature B modules are present
      expect(optimized).toContain('722:'); // featureB
      expect(optimized).toContain('803:'); // expensiveUIUtils
      expect(optimized).toContain('812:'); // networkUtils
      
      // Verify removed modules are not present
      expect(optimized).not.toContain('153:'); // featureA
      expect(optimized).not.toContain('418:'); // dataProcessor
      expect(optimized).not.toContain('78:');  // heavyMathUtils
      expect(optimized).not.toContain('422:'); // debugUtils
    });

    it('should remove all feature modules when no features are enabled', async () => {
      const source = loadTestCase('webpack-bundles', 'bundle-all-features.js');
      const config = TEST_CONFIGS.webpack.minimal;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('webpack-minimal', source, optimized, analysis);
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      expect(analysis.modules.removed).toBe(7); // All feature modules removed
      
      // Should have minimal webpack boilerplate only
      expect(analysis.modules.optimized).toBe(0);
      
      // Verify all feature modules are removed
      expect(optimized).not.toContain('153:'); // featureA
      expect(optimized).not.toContain('418:'); // dataProcessor
      expect(optimized).not.toContain('78:');  // heavyMathUtils
      expect(optimized).not.toContain('722:'); // featureB
      expect(optimized).not.toContain('803:'); // expensiveUIUtils
      expect(optimized).not.toContain('812:'); // networkUtils
      expect(optimized).not.toContain('422:'); // debugUtils
    });

    it('should keep only debug modules when only debug mode is enabled', async () => {
      const source = loadTestCase('webpack-bundles', 'bundle-all-features.js');
      const config = TEST_CONFIGS.webpack.debugOnly;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('webpack-debug-only', source, optimized, analysis);
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      expect(analysis.modules.optimized).toBe(1); // Only debug module
      
      // Verify debug module is present
      expect(optimized).toContain('422:'); // debugUtils
      
      // Verify all other modules are removed
      expect(optimized).not.toContain('153:'); // featureA
      expect(optimized).not.toContain('418:'); // dataProcessor
      expect(optimized).not.toContain('78:');  // heavyMathUtils
      expect(optimized).not.toContain('722:'); // featureB
      expect(optimized).not.toContain('803:'); // expensiveUIUtils
      expect(optimized).not.toContain('812:'); // networkUtils
    });
  });

  describe('Pre-optimized Bundles', () => {
    it('should handle already optimized Feature A bundle', async () => {
      const source = loadTestCase('webpack-bundles', 'bundle-feature-a-only.js');
      const config = TEST_CONFIGS.webpack.featureAOnly;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('webpack-pre-optimized-feature-a', source, optimized, analysis);
      
      // Should not reduce size much since already optimized
      expect(analysis.modules.optimized).toBe(3);
      expect(optimized).toContain('153:'); // featureA
      expect(optimized).toContain('418:'); // dataProcessor
      expect(optimized).toContain('78:');  // heavyMathUtils
    });
  });

  describe('Performance Metrics', () => {
    it('should measure optimization performance across different configurations', async () => {
      const source = loadTestCase('webpack-bundles', 'bundle-all-features.js');
      const configurations = [
        { name: 'allFeatures', config: TEST_CONFIGS.webpack.allFeatures },
        { name: 'featureAOnly', config: TEST_CONFIGS.webpack.featureAOnly },
        { name: 'minimal', config: TEST_CONFIGS.webpack.minimal }
      ];
      
      const results = [];
      
      for (const { name, config } of configurations) {
        const startTime = performance.now();
        const optimized = await optimizer.optimizeCode(source, config);
        const endTime = performance.now();
        
        const analysis = optimizer.analyzeOptimization(source, optimized, config);
        analysis.executionTime = endTime - startTime;
        
        results.push({ name, analysis });
      }
      
      // All optimizations should complete in reasonable time
      results.forEach(({ name, analysis }) => {
        expect(analysis.executionTime).toBeLessThan(1000); // Less than 1 second
      });
      
      // Minimal config should have the highest size reduction
      const minimal = results.find(r => r.name === 'minimal');
      const allFeatures = results.find(r => r.name === 'allFeatures');
      
      expect(minimal.analysis.sizes.reduction).toBeGreaterThan(allFeatures.analysis.sizes.reduction);
    });
  });
}); 