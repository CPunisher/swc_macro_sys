#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Import the SWC macro WASM optimizer
async function loadOptimizer() {
  try {
    const swcMacro = await import('../../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js');
    return swcMacro;
  } catch (error) {
    console.error('Failed to load SWC macro optimizer:', error.message);
    process.exit(1);
  }
}

async function main() {
  console.log('🧪 TESTING RUST TEST PARITY IN JAVASCRIPT\n');

  // Use the same files as the Rust test
  const mfChunkPath = path.resolve(__dirname, '../../crates/swc_macro_wasm/tests/fixtures/module_federation_lodash_chunk.js');
  const hostUsagePath = path.resolve(__dirname, '../../crates/swc_macro_wasm/tests/fixtures/module_federation_usage.json');
  const remoteUsagePath = path.resolve(__dirname, '../../crates/swc_macro_wasm/tests/fixtures/module_federation_remote_usage.json');

  // Check if fixtures exist
  if (!fs.existsSync(mfChunkPath)) {
    console.log('❌ MF chunk fixture not found:', mfChunkPath);
    console.log('Run cargo test first to generate fixtures');
    process.exit(1);
  }

  console.log('='.repeat(80));
  console.log('📊 LOADING TEST FIXTURES');
  console.log('='.repeat(80));

  // Load the exact same data as the Rust test
  const originalCode = fs.readFileSync(mfChunkPath, 'utf8');
  const originalSize = originalCode.length;

  console.log(`Real MF chunk size: ${originalSize} bytes (${(originalSize/1024).toFixed(2)} KB)`);

  // Load usage data exactly like the Rust test
  const hostUsage = JSON.parse(fs.readFileSync(hostUsagePath, 'utf8'));
  const remoteUsage = JSON.parse(fs.readFileSync(remoteUsagePath, 'utf8'));

  // Extract usage patterns exactly like the Rust test
  const hostUsed = hostUsage.consume_shared_modules['lodash-es'].used_exports;
  const remoteUsed = remoteUsage.consume_shared_modules['lodash-es'].used_exports;
  const unusedExports = hostUsage.consume_shared_modules['lodash-es'].unused_exports;

  console.log(`Host app uses ${hostUsed.length} lodash exports: [${hostUsed.join(', ')}]`);
  console.log(`Remote app uses ${remoteUsed.length} lodash exports: [${remoteUsed.join(', ')}]`);

  // Merge used exports (union) exactly like the Rust test
  const allUsedExports = new Set([...hostUsed, ...remoteUsed]);
  console.log(`Combined used exports: ${allUsedExports.size} total`);

  // Create tree shake config exactly like the Rust test
  const treeShakeConfig = {};
  
  // Mark used exports as true, unused as false
  for (const exportName of allUsedExports) {
    treeShakeConfig[exportName] = true;
  }
  for (const exportName of unusedExports) {
    if (!allUsedExports.has(exportName)) {
      treeShakeConfig[exportName] = false;
    }
  }

  const config = {
    treeShake: {
      'lodash-es': treeShakeConfig
    }
  };

  console.log(`Tree shake config includes ${Object.keys(treeShakeConfig).length} exports (${allUsedExports.size} used, ${Object.keys(treeShakeConfig).length - allUsedExports.size} unused)`);

  console.log('\n' + '='.repeat(80));
  console.log('🔧 RUNNING OPTIMIZATION (DIRECT RUST TEST REPLICATION)');
  console.log('='.repeat(80));

  // Load optimizer
  const optimizer = await loadOptimizer();

  // Run optimization exactly like the Rust test
  const configStr = JSON.stringify(config);
  const optimizedCode = optimizer.optimize(originalCode, configStr);
  const optimizedSize = optimizedCode.length;
  const reduction = ((originalSize - optimizedSize) / originalSize) * 100;

  console.log('Real MF chunk optimization results:');
  console.log(`  Original size: ${originalSize} bytes (${(originalSize/1024).toFixed(2)} KB)`);
  console.log(`  Optimized size: ${optimizedSize} bytes (${(optimizedSize/1024).toFixed(2)} KB)`);
  console.log(`  Size reduction: ${reduction.toFixed(2)}% (${originalSize - optimizedSize} bytes saved)`);

  console.log('\n' + '='.repeat(80));
  console.log('📋 ANALYSIS');
  console.log('='.repeat(80));

  // Analyze the results
  console.log('CHUNK FORMAT ANALYSIS:');
  console.log(`  Original contains 'exports.modules': ${originalCode.includes('exports.modules')}`);
  console.log(`  Optimized contains 'exports.modules': ${optimizedCode.includes('exports.modules')}`);
  console.log(`  Original contains '__webpack_require__': ${originalCode.includes('__webpack_require__')}`);
  console.log(`  Optimized contains '__webpack_require__': ${optimizedCode.includes('__webpack_require__')}`);

  // Module counting
  const originalModules = countLodashModules(originalCode);
  const optimizedModules = countLodashModules(optimizedCode);
  
  console.log(`\nMODULE ANALYSIS:`);
  console.log(`  Original lodash modules: ${originalModules}`);
  console.log(`  Optimized lodash modules: ${optimizedModules}`);
  console.log(`  Modules removed: ${originalModules - optimizedModules}`);

  // Key lodash modules check
  const keyModules = ['map.js', 'filter.js', 'default.js', 'uniq.js', 'sortBy.js', 'omit.js', 'capitalize.js', 'pick.js', 'groupBy.js', 'throttle.js', 'debounce.js'];
  const originalKeyModules = keyModules.filter(mod => originalCode.includes(mod));
  const optimizedKeyModules = keyModules.filter(mod => optimizedCode.includes(mod));
  
  console.log(`\nKEY LODASH MODULES:`);
  console.log(`  Original: ${originalKeyModules.length}/11 present: [${originalKeyModules.join(', ')}]`);
  console.log(`  Optimized: ${optimizedKeyModules.length}/11 present: [${optimizedKeyModules.join(', ')}]`);

  console.log('\n' + '='.repeat(80));
  console.log('🎯 COMPARISON WITH EXPECTED RESULTS');
  console.log('='.repeat(80));

  // Compare with Rust test expectations
  const expectedReduction = 99.93;
  const actualReduction = reduction;
  const difference = Math.abs(expectedReduction - actualReduction);

  console.log(`Expected reduction (from Rust test): ${expectedReduction}%`);
  console.log(`Actual reduction (JS replication): ${actualReduction.toFixed(2)}%`);
  console.log(`Difference: ${difference.toFixed(2)}% points`);

  if (difference < 1.0) {
    console.log('✅ PERFECT PARITY - JavaScript matches Rust test results!');
  } else if (difference < 5.0) {
    console.log('✅ CLOSE PARITY - JavaScript nearly matches Rust test results');
  } else if (actualReduction > 90.0) {
    console.log('✅ EXCELLENT OPTIMIZATION - High reduction achieved');
  } else if (actualReduction > 70.0) {
    console.log('📊 GOOD OPTIMIZATION - Decent reduction achieved');
  } else if (actualReduction > 30.0) {
    console.log('⚠️  MODERATE OPTIMIZATION - May indicate issues');
  } else {
    console.log('❌ POOR OPTIMIZATION - Significant discrepancy detected');
  }

  console.log('\n' + '='.repeat(80));
  console.log('🔍 DEBUGGING INFORMATION');
  console.log('='.repeat(80));

  // Additional debugging
  console.log(`Configuration used:`);
  console.log(`  Tree shake exports: ${Object.keys(treeShakeConfig).length}`);
  console.log(`  Used exports: ${allUsedExports.size}`);
  console.log(`  Unused exports: ${Object.keys(treeShakeConfig).length - allUsedExports.size}`);

  // Check if tree shaking is working
  if (originalModules - optimizedModules > 100) {
    console.log('✅ Tree shaking is actively removing modules');
  } else if (originalModules - optimizedModules > 10) {
    console.log('📊 Tree shaking is removing some modules');
  } else {
    console.log('⚠️  Tree shaking may not be working effectively');
  }

  // Check final size
  if (optimizedSize < 5000) {
    console.log('✅ Optimized chunk is very small - excellent compression');
  } else if (optimizedSize < 50000) {
    console.log('📊 Optimized chunk is reasonably small');
  } else {
    console.log('⚠️  Optimized chunk is still quite large');
  }

  console.log('\n' + '='.repeat(80));
  console.log('💡 CONCLUSION');
  console.log('='.repeat(80));

  if (actualReduction > 95.0) {
    console.log('🎉 BREAKTHROUGH: JavaScript test replicates Rust test success!');
    console.log('The tree shaker is working perfectly when called correctly.');
  } else if (actualReduction > 70.0) {
    console.log('📊 GOOD RESULT: Significant optimization achieved');
  } else {
    console.log('🔍 INVESTIGATION NEEDED: Results don\'t match Rust test expectations');
    console.log('This suggests a difference in how the optimizer is being called or configured.');
  }

  console.log('\n✅ Rust parity test completed!');
}

function countLodashModules(code) {
  return (code.match(/node_modules\/\.pnpm\/lodash-es/g) || []).length;
}

// Run the main function
if (require.main === module) {
  main().catch(console.error);
}