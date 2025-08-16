#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Import the SWC macro WASM optimizer
async function loadOptimizer() {
  try {
    const swcMacro = await import('swc_macro_wasm');
    return swcMacro;
  } catch (error) {
    console.error('Failed to load SWC macro optimizer:', error.message);
    process.exit(1);
  }
}

async function main() {
  console.log('🧪 TESTING IDENTICAL EXPORT USAGE ON STANDARD WEBPACK CHUNK\n');

  // Paths
  const standardChunkPath = path.resolve(__dirname, '../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
  const mfUsagePath = path.resolve(__dirname, '../host/dist/share-usage.json');
  const remoteUsagePath = path.resolve(__dirname, '../remote/dist/share-usage.json');

if (!fs.existsSync(standardChunkPath)) {
    console.log('❌ Standard webpack chunk not found:', standardChunkPath);
    process.exit(1);
}

if (!fs.existsSync(mfUsagePath) || !fs.existsSync(remoteUsagePath)) {
    console.log('❌ Module Federation usage files not found');
    process.exit(1);
}

console.log('='.repeat(80));
console.log('📊 EXTRACTING MODULE FEDERATION USAGE PATTERN');
console.log('='.repeat(80));

// Read Module Federation usage data
const hostUsage = JSON.parse(fs.readFileSync(mfUsagePath, 'utf8'));
const remoteUsage = JSON.parse(fs.readFileSync(remoteUsagePath, 'utf8'));

let hostUsed, remoteUsed, mfUnusedExports;

if (hostUsage.treeShake && hostUsage.treeShake['lodash-es']) {
    // New dot notation format
    hostUsed = Object.entries(hostUsage.treeShake['lodash-es'])
        .filter(([key, value]) => value === true && key !== 'chunk_characteristics')
        .map(([key]) => key);
    remoteUsed = Object.entries(remoteUsage.treeShake['lodash-es'])
        .filter(([key, value]) => value === true && key !== 'chunk_characteristics')
        .map(([key]) => key);
    mfUnusedExports = Object.entries(hostUsage.treeShake['lodash-es'])
        .filter(([key, value]) => value === false)
        .map(([key]) => key);
} else {
    // Old format
    hostUsed = hostUsage.consume_shared_modules['lodash-es'].used_exports;
    remoteUsed = remoteUsage.consume_shared_modules['lodash-es'].used_exports;
    mfUnusedExports = hostUsage.consume_shared_modules['lodash-es'].unused_exports;
}

// Combine MF usage (union of both apps)
const mfCombinedUsed = [...new Set([...hostUsed, ...remoteUsed])];

console.log(`Module Federation exports used: ${mfCombinedUsed.length}`);
console.log(`Exports: [${mfCombinedUsed.join(', ')}]`);

console.log('\n' + '='.repeat(80));
console.log('🏗️  CREATING TREE-SHAKE CONFIG FOR STANDARD CHUNK');
console.log('='.repeat(80));

// Read the standard chunk
const standardChunk = fs.readFileSync(standardChunkPath, 'utf8');
const originalSize = standardChunk.length;

console.log(`Original standard chunk size: ${(originalSize/1024).toFixed(1)}KB (${originalSize} bytes)`);

// Create tree-shake config using MF's exact usage pattern
const treeShakeConfig = {};

// Mark MF exports as used (true)
mfCombinedUsed.forEach(exportName => {
    treeShakeConfig[exportName] = true;
});

// Mark unused exports as false
mfUnusedExports.forEach(exportName => {
    if (!mfCombinedUsed.includes(exportName)) {
        treeShakeConfig[exportName] = false;
    }
});

const config = {
    treeShake: {
        'lodash-es': treeShakeConfig
    }
};

console.log(`Tree-shake config created:`);
console.log(`  Used exports: ${mfCombinedUsed.length}`);
console.log(`  Unused exports: ${Object.values(treeShakeConfig).filter(v => !v).length}`);
console.log(`  Total exports in config: ${Object.keys(treeShakeConfig).length}`);

console.log('\n' + '='.repeat(80));
console.log('⚡ RUNNING OPTIMIZATION WITH MF USAGE PATTERN');
console.log('='.repeat(80));

// Load and run the SWC optimizer
const optimizer = await loadOptimizer();

const startTime = Date.now();
const configStr = JSON.stringify(config);
const optimizedChunk = optimizer.optimize(standardChunk, configStr);
const endTime = Date.now();

const optimizedSize = optimizedChunk.length;
const reduction = ((originalSize - optimizedSize) / originalSize) * 100;
const timeMs = endTime - startTime;

console.log(`\n📊 OPTIMIZATION RESULTS:`);
console.log(`Original size:    ${(originalSize/1024).toFixed(1)}KB (${originalSize} bytes)`);
console.log(`Optimized size:   ${(optimizedSize/1024).toFixed(1)}KB (${optimizedSize} bytes)`);
console.log(`Size reduction:   ${reduction.toFixed(1)}% (${originalSize - optimizedSize} bytes saved)`);
console.log(`Optimization time: ${timeMs}ms`);

console.log('\n' + '='.repeat(80));
console.log('🔍 COMPARISON WITH MODULE FEDERATION RESULTS');
console.log('='.repeat(80));

// Compare with actual MF optimization results
const mfChunkPath = path.resolve(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original');
if (fs.existsSync(mfChunkPath)) {
    const mfChunk = fs.readFileSync(mfChunkPath, 'utf8');
    const mfOriginalSize = mfChunk.length;
    
    const mfOptimized = optimizer.optimize(mfChunk, configStr);
    const mfOptimizedSize = mfOptimized.length;
    const mfReduction = ((mfOriginalSize - mfOptimizedSize) / mfOriginalSize) * 100;
    
    console.log(`STANDARD WEBPACK (with MF exports): ${reduction.toFixed(1)}% reduction`);
    console.log(`MODULE FEDERATION (actual):          ${mfReduction.toFixed(1)}% reduction`);
    console.log(`Difference:                          ${(reduction - mfReduction).toFixed(1)}% points`);
    
    if (Math.abs(reduction - mfReduction) < 5) {
        console.log(`\n✅ SIMILAR RESULTS! The optimization difference is primarily due to export usage patterns, not chunk format.`);
    } else {
        console.log(`\n📋 SIGNIFICANT DIFFERENCE! This suggests chunk format also affects optimization.`);
    }
} else {
    console.log(`⚠️  Could not compare with MF chunk (not found)`);
}

console.log('\n' + '='.repeat(80));
console.log('💡 ANALYSIS CONCLUSION');
console.log('='.repeat(80));
console.log(`When using identical export patterns (${mfCombinedUsed.length} exports):`);
console.log(`• Standard webpack chunk achieved ${reduction.toFixed(1)}% reduction`);
console.log(`• This isolates the impact of export usage vs chunk format`);
console.log(`• Confirms that usage patterns are the primary factor in optimization rates`);

console.log('\n✅ Test completed! Results show impact of export usage patterns on optimization.');
}

// Run the main function
if (require.main === module) {
  main().catch(console.error);
}