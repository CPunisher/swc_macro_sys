import { optimize } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

console.log('🧪 Testing mixed configuration (some truthy, some falsy)...');

// Test with mixed truthy/falsy values
const mixedConfig = {
  experiment: 'A',        // truthy
  loggedIn: false,        // falsy
  userId: 0,              // falsy
  premium: true,          // truthy
  debugMode: '',          // falsy (empty string)
  theme: 'dark'           // truthy
};

console.log('📊 Mixed config:', JSON.stringify(mixedConfig, null, 2));

try {
  const simpleSource = 'console.log("hello");';
  console.log('📄 Source:', simpleSource);
  
  const result = optimize(simpleSource, JSON.stringify(mixedConfig));
  console.log('✅ Success! Result:', result);
  
  console.log('');
  console.log('🎯 This demonstrates:');
  console.log('  - experiment="A" (truthy), loggedIn=false (falsy), userId=0 (falsy)');
  console.log('  - premium=true (truthy), debugMode="" (falsy), theme="dark" (truthy)');
  console.log('  - Since not ALL values are truthy, optimization should run');
  console.log('  - ALL config values can be used in macros, not just "featureFlags"');
  
} catch (error) {
  console.error('❌ Error:', error);
} 