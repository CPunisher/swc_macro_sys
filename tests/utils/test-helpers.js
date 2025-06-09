import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const projectRoot = path.resolve(__dirname, '../..');

/**
 * Test case configurations for different scenarios
 */
export const TEST_CONFIGS = {
  // Webpack bundle configurations
  webpack: {
    allFeatures: {
      features: {
        enableFeatureA: true,
        enableFeatureB: true,
        enableDebugMode: true
      }
    },
    featureAOnly: {
      features: {
        enableFeatureA: true,
        enableFeatureB: false,
        enableDebugMode: false
      }
    },
    featureBOnly: {
      features: {
        enableFeatureA: false,
        enableFeatureB: true,
        enableDebugMode: false
      }
    },
    minimal: {
      features: {
        enableFeatureA: false,
        enableFeatureB: false,
        enableDebugMode: false
      }
    },
    debugOnly: {
      features: {
        enableFeatureA: false,
        enableFeatureB: false,
        enableDebugMode: true
      }
    }
  },

  // Simple code configurations
  simple: {
    allFeatures: {
      featureFlags: {
        enableExpensiveFeature: true,
        enableDebugMode: true,
        enableExperimentalFeature: true
      },
      build: {
        mode: 'development'
      },
      api: {
        url: 'http://localhost:3000'
      },
      user: {
        isLoggedIn: true
      }
    },
    production: {
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
    },
    debugOnly: {
      featureFlags: {
        enableExpensiveFeature: false,
        enableDebugMode: true,
        enableExperimentalFeature: false
      },
      build: {
        mode: 'development'
      }
    }
  },

  // Complex nested configurations
  complex: {
    mobileProduction: {
      platform: {
        isMobile: true,
        isDesktop: false
      },
      featureFlags: {
        enableMobileOptimizations: true,
        enableDesktopFeatures: false,
        enableAdvancedFeatures: false
      },
      user: {
        isPremium: true,
        isAdmin: false,
        permissions: {
          canAccessAdvanced: false
        }
      },
      environment: {
        isProduction: true
      },
      build: {
        target: 'production'
      }
    },
    desktopAdmin: {
      platform: {
        isMobile: false,
        isDesktop: true
      },
      featureFlags: {
        enableMobileOptimizations: false,
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
    }
  },

  // JSX React component configurations
  jsx: {
    // Platform-specific configurations
    mobilePlatform: {
      platform: {
        isMobile: true,
        isDesktop: false,
        hasVibration: true,
        hasWakeLock: true,
        hasDeviceOrientation: true
      },
      featureFlags: {
        hasMobileCamera: true,
        hasDesktopShortcuts: false,
        hasAdvancedAnalytics: false,
        hasCollaboration: false,
        has3dVisualization: false,
        hasNotifications: true,
        hasVideoCalling: false,
        hasAiSuggestions: false
      },
      user: {
        isFree: true,
        isEnterprise: false,
        isPremium: false,
        isAdmin: false
      },
      abTests: {
        isGridLayout: false,
        isListLayout: true
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },
    desktopPlatform: {
      platform: {
        isMobile: false,
        isDesktop: true,
        hasVibration: false,
        hasWakeLock: false,
        hasDeviceOrientation: false,
        hasWebGL: true
      },
      featureFlags: {
        hasMobileCamera: false,
        hasDesktopShortcuts: true,
        hasAdvancedAnalytics: false,
        hasCollaboration: false,
        has3dVisualization: false,
        hasNotifications: true,
        hasVideoCalling: false,
        hasAiSuggestions: false
      },
      user: {
        isFree: true,
        isEnterprise: false,
        isPremium: false,
        isAdmin: false
      },
      abTests: {
        isGridLayout: true,
        isListLayout: false
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },

    // User tier configurations
    enterpriseUser: {
      platform: {
        isMobile: false,
        isDesktop: true,
        hasWebGL: true
      },
      featureFlags: {
        hasMobileCamera: false,
        hasDesktopShortcuts: true,
        hasAdvancedAnalytics: true,
        hasCollaboration: true,
        has3dVisualization: true,
        hasNotifications: true,
        hasVideoCalling: true,
        hasAiSuggestions: true
      },
      user: {
        isEnterprise: true,
        isPremium: false,
        isFree: false,
        isAdmin: false
      },
      abTests: {
        isGridLayout: true,
        isListLayout: false
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },
    premiumUser: {
      platform: {
        isMobile: false,
        isDesktop: true,
        hasWebGL: true
      },
      featureFlags: {
        hasMobileCamera: false,
        hasDesktopShortcuts: true,
        hasAdvancedAnalytics: true,
        hasCollaboration: false,
        has3dVisualization: true,
        hasNotifications: true,
        hasVideoCalling: false,
        hasAiSuggestions: true
      },
      user: {
        isFree: false,
        isEnterprise: false,
        isPremium: true,
        isAdmin: false
      },
      abTests: {
        isGridLayout: true,
        isListLayout: false
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },
    freeUser: {
      platform: {
        isMobile: false,
        isDesktop: true
      },
      featureFlags: {
        hasMobileCamera: false,
        hasDesktopShortcuts: false,
        hasAdvancedAnalytics: false,
        hasCollaboration: false,
        has3dVisualization: false,
        hasNotifications: true,
        hasVideoCalling: false,
        hasAiSuggestions: false
      },
      user: {
        isFree: true,
        isEnterprise: false,
        isPremium: false,
        isAdmin: false
      },
      abTests: {
        isGridLayout: true,
        isListLayout: false
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },

    // A/B Testing configurations
    gridLayout: {
      platform: {
        isMobile: false,
        isDesktop: true
      },
      featureFlags: {
        hasAdvancedAnalytics: true,
        has3dVisualization: true,
        hasNotifications: true
      },
      user: {
        isFree: false,
        isEnterprise: false,
        isPremium: true,
        isAdmin: false
      },
      abTests: {
        isGridLayout: true,
        isListLayout: false
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },
    listLayout: {
      platform: {
        isMobile: true,
        isDesktop: false
      },
      featureFlags: {
        hasAdvancedAnalytics: true,
        has3dVisualization: true,
        hasMobileCamera: true,
        hasNotifications: true
      },
      user: {
        isFree: false,
        isEnterprise: false,
        isPremium: true,
        isAdmin: false
      },
      abTests: {
        isGridLayout: false,
        isListLayout: true
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },

    // Complex combinations
    mobilePremiumCamera: {
      platform: {
        isMobile: true,
        isDesktop: false,
        hasVibration: true,
        hasWakeLock: true,
        hasDeviceOrientation: true
      },
      featureFlags: {
        hasMobileCamera: true,
        hasDesktopShortcuts: false,
        hasAdvancedAnalytics: true,
        hasCollaboration: false,
        has3dVisualization: true,
        hasNotifications: true,
        hasVideoCalling: false,
        hasAiSuggestions: true
      },
      user: {
        isFree: false,
        isEnterprise: false,
        isPremium: true,
        isAdmin: false
      },
      abTests: {
        isGridLayout: false,
        isListLayout: true
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },
    adminUser: {
      platform: {
        isMobile: false,
        isDesktop: true,
        hasWebGL: true
      },
      featureFlags: {
        hasMobileCamera: false,
        hasDesktopShortcuts: true,
        hasAdvancedAnalytics: true,
        hasCollaboration: true,
        has3dVisualization: true,
        hasNotifications: true,
        hasVideoCalling: true,
        hasAiSuggestions: true
      },
      user: {
        isFree: false,
        isEnterprise: false,
        isPremium: false,
        isAdmin: true
      },
      abTests: {
        isGridLayout: true,
        isListLayout: false
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },
    minimalConfig: {
      platform: {
        isMobile: false,
        isDesktop: true
      },
      featureFlags: {
        hasMobileCamera: false,
        hasDesktopShortcuts: false,
        hasAdvancedAnalytics: false,
        hasCollaboration: false,
        has3dVisualization: false,
        hasNotifications: false,
        hasVideoCalling: false,
        hasAiSuggestions: false
      },
      user: {
        isFree: true,
        isEnterprise: false,
        isPremium: false,
        isAdmin: false
      },
      abTests: {
        isGridLayout: true,
        isListLayout: false
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },

    // Build and inline define configurations
    productionBuild: {
      platform: {
        isMobile: false,
        isDesktop: true
      },
      featureFlags: {
        hasAdvancedAnalytics: true,
        hasNotifications: true
      },
      user: {
        isFree: false,
        isEnterprise: false,
        isPremium: true,
        isAdmin: false
      },
      abTests: {
        isGridLayout: true,
        isListLayout: false
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    },
    complexNested: {
      platform: {
        isMobile: true,
        isDesktop: false,
        hasVibration: true,
        hasWakeLock: true,
        hasDeviceOrientation: true
      },
      featureFlags: {
        hasMobileCamera: true,
        hasDesktopShortcuts: false,
        hasAdvancedAnalytics: true,
        hasCollaboration: true,
        has3dVisualization: true,
        hasNotifications: true,
        hasVideoCalling: true,
        hasAiSuggestions: true
      },
      user: {
        isFree: false,
        isEnterprise: true,
        isPremium: false,
        isAdmin: false
      },
      abTests: {
        isGridLayout: false,
        isListLayout: true
      },
      build: {
        target: 'production',
        timestamp: new Date().toISOString()
      }
    }
  }
};

/**
 * Load test case source file
 */
export function loadTestCase(category, filename) {
  const filePath = path.join(projectRoot, 'test-cases', category, filename);
  
  if (!fs.existsSync(filePath)) {
    throw new Error(`Test case file not found: ${filePath}`);
  }
  
  return fs.readFileSync(filePath, 'utf8');
}

/**
 * Get all test case files in a category
 */
export function getTestCaseFiles(category) {
  const categoryPath = path.join(projectRoot, 'test-cases', category);
  
  if (!fs.existsSync(categoryPath)) {
    return [];
  }
  
  return fs.readdirSync(categoryPath)
    .filter(file => file.endsWith('.js'))
    .map(file => ({
      filename: file,
      name: path.basename(file, '.js'),
      path: path.join(categoryPath, file)
    }));
}

/**
 * Create test snapshots directory
 */
export function ensureSnapshotsDir(testName) {
  const snapshotsDir = path.join(projectRoot, 'test-results', 'snapshots', testName);
  fs.mkdirSync(snapshotsDir, { recursive: true });
  return snapshotsDir;
}

/**
 * Save optimization snapshot
 */
export function saveSnapshot(testName, original, optimized, analysis) {
  const snapshotsDir = ensureSnapshotsDir(testName);
  
  // Save original source
  fs.writeFileSync(
    path.join(snapshotsDir, 'original.js'),
    original
  );
  
  // Save optimized source
  fs.writeFileSync(
    path.join(snapshotsDir, 'optimized.js'),
    optimized
  );
  
  // Save analysis results
  fs.writeFileSync(
    path.join(snapshotsDir, 'analysis.json'),
    JSON.stringify(analysis, null, 2)
  );
  
  return snapshotsDir;
}

/**
 * Expected results for webpack test cases
 */
export const EXPECTED_WEBPACK_MODULES = {
  allFeatures: ['153', '418', '78', '722', '803', '812', '422'],
  featureAOnly: ['153', '418', '78'],
  featureBOnly: ['722', '803', '812'], 
  minimal: [],
  debugOnly: ['422']
};

/**
 * Module ID to name mapping
 */
export const MODULE_NAMES = {
  '153': 'featureA',
  '418': 'dataProcessor',
  '78': 'heavyMathUtils',
  '722': 'featureB',
  '803': 'expensiveUIUtils',
  '812': 'networkUtils',
  '422': 'debugUtils'
};

/**
 * Validate optimization results
 */
export function validateOptimization(testName, analysis, expectedModules = null) {
  const results = {
    testName,
    passed: true,
    errors: [],
    analysis
  };
  
  // Check if optimization reduced size (unless all features enabled)
  if (testName !== 'allFeatures' && analysis.sizes.reduction <= 0) {
    results.passed = false;
    results.errors.push('Expected size reduction but got none');
  }
  
  // Check expected modules if provided
  if (expectedModules && EXPECTED_WEBPACK_MODULES[testName]) {
    const expected = EXPECTED_WEBPACK_MODULES[testName];
    const actualModules = analysis.modules.optimized;
    
    if (actualModules !== expected.length) {
      results.passed = false;
      results.errors.push(
        `Expected ${expected.length} modules, got ${actualModules}. Expected: [${expected.join(', ')}]`
      );
    }
  }
  
  return results;
}

/**
 * Validate that macro comments are properly removed
 */
export function validateMacroRemoval(optimizedCode, conditionText) {
  // Check that the specific condition text is not present in optimized code
  expect(optimizedCode).not.toContain(`condition="${conditionText}"`);
  
  // Check that no macro syntax remains
  expect(optimizedCode).not.toContain('@common:if');
  expect(optimizedCode).not.toContain('@common:endif');
  expect(optimizedCode).not.toContain('@common:define-inline');
}

/**
 * Generate test report
 */
export function generateTestReport(results) {
  const report = {
    timestamp: new Date().toISOString(),
    summary: {
      total: results.length,
      passed: results.filter(r => r.passed).length,
      failed: results.filter(r => !r.passed).length
    },
    results: results,
    totalSizeReduction: results.reduce((acc, r) => acc + (r.analysis?.sizes?.reduction || 0), 0)
  };
  
  // Save report
  const reportPath = path.join(projectRoot, 'test-results', 'optimization-report.json');
  fs.mkdirSync(path.dirname(reportPath), { recursive: true });
  fs.writeFileSync(reportPath, JSON.stringify(report, null, 2));
  
  return report;
} 
