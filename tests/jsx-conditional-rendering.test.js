import { describe, test, expect, beforeAll } from 'vitest';
import { optimizer } from './utils/optimizer.js';
import { 
  loadTestCase, 
  TEST_CONFIGS, 
  saveSnapshot,
  validateMacroRemoval
} from './utils/test-helpers.js';

describe('JSX Conditional Rendering', () => {
  beforeAll(async () => {
    await optimizer.initialize();
  });

  const loadJSXSource = () => {
    return loadTestCase('simple-code', 'jsx-conditional-rendering.js');
  };

  describe('Platform-specific rendering', () => {
    test('should optimize for mobile platform only', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.mobilePlatform;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include mobile-specific content
      expect(optimized).toContain('📱 Mobile Interface Active');
      expect(optimized).toContain('📷 Camera access enabled');
      
      // Should NOT include desktop-specific content  
      expect(optimized).not.toContain('🖥️ Desktop Interface Active');
      expect(optimized).not.toContain('⌨️ Keyboard shortcuts');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'platform.isDesktop');
      
      saveSnapshot('jsx-mobile-platform', source, optimized, analysis);
    });

    test('should optimize for desktop platform only', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.desktopPlatform;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include desktop-specific content
      expect(optimized).toContain('🖥️ Desktop Interface Active');
      expect(optimized).toContain('⌨️ Keyboard shortcuts');
      
      // Should NOT include mobile-specific content
      expect(optimized).not.toContain('📱 Mobile Interface Active');
      expect(optimized).not.toContain('📷 Camera access');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'platform.isMobile');
      
      saveSnapshot('jsx-desktop-platform', source, optimized, analysis);
    });
  });

  describe('User tier optimization', () => {
    test('should optimize for enterprise user features', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.enterpriseUser;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include enterprise features
      expect(optimized).toContain('🏢 Enterprise Features');
      expect(optimized).toContain('👥 Real-time collaboration');
      expect(optimized).toContain('📈 Enterprise analytics');
      
      // Should NOT include premium/free specific features
      expect(optimized).not.toContain('⭐ Premium Features');
      expect(optimized).not.toContain('🆓 Free Tier');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'user.type === \'premium\'');
      
      saveSnapshot('jsx-enterprise-user', source, optimized, analysis);
    });

    test('should optimize for premium user features', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.premiumUser;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include premium features
      expect(optimized).toContain('⭐ Premium Features');
      expect(optimized).toContain('📊 Advanced analytics');
      expect(optimized).toContain('🤖 AI-powered suggestions');
      
      // Should NOT include enterprise/free specific features
      expect(optimized).not.toContain('🏢 Enterprise Features');
      expect(optimized).not.toContain('👥 Real-time collaboration');
      expect(optimized).not.toContain('🆓 Free Tier');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'user.type === \'enterprise\'');
      
      saveSnapshot('jsx-premium-user', source, optimized, analysis);
    });

    test('should optimize for free user features', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.freeUser;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include free tier features
      expect(optimized).toContain('🆓 Free Tier');
      expect(optimized).toContain('Basic features available');
      
      // Should NOT include premium/enterprise features
      expect(optimized).not.toContain('⭐ Premium Features');
      expect(optimized).not.toContain('🏢 Enterprise Features');
      expect(optimized).not.toContain('📊 Advanced analytics');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'user.type === \'enterprise\'');
      
      saveSnapshot('jsx-free-user', source, optimized, analysis);
    });
  });

  describe('A/B Testing optimization', () => {
    test('should optimize for grid dashboard layout', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.gridLayout;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include grid layout elements
      expect(optimized).toContain('Grid Item 1');
      expect(optimized).toContain('Grid Item 2');
      expect(optimized).toContain('gridTemplateColumns');
      
      // Should NOT include list layout elements
      expect(optimized).not.toContain('Linear layout active');
      expect(optimized).not.toContain('List Item 1');
      expect(optimized).not.toContain('flexDirection:');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'abTests.dashboardLayout === \'list\'');
      
      saveSnapshot('jsx-grid-layout', source, optimized, analysis);
    });

    test('should optimize for list dashboard layout', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.listLayout;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include list layout elements
      expect(optimized).toContain('Linear layout active');
      expect(optimized).toContain('List Item 1');
      expect(optimized).toContain('flexDirection:');
      
      // Should NOT include grid layout elements  
      expect(optimized).not.toContain('Grid Item 1');
      expect(optimized).not.toContain('gridTemplateColumns');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'abTests.dashboardLayout === \'grid\'');
      
      saveSnapshot('jsx-list-layout', source, optimized, analysis);
    });
  });

  describe('Feature flag combinations', () => {
    test('should optimize for mobile premium user with camera', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.mobilePremiumCamera;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include mobile + premium + camera features
      expect(optimized).toContain('📱 Mobile Features');
      expect(optimized).toContain('📷 Camera access enabled');
      expect(optimized).toContain('⭐ Premium Features');
      expect(optimized).toContain('📊 Advanced analytics');
      
      // Should NOT include desktop features
      expect(optimized).not.toContain('🖥️ Desktop Interface');
      expect(optimized).not.toContain('⌨️ Keyboard shortcuts');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'featureFlags.desktop-shortcuts');
      
      saveSnapshot('jsx-mobile-premium-camera', source, optimized, analysis);
    });

    test('should optimize for admin user with all privileges', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.adminUser;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should include admin panel
      expect(optimized).toContain('🔧 Admin Panel');
      expect(optimized).toContain('Feature Flags Control');
      expect(optimized).toContain('A/B Test Management');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      saveSnapshot('jsx-admin-user', source, optimized, analysis);
    });

    test('should optimize for minimal configuration', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.minimalConfig;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should only include basic features
      expect(optimized).toContain('🆓 Free Tier');
      expect(optimized).toContain('Basic features available');
      
      // Should NOT include advanced features
      expect(optimized).not.toContain('📊 Advanced analytics');
      expect(optimized).not.toContain('👥 Real-time collaboration');
      expect(optimized).not.toContain('📷 Camera access');
      expect(optimized).not.toContain('🔧 Admin Panel');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'featureFlags.advanced-analytics');
      
      saveSnapshot('jsx-minimal-config', source, optimized, analysis);
    });
  });

  describe('Inline defines processing', () => {
    test('should process build configuration inline defines', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.productionBuild;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should replace inline defines with actual values
      expect(optimized).toContain('production');
      expect(optimized).not.toContain('@common:define-inline');
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      saveSnapshot('jsx-production-build', source, optimized, analysis);
    });
  });

  describe('Complex nested conditions', () => {
    test('should handle deeply nested platform and feature combinations', async () => {
      const source = loadJSXSource();
      const config = TEST_CONFIGS.jsx.complexNested;
      
      const optimized = await optimizer.optimizeCode(source, config, { isJSX: true });
      const analysis = optimizer.analyzeOptimization(source, optimized, config);
      
      // Should preserve mobile features and remove desktop features
      expect(optimized).toContain('📱 Mobile Interface Active');
      expect(optimized).toContain('✨ Haptic feedback available');
      expect(optimized).not.toContain('🖥️ Desktop Interface Active');
      expect(optimized).not.toContain('⌨️ Keyboard shortcuts');
      
      expect(analysis.sizes.reduction).toBeGreaterThan(0);
      
      validateMacroRemoval(optimized, 'platform.isDesktop');
      
      saveSnapshot('jsx-complex-nested', source, optimized, analysis);
    });
  });
}); 