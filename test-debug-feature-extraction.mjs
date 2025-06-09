import { optimize } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

console.log('🧪 Testing feature extraction step by step...');

const testConfigs = [
  // Test 1: All truthy (should work)
  {
    name: 'all-truthy',
    config: { experiment: 'A', loggedIn: true, userId: 12345 }
  },
  // Test 2: Single falsy value (might fail)
  {
    name: 'one-falsy',
    config: { experiment: 'A', loggedIn: false }
  },
  // Test 3: Multiple falsy values (might fail)
  {
    name: 'multiple-falsy',
    config: { experiment: 'A', loggedIn: false, userId: 0 }
  }
];

for (const test of testConfigs) {
  console.log(`\n📊 Testing: ${test.name}`);
  console.log(`Config: ${JSON.stringify(test.config)}`);
  
  try {
    const result = optimize('console.log("test");', JSON.stringify(test.config));
    console.log(`✅ Success: ${result}`);
  } catch (error) {
    console.log(`❌ Failed at ${test.name}:`, error.message);
    console.log('This helps identify which configuration causes the issue');
    break; // Stop at first failure to isolate the problem
  }
} 