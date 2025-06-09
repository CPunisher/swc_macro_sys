import { optimize } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

console.log('🧪 Testing conditional macro behavior...');

const simpleCode = `
// Test conditional compilation
console.log("Always present");

/* @common:if [condition="features.testFlag"] */
console.log("This should only appear when testFlag is true");
/* @common:endif */

console.log("Always present end");
`;

console.log('📝 Input code:');
console.log(simpleCode);

// Test 1: testFlag = true (should include conditional code)
console.log('\n🧪 Test 1: testFlag = true');
const config1 = { features: { testFlag: true } };
console.log('Config:', JSON.stringify(config1));
try {
  const result1 = optimize(simpleCode, JSON.stringify(config1));
  console.log('Result:');
  console.log(result1);
} catch (error) {
  console.error('Error:', error.message);
}

// Test 2: testFlag = false (should exclude conditional code)
console.log('\n🧪 Test 2: testFlag = false');
const config2 = { features: { testFlag: false } };
console.log('Config:', JSON.stringify(config2));
try {
  const result2 = optimize(simpleCode, JSON.stringify(config2));
  console.log('Result:');
  console.log(result2);
} catch (error) {
  console.error('Error:', error.message);
}

// Test 3: no features (should exclude conditional code)
console.log('\n🧪 Test 3: no features');
const config3 = {};
console.log('Config:', JSON.stringify(config3));
try {
  const result3 = optimize(simpleCode, JSON.stringify(config3));
  console.log('Result:');
  console.log(result3);
} catch (error) {
  console.error('Error:', error.message);
} 