import { describe, it, expect, beforeAll } from 'vitest';
import { optimizer } from './utils/optimizer.js';
import { 
  loadTestCase, 
  saveSnapshot
} from './utils/test-helpers.js';

describe('Dead Code Elimination Tests', () => {
  beforeAll(async () => {
    await optimizer.initialize();
  });

  describe('Simple Dead Code Elimination', () => {
    it('should remove unused function when only call is conditionally eliminated', async () => {
      const source = loadTestCase('simple-code', 'dead-code-elimination.js');
      const config = {
        featureFlags: {
          enableExperimentalFeature: false,
          enableHeavyMath: false,
          enableComplexUI: false,
          enableDetailedLogging: false,
          enableExperimentalAnalytics: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('dce-all-disabled', source, optimized, analysis);
      
      // Should have significant size reduction
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      // Functions that are only called conditionally should be removed
      expect(optimized).not.toContain('heavyMathCalculation');
      expect(optimized).not.toContain('renderComplexDashboard');
      expect(optimized).not.toContain('performDetailedLogging');
      expect(optimized).not.toContain('trackExperimentalMetrics');
      
      // validateUserPermissions should also be removed since it's only called in disabled UI block
      expect(optimized).not.toContain('validateUserPermissions');
      
      // Baseline functions should remain
      expect(optimized).toContain('baselineFunction');
      expect(optimized).toContain('runApplication');
      expect(optimized).toContain('testIsolatedCalls');
    });

    it('should keep functions when their calls are enabled', async () => {
      const source = loadTestCase('simple-code', 'dead-code-elimination.js');
      const config = {
        featureFlags: {
          enableHeavyMath: true,
          enableComplexUI: true,
          enableDetailedLogging: false,
          enableExperimentalAnalytics: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('dce-partial-enabled', source, optimized, analysis);
      
      // Functions with enabled calls should be present
      expect(optimized).toContain('heavyMathCalculation');
      expect(optimized).toContain('renderComplexDashboard');
      expect(optimized).toContain('validateUserPermissions');
      
      // Functions with disabled calls should be removed
      expect(optimized).not.toContain('performDetailedLogging');
      expect(optimized).not.toContain('trackExperimentalMetrics');
      
      // Always present functions
      expect(optimized).toContain('baselineFunction');
    });

    it('should handle multiple conditional calls to same function', async () => {
      const source = loadTestCase('simple-code', 'dead-code-elimination.js');
      const config = {
        featureFlags: {
          enableFeatureA: true,
          enableFeatureB: false,
          enableFeatureC: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('dce-multiple-calls-partial', source, optimized, analysis);
      
      // Function should be kept if at least one call is enabled
      expect(optimized).toContain('trackExperimentalMetrics');
      expect(optimized).toContain('testMultipleConditionalCalls');
    });

    it('should remove function when all calls are disabled', async () => {
      const source = loadTestCase('simple-code', 'dead-code-elimination.js');
      const config = {
        featureFlags: {
          enableFeatureA: false,
          enableFeatureB: false,
          enableFeatureC: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('dce-multiple-calls-disabled', source, optimized, analysis);
      
      // Function should be removed when all calls are disabled
      expect(optimized).not.toContain('trackExperimentalMetrics');
      expect(optimized).toContain('testMultipleConditionalCalls'); // Container function should remain
    });
  });

  describe('Complex Dead Code Elimination', () => {
    it('should eliminate entire dependency chains', async () => {
      const source = loadTestCase('complex-scenarios', 'advanced-dead-code.js');
      const config = {
        features: {
          enableHeavyComputation: false,
          enableAI: false,
          enableAdvancedGraphics: false,
          enableAnalytics: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('dce-complex-all-disabled', source, optimized, analysis);
      
      // Should have very significant size reduction
      expect(analysis.sizes.reduction).toBeGreaterThan(1000);
      
      // Heavy computation chain should be eliminated
      expect(optimized).not.toContain('performMatrixMultiplication');
      expect(optimized).not.toContain('optimizeAlgorithm');
      expect(optimized).not.toContain('generateHeavyReport');
      
      // ML/AI functions should be eliminated
      expect(optimized).not.toContain('trainNeuralNetwork');
      expect(optimized).not.toContain('runInference');
      
      // Graphics functions should be eliminated
      expect(optimized).not.toContain('initializeGraphicsEngine');
      expect(optimized).not.toContain('renderComplexScene');
      expect(optimized).not.toContain('applyPostProcessing');
      
      // Analytics functions should be eliminated
      expect(optimized).not.toContain('collectTelemetryData');
      expect(optimized).not.toContain('processAnalytics');
      
      // Main application functions should remain but be simplified
      expect(optimized).toContain('runDataProcessingPipeline');
      expect(optimized).toContain('runMLWorkflow');
      expect(optimized).toContain('runGraphicsApplication');
      expect(optimized).toContain('runAnalyticsDashboard');
      expect(optimized).toContain('baselineFunction');
    });

    it('should keep dependency chains when features are enabled', async () => {
      const source = loadTestCase('complex-scenarios', 'advanced-dead-code.js');
      const config = {
        features: {
          enableHeavyComputation: true,
          enableAI: true,
          enableAdvancedGraphics: false,
          enableAnalytics: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('dce-complex-partial-enabled', source, optimized, analysis);
      
      // Heavy computation chain should be present
      expect(optimized).toContain('performMatrixMultiplication');
      expect(optimized).toContain('optimizeAlgorithm');
      expect(optimized).toContain('generateHeavyReport');
      
      // ML/AI functions should be present
      expect(optimized).toContain('trainNeuralNetwork');
      expect(optimized).toContain('runInference');
      
      // Graphics functions should be eliminated (disabled)
      expect(optimized).not.toContain('initializeGraphicsEngine');
      expect(optimized).not.toContain('renderComplexScene');
      expect(optimized).not.toContain('applyPostProcessing');
      
      // Analytics functions should be eliminated (disabled)
      expect(optimized).not.toContain('collectTelemetryData');
      expect(optimized).not.toContain('processAnalytics');
    });

    it('should handle isolated function elimination', async () => {
      const source = loadTestCase('complex-scenarios', 'advanced-dead-code.js');
      const config = {
        features: {
          enableExperimentalMath: false,
          enableFullPipeline: false,
          enablePathA: false,
          enablePathB: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('dce-complex-isolated-disabled', source, optimized, analysis);
      
      // Isolated test functions should remain but be simplified
      expect(optimized).toContain('testIsolatedHeavyComputation');
      expect(optimized).toContain('testDependencyChain');
      expect(optimized).toContain('testMultipleConditionalUsage');
      
      // Functions called only in disabled conditions should be removed
      expect(optimized).not.toContain('performMatrixMultiplication');
      expect(optimized).not.toContain('optimizeAlgorithm');
      expect(optimized).not.toContain('generateHeavyReport');
    });

    it('should preserve functions used in multiple paths when some are enabled', async () => {
      const source = loadTestCase('complex-scenarios', 'advanced-dead-code.js');
      const config = {
        features: {
          enablePathA: true,
          enablePathB: false,
          enableHeavyComputation: false,
          enableFullPipeline: false
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('dce-complex-multiple-paths', source, optimized, analysis);
      
      // optimizeAlgorithm should be kept because it's used in enabled pathA
      expect(optimized).toContain('optimizeAlgorithm');
      expect(optimized).toContain('testMultipleConditionalUsage');
      
      // Functions not used in any enabled path should be removed
      expect(optimized).not.toContain('generateHeavyReport');
      expect(optimized).not.toContain('performMatrixMultiplication');
    });
  });

  describe('DCE Performance Analysis', () => {
    it('should measure DCE effectiveness across configurations', async () => {
      const source = loadTestCase('complex-scenarios', 'advanced-dead-code.js');
      const configs = [
        { 
          name: 'allDisabled', 
          config: { 
            features: { 
              enableHeavyComputation: false,
              enableAI: false,
              enableAdvancedGraphics: false,
              enableAnalytics: false
            } 
          } 
        },
        { 
          name: 'allEnabled', 
          config: { 
            features: { 
              enableHeavyComputation: true,
              enableAI: true,
              enableAdvancedGraphics: true,
              enableAnalytics: true
            } 
          } 
        },
        { 
          name: 'partialEnabled', 
          config: { 
            features: { 
              enableHeavyComputation: true,
              enableAI: false,
              enableAdvancedGraphics: false,
              enableAnalytics: true
            } 
          } 
        }
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
        expect(analysis.executionTime).toBeLessThan(2000);
      });
      
      // All disabled should have the most size reduction
      const allDisabled = results.find(r => r.name === 'allDisabled');
      const allEnabled = results.find(r => r.name === 'allEnabled');
      const partialEnabled = results.find(r => r.name === 'partialEnabled');
      
      expect(allDisabled.analysis.sizes.reduction).toBeGreaterThan(allEnabled.analysis.sizes.reduction);
      expect(allDisabled.analysis.sizes.reduction).toBeGreaterThan(partialEnabled.analysis.sizes.reduction);
      expect(partialEnabled.analysis.sizes.reduction).toBeGreaterThan(allEnabled.analysis.sizes.reduction);
      
      // Verify meaningful DCE
      expect(allDisabled.analysis.sizes.reductionPercent).toBeGreaterThan(30); // Should remove at least 30% when all disabled
    });
  });
}); 