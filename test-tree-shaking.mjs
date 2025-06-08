import { optimize } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';
import fs from 'fs';

// Feature to module dependency mapping
const FEATURE_MODULE_DEPENDENCIES = {
  'features.enableFeatureA': ['153', '418', '78'], // featureA + dataProcessor + heavyMathUtils
  'features.enableFeatureB': ['722', '803', '812'], // featureB + expensiveUIUtils + networkUtils
  'features.enableDebugMode': ['422'] // debugUtils
};

// All module IDs and their descriptions
const ALL_MODULES = {
  '418': 'dataProcessor',
  '422': 'debugUtils', 
  '803': 'expensiveUIUtils',
  '153': 'featureA',
  '722': 'featureB',
  '78': 'heavyMathUtils',
  '812': 'networkUtils'
};

function getExpectedModulesForConfig(config) {
  const expectedModules = new Set();
  
  // Add modules for enabled features
  Object.entries(config).forEach(([feature, enabled]) => {
    if (enabled) {
      const featureKey = `features.${feature}`;
      const dependencies = FEATURE_MODULE_DEPENDENCIES[featureKey] || [];
      dependencies.forEach(moduleId => expectedModules.add(moduleId));
    }
  });
  
  return expectedModules;
}

function analyzeBundle(code, description, config = null) {
  // Basic stats
  const totalSize = code.length;
  const totalLines = code.split('\n').length;
  
  // Check which modules are present
  const moduleStatuses = {};
  
  if (config) {
    const expectedModules = getExpectedModulesForConfig(config);
    
    Object.entries(ALL_MODULES).forEach(([id, name]) => {
      const isPresent = code.includes(`${id}:`);
      const shouldBePresent = expectedModules.has(id);
      
      if (shouldBePresent && isPresent) {
        moduleStatuses[id] = `✅ ${name} (correctly included)`;
      } else if (!shouldBePresent && !isPresent) {
        moduleStatuses[id] = `✅ ${name} (correctly removed)`;
      } else if (!shouldBePresent && isPresent) {
        moduleStatuses[id] = `❌ ${name} (should be removed but kept)`;
      } else if (shouldBePresent && !isPresent) {
        moduleStatuses[id] = `❌ ${name} (should be present but missing)`;
      }
    });
  } else {
    // For original bundle, just show present/absent without judgment
    Object.entries(ALL_MODULES).forEach(([id, name]) => {
      const isPresent = code.includes(`${id}:`);
      moduleStatuses[id] = isPresent ? `✅ ${name}` : `❌ ${name} (not present)`;
    });
  }
  
  console.log(`📊 ${description}:`);
  console.log(`   Size: ${totalSize} chars (${totalLines} lines)`);
  
  if (config) {
    const expectedModules = getExpectedModulesForConfig(config);
    const actualModules = new Set();
    
    Object.entries(ALL_MODULES).forEach(([id, name]) => {
      if (code.includes(`${id}:`)) {
        actualModules.add(id);
      }
    });
    
    console.log(`   Expected modules for this config: [${Array.from(expectedModules).join(', ')}]`);
    console.log(`   Actually present modules: [${Array.from(actualModules).join(', ')}]`);
    
    // Check for correctly removed modules
    const shouldBeRemoved = new Set();
    Object.keys(ALL_MODULES).forEach(id => {
      if (!expectedModules.has(id)) {
        shouldBeRemoved.add(id);
      }
    });
    
    const correctlyRemoved = [];
    const incorrectlyKept = [];
    
    shouldBeRemoved.forEach(id => {
      if (!actualModules.has(id)) {
        correctlyRemoved.push(id);
      } else {
        incorrectlyKept.push(id);
      }
    });
    
    if (correctlyRemoved.length > 0) {
      console.log(`   ✅ Correctly removed: [${correctlyRemoved.map(id => ALL_MODULES[id]).join(', ')}]`);
    }
    if (incorrectlyKept.length > 0) {
      console.log(`   ❌ Should be removed but kept: [${incorrectlyKept.map(id => ALL_MODULES[id]).join(', ')}]`);
    }
  }
  
  console.log(`   Module-by-module status:`);
  Object.entries(moduleStatuses).forEach(([id, status]) => {
    console.log(`     ${id}: ${status}`);
  });
  
  // Count issues for summary
  const issues = Object.values(moduleStatuses).filter(status => status.includes('❌')).length;
  const correct = Object.values(moduleStatuses).filter(status => status.includes('✅')).length;
  
  if (config && issues > 0) {
    console.log(`   📊 Summary: ${correct} correct, ${issues} issues`);
  }
  
  return { totalSize, totalLines, moduleStatuses, issues: issues || 0 };
}

async function testTreeShaking() {
  console.log('🌳 Tree Shaking Demo - Processing bundler-chunk.js');
  console.log('============================================================');
  
  // Read the bundled code
  const originalCode = fs.readFileSync('bundler-chunk.js', 'utf8');
  
  // Analyze original bundle
  const originalStats = analyzeBundle(originalCode, 'Original Bundle');
  console.log('');
  
  try {
    console.log('✅ WASM module loaded successfully');
    console.log('');
    
    console.log('🔄 Testing different feature flag configurations...');
    console.log('');
    
    // Test configurations with expected results
    const testConfigs = [
      {
        name: 'allEnabled',
        config: { enableFeatureA: true, enableFeatureB: true, enableDebugMode: true },
        description: 'All features enabled - should keep all modules'
      },
      {
        name: 'onlyFeatureA', 
        config: { enableFeatureA: true, enableFeatureB: false, enableDebugMode: false },
        description: 'Only Feature A - should remove featureB, expensiveUIUtils, networkUtils, debugUtils'
      },
      {
        name: 'onlyFeatureB',
        config: { enableFeatureA: false, enableFeatureB: true, enableDebugMode: false },
        description: 'Only Feature B - should remove featureA, heavyMathUtils, dataProcessor, debugUtils'
      },
      {
        name: 'minimal',
        config: { enableFeatureA: false, enableFeatureB: false, enableDebugMode: false },
        description: 'No features - should remove all feature modules, keep only base functionality'
      }
    ];
    
    for (const test of testConfigs) {
      console.log(`⚙️  Testing: ${test.name}`);
      console.log(`   Config: ${JSON.stringify(test.config)}`);
      console.log(`   Expected: ${test.description}`);
      
      const startTime = performance.now();
      // Use the old format that the WASM expects
      const configForWasm = { features: test.config };
      const optimizedCode = optimize(originalCode, JSON.stringify(configForWasm));
      const endTime = performance.now();
      
      const optimizedStats = analyzeBundle(optimizedCode, `Optimized (${test.name})`, test.config);
      
      console.log('');
      console.log(`   ⚡ Optimized in ${(endTime - startTime).toFixed(2)}ms`);
      
      const sizeDiff = optimizedStats.totalSize - originalStats.totalSize;
      const sizePercent = ((sizeDiff / originalStats.totalSize) * 100).toFixed(1);
      console.log(`   📉 Size change: ${originalStats.totalSize} → ${optimizedStats.totalSize} chars (${sizePercent > 0 ? '+' : ''}${sizePercent}%)`);
      
      console.log('');
      console.log(`   📄 Optimized output (${test.name}):`);
      console.log('   ==================================================');
      // Show a condensed version of the output
      const lines = optimizedCode.split('\n');
      if (lines.length > 50) {
        console.log(lines.slice(0, 25).join('\n'));
        console.log('   ... (truncated) ...');
        console.log(lines.slice(-25).join('\n'));
      } else {
        console.log(optimizedCode);
      }
      console.log('   ==================================================');
      console.log('--------------------------------------------------');
    }
    
    console.log('');
    console.log('🎯 Tree Shaking Analysis Summary:');
    console.log('✅ Conditional compilation working at entry point level');
    console.log('❓ Module-level tree shaking needs enhancement');
  } catch (error) {
    console.error('❌ Error during processing:', error);
  }
}

testTreeShaking(); 