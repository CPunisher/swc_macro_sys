// there is no wasm initialization error. never change the wasm implementation here.
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

// Results tracking for summary
const testResults = [];

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
  const originalSize = originalCode.length;
  
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
      
      const optimizedSize = optimizedCode.length;
      const sizeReduction = originalSize - optimizedSize;
      const reductionPercent = ((sizeReduction / originalSize) * 100);
      
      const optimizedStats = analyzeBundle(optimizedCode, `Optimized (${test.name})`, test.config);
      
      console.log('');
      console.log(`   ⚡ Optimized in ${(endTime - startTime).toFixed(2)}ms`);
      console.log(`   📉 Size: ${originalSize} → ${optimizedSize} chars (${sizeReduction >= 0 ? '-' : '+'}${Math.abs(reductionPercent).toFixed(1)}%)`);
      
      // Store results for summary
      testResults.push({
        name: test.name,
        description: test.description,
        originalSize,
        optimizedSize,
        sizeReduction,
        reductionPercent,
        executionTime: endTime - startTime,
        issues: optimizedStats.issues
      });
      
      console.log('');
      console.log(`   📄 Optimized output (${test.name}):`);
      console.log('   ==================================================');
      // Show a condensed version of the output
      console.log(optimizedCode)
      console.log('   ==================================================');
      console.log('--------------------------------------------------');
    }
    
    // Print comprehensive summary
    printFinalSummary();
    
  } catch (error) {
    console.error('❌ Error during processing:', error);
  }
}

function printFinalSummary() {
  console.log('');
  console.log('📈 TREE SHAKING PERFORMANCE SUMMARY');
  console.log('================================================================');
  console.log(`📦 Original bundle size: ${testResults[0]?.originalSize || 'N/A'} chars`);
  console.log('');
  
  console.log('🎯 Optimization Results by Scenario:');
  console.log('┌─────────────────┬─────────────┬─────────────┬─────────────┬──────────┬─────────┐');
  console.log('│ Scenario        │ Original    │ Optimized   │ Reduction   │ % Saved  │ Issues  │');
  console.log('├─────────────────┼─────────────┼─────────────┼─────────────┼──────────┼─────────┤');
  
  testResults.forEach(result => {
    const scenarioName = result.name.padEnd(15);
    const originalSize = result.originalSize.toString().padStart(11);
    const optimizedSize = result.optimizedSize.toString().padStart(11);
    const reduction = result.sizeReduction.toString().padStart(11);
    const percent = `${result.reductionPercent.toFixed(1)}%`.padStart(8);
    const issues = result.issues.toString().padStart(7);
    
    console.log(`│ ${scenarioName} │ ${originalSize} │ ${optimizedSize} │ ${reduction} │ ${percent} │ ${issues} │`);
  });
  
  console.log('└─────────────────┴─────────────┴─────────────┴─────────────┴──────────┴─────────┘');
  console.log('');
  
  // Calculate average reduction
  const avgReduction = testResults.reduce((sum, result) => sum + result.reductionPercent, 0) / testResults.length;
  const bestReduction = Math.max(...testResults.map(r => r.reductionPercent));
  const worstReduction = Math.min(...testResults.map(r => r.reductionPercent));
  
  console.log('📊 Performance Metrics:');
  console.log(`   🏆 Best reduction: ${bestReduction.toFixed(1)}% (${testResults.find(r => r.reductionPercent === bestReduction)?.name})`);
  console.log(`   📉 Worst reduction: ${worstReduction.toFixed(1)}% (${testResults.find(r => r.reductionPercent === worstReduction)?.name})`);
  console.log(`   📊 Average reduction: ${avgReduction.toFixed(1)}%`);
  
  const avgExecutionTime = testResults.reduce((sum, result) => sum + result.executionTime, 0) / testResults.length;
  console.log(`   ⚡ Average execution time: ${avgExecutionTime.toFixed(2)}ms`);
  
  const totalIssues = testResults.reduce((sum, result) => sum + result.issues, 0);
  console.log(`   🎯 Total accuracy issues: ${totalIssues}`);
  
  console.log('');
  
  // CRITICAL: Make test fail if there are any accuracy issues
  if (totalIssues > 0) {
    console.log('❌ TREE SHAKING TEST FAILED!');
    console.log(`   Found ${totalIssues} accuracy issues across test scenarios`);
    console.log('   Expected: All modules should be correctly included/excluded based on feature flags');
    console.log('   Actual: Some modules are incorrectly kept when they should be removed');
    console.log('');
    console.log('🔧 This indicates problems with:');
    console.log('   - Conditional macro processing not removing feature-flagged code');
    console.log('   - Tree shaking not properly detecting unused modules');
    console.log('   - Module graph analysis missing dependency relationships');
    console.log('');
    process.exit(1);
  }
  
  console.log('🎉 Tree Shaking Analysis Complete!');
  console.log('✅ Conditional compilation working at entry point level');
  console.log('✅ Advanced webpack module graph analysis operational');
  console.log(`✅ Achieving ${avgReduction.toFixed(1)}% average bundle size reduction`);
}

testTreeShaking(); 