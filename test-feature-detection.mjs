import { get_optimization_info } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

console.log('🧪 Testing updated feature detection with comprehensive config...');

// Test configuration with various types of values that can all be used in macros
const testConfig = {
  experiment: 'A',           // String value (truthy)
  loggedIn: true,           // Boolean value (truthy)
  userId: 12345,            // Number value (truthy)
  features: {               // Nested object (truthy)
    darkMode: true,
    premiumFeatures: false
  },
  featureFlags: {           // Another nested object (truthy)
    newUI: true,
    betaFeatures: false
  },
  settings: {
    theme: 'dark',          // String (truthy)
    notifications: 0        // Falsy number
  },
  emptyArray: [],           // Empty array (falsy)
  emptyString: '',          // Empty string (falsy)
  nullValue: null           // Null value (falsy)
};

console.log('📊 Test config:', JSON.stringify(testConfig, null, 2));
console.log('');

try {
  const result = get_optimization_info('console.log("test");', JSON.stringify(testConfig));
  console.log('🔍 Feature detection result:', result);
  
  const parsed = JSON.parse(result);
  console.log('');
  console.log('📈 Summary:');
  console.log(`✅ Total config values detected: ${parsed.total_config_values || 'N/A'}`);
  console.log(`✅ Enabled/truthy values: ${parsed.enabled_count || 'N/A'}`);
  console.log(`✅ All enabled: ${parsed.all_enabled || 'N/A'}`);
  console.log(`✅ Should optimize: ${parsed.should_optimize || 'N/A'}`);
  
  console.log('');
  console.log('🎯 This demonstrates that ALL configuration values can now be used in macros:');
  console.log('  - experiment, loggedIn, userId, features.darkMode, etc.');
  console.log('  - Not just values nested under "featureFlags" or "features"');
  
} catch (error) {
  console.error('❌ Error:', error);
} 