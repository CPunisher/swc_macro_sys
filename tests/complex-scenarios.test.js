import { describe, it, expect, beforeAll } from 'vitest';
import { optimizer } from './utils/optimizer.js';
import { 
  loadTestCase, 
  TEST_CONFIGS, 
  saveSnapshot
} from './utils/test-helpers.js';

describe('Complex Scenarios Optimization', () => {
  beforeAll(async () => {
    await optimizer.initialize();
  });

  describe('Nested Conditional Compilation', () => {
    it('should handle mobile production configuration with premium features', async () => {
      const source = loadTestCase('complex-scenarios', 'nested-conditions.js');
      const config = TEST_CONFIGS.complex.mobileProduction;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('complex-mobile-production', source, optimized, analysis);
      
      // Should have size reduction due to removing desktop features
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      // Should contain mobile features
      expect(optimized).toContain('mobileOptimizedFeature');
      expect(optimized).toContain('premiumMobileFeature');
      
      // Should not contain desktop features
      expect(optimized).not.toContain('desktopFeature');
      expect(optimized).not.toContain('adminDesktopFeature');
      
      // Should not contain advanced features (user can't access)
      expect(optimized).not.toContain('productionAdvancedFeature');
      
      // Should keep base functionality
      expect(optimized).toContain('baseFunction');
    });

    it('should handle desktop admin configuration with full features', async () => {
      const source = loadTestCase('complex-scenarios', 'nested-conditions.js');
      const config = TEST_CONFIGS.complex.desktopAdmin;
      
      const optimized = await optimizer.optimizeCode(source, config);
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Save snapshot
      saveSnapshot('complex-desktop-admin', source, optimized, analysis);
      
      // Should contain desktop features
      expect(optimized).toContain('desktopFeature');
      expect(optimized).toContain('adminDesktopFeature');
      
      // Should contain advanced features (admin with permissions)
      expect(optimized).toContain('productionAdvancedFeature');
      
      // Should not contain mobile features
      expect(optimized).not.toContain('mobileOptimizedFeature');
      expect(optimized).not.toContain('premiumMobileFeature');
      
      // Should keep base functionality
      expect(optimized).toContain('baseFunction');
    });

    it('should handle minimal configuration removing all conditional features', async () => {
      const source = loadTestCase('complex-scenarios', 'nested-conditions.js');
      const minimalConfig = {
        platform: {
          isMobile: false,
          isDesktop: false
        },
        featureFlags: {
          enableMobileOptimizations: false,
          enableDesktopFeatures: false,
          enableAdvancedFeatures: false
        },
        user: {
          isPremium: false,
          isAdmin: false,
          permissions: {
            canAccessAdvanced: false
          }
        },
        environment: {
          isProduction: false
        },
        build: {
          target: 'development'
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, minimalConfig);
      const analysis = optimizer.analyzeOptimization(source, optimized, minimalConfig);
      
      // Save snapshot
      saveSnapshot('complex-minimal', source, optimized, analysis);
      
      // Should have significant size reduction
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      // Should not contain any conditional features
      expect(optimized).not.toContain('mobileOptimizedFeature');
      expect(optimized).not.toContain('premiumMobileFeature');
      expect(optimized).not.toContain('desktopFeature');
      expect(optimized).not.toContain('adminDesktopFeature');
      expect(optimized).not.toContain('productionAdvancedFeature');
      
      // Should keep only base functionality
      expect(optimized).toContain('baseFunction');
    });
  });

  describe('Complex Dependency Analysis', () => {
    it('should analyze conditional block removal effectiveness', async () => {
      const source = loadTestCase('complex-scenarios', 'nested-conditions.js');
      const configs = [
        { name: 'mobileProduction', config: TEST_CONFIGS.complex.mobileProduction },
        { name: 'desktopAdmin', config: TEST_CONFIGS.complex.desktopAdmin }
      ];
      
      const results = [];
      
      for (const { name, config } of configs) {
        const optimized = await optimizer.optimizeCode(source, config);
        const analysis = optimizer.analyzeOptimization(source, optimized, config);
        results.push({ name, analysis });
      }
      
      // Both should have conditional blocks removed
      results.forEach(({ name, analysis }) => {
        expect(analysis.conditionalBlocks.removed).toBeGreaterThan(0);
        expect(analysis.sizes.reduction).toBeGreaterThan(0);
      });
      
      // Desktop admin should have more features than mobile
      const mobile = results.find(r => r.name === 'mobileProduction');
      const desktop = results.find(r => r.name === 'desktopAdmin');
      
      // Desktop config should result in larger optimized size (more features)
      expect(desktop.analysis.sizes.optimized).toBeGreaterThan(mobile.analysis.sizes.optimized);
    });
  });

  describe('Inline Define Handling', () => {
    it('should properly replace nested inline defines', async () => {
      const source = loadTestCase('complex-scenarios', 'nested-conditions.js');
      const customConfig = {
        platform: {
          isMobile: true,
          isDesktop: false
        },
        mobile: {
          config: '{"theme": "dark", "animations": true}'
        },
        desktop: {
          config: '{"layout": "grid", "sidebar": true}'
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, customConfig);
      const analysis = optimizer.analyzeOptimization(source, optimized, customConfig);
      
      // Save snapshot
      saveSnapshot('complex-inline-defines', source, optimized, analysis);
      
      // Should replace mobile config with custom value
      expect(optimized).toContain('{"theme": "dark", "animations": true}');
    });
  });

  describe('Edge Cases', () => {
    it('should handle deeply nested conditions correctly', async () => {
      const source = loadTestCase('complex-scenarios', 'nested-conditions.js');
      const edgeConfig = {
        platform: {
          isMobile: false,
          isDesktop: true
        },
        featureFlags: {
          enableDesktopFeatures: true,
          enableAdvancedFeatures: true
        },
        user: {
          isPremium: true,
          isAdmin: true,
          permissions: {
            canAccessAdvanced: true
          }
        },
        environment: {
          isProduction: true
        },
        build: {
          target: 'production'
        }
      };
      
      const optimized = await optimizer.optimizeCode(source, edgeConfig);
      const analysis = optimizer.analyzeOptimization(source, optimized, edgeConfig);
      
      // Save snapshot
      saveSnapshot('complex-edge-case', source, optimized, analysis);
      
      // Should contain the deeply nested production advanced feature
      expect(optimized).toContain('productionAdvancedFeature');
      expect(optimized).toContain('adminDesktopFeature');
      expect(optimized).toContain('desktopFeature');
      
      // Should not contain mobile features
      expect(optimized).not.toContain('mobileOptimizedFeature');
      expect(optimized).not.toContain('premiumMobileFeature');
    });

    it('should handle empty configuration gracefully', async () => {
      const source = loadTestCase('complex-scenarios', 'nested-conditions.js');
      const emptyConfig = {};
      
      const optimized = await optimizer.optimizeCode(source, emptyConfig);
      const analysis = optimizer.analyzeOptimization(source, optimized, emptyConfig);
      
      // Save snapshot
      saveSnapshot('complex-empty-config', source, optimized, analysis);
      
      // Should still function and remove conditional blocks
      expect(analysis.sizes.optimized).toBeGreaterThan(0);
      
      // Should keep base functionality
      expect(optimized).toContain('baseFunction');
    });
  });

  describe('Performance with Complex Nesting', () => {
    it('should optimize complex nested conditions efficiently', async () => {
      const source = loadTestCase('complex-scenarios', 'nested-conditions.js');
      const testConfigs = [
        TEST_CONFIGS.complex.mobileProduction,
        TEST_CONFIGS.complex.desktopAdmin
      ];
      
      const results = [];
      
      for (const config of testConfigs) {
        const startTime = performance.now();
        const optimized = await optimizer.optimizeCode(source, config);
        const endTime = performance.now();
        
        const analysis = optimizer.analyzeOptimization(source, optimized, config);
        analysis.executionTime = endTime - startTime;
        
        results.push(analysis);
      }
      
      // All complex optimizations should complete in reasonable time
      results.forEach((analysis) => {
        expect(analysis.executionTime).toBeLessThan(2000); // Less than 2 seconds for complex cases
      });
      
      // Should achieve meaningful size reductions
      results.forEach((analysis) => {
        expect(analysis.sizes.reductionPercent).toBeGreaterThan(0);
      });
    });
  });
}); 