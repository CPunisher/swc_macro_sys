import { describe, it, expect, beforeAll } from 'vitest';
import { optimizer } from './utils/optimizer.js';
import { 
  loadTestCase, 
  saveSnapshot
} from './utils/test-helpers.js';

describe('Isolated Function Call Tests', () => {
  beforeAll(async () => {
    await optimizer.initialize();
  });

  it('should remove function when its only call is conditionally eliminated', async () => {
    const source = loadTestCase('simple-code', 'isolated-function-call.js');
    const config = {
      featureFlags: {
        enableExperimentalFeature: false,
        enableAnotherFeature: false
      }
    };
    
    const optimized = await optimizer.optimizeCode(source, config);
    const analysis = optimizer.analyzeOptimization(source, optimized, config);
    
    // Save snapshot
    saveSnapshot('isolated-call-disabled', source, optimized, analysis);
    
    console.log('Original size:', analysis.sizes.original);
    console.log('Optimized size:', analysis.sizes.optimized);
    console.log('Reduction:', analysis.sizes.reduction, 'bytes');
    console.log('Reduction %:', analysis.sizes.reductionPercent + '%');
    
    // Should have size reduction
    expect(analysis.sizes.reduction).toBeGreaterThan(0);
    
    // Functions should be removed since their calls are disabled
    expect(optimized).not.toContain('experimentalFeature');
    expect(optimized).not.toContain('anotherFeature');
    
    // Main functions should remain
    expect(optimized).toContain('main');
    expect(optimized).toContain('baselineFunction');
  });

  it('should keep function when its call is enabled', async () => {
    const source = loadTestCase('simple-code', 'isolated-function-call.js');
    const config = {
      featureFlags: {
        enableExperimentalFeature: true,
        enableAnotherFeature: false
      }
    };
    
    const optimized = await optimizer.optimizeCode(source, config);
    const analysis = optimizer.analyzeOptimization(source, optimized, config);
    
    // Save snapshot
    saveSnapshot('isolated-call-partial', source, optimized, analysis);
    
    // experimentalFeature should be kept (call enabled)
    expect(optimized).toContain('experimentalFeature');
    
    // anotherFeature should be removed (call disabled)
    expect(optimized).not.toContain('anotherFeature');
    
    // Main functions should remain
    expect(optimized).toContain('main');
    expect(optimized).toContain('baselineFunction');
  });
}); 