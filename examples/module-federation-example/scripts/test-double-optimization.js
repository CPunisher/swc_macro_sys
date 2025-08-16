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
  console.log('🔄 TESTING DOUBLE OPTIMIZATION EFFECTS\n');

  // Paths
  const mfChunkPath = path.resolve(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original');
  const standardChunkPath = path.resolve(__dirname, '../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
  const mfUsagePath = path.resolve(__dirname, '../host/dist/share-usage.json');
  const remoteUsagePath = path.resolve(__dirname, '../remote/dist/share-usage.json');

  if (!fs.existsSync(mfChunkPath) || !fs.existsSync(standardChunkPath)) {
    console.log('❌ Required chunk files not found');
    process.exit(1);
  }

  // Load optimizer
  const optimizer = await loadOptimizer();

  // Read Module Federation usage data
  const hostUsage = JSON.parse(fs.readFileSync(mfUsagePath, 'utf8'));
  const remoteUsage = JSON.parse(fs.readFileSync(remoteUsagePath, 'utf8'));

  let config;
  if (hostUsage.treeShake && hostUsage.treeShake['lodash-es']) {
    // Already in new dot notation format
    config = hostUsage;
  } else {
    // Convert from old format
    const hostUsed = hostUsage.consume_shared_modules['lodash-es'].used_exports;
    const remoteUsed = remoteUsage.consume_shared_modules['lodash-es'].used_exports;
    const mfCombinedUsed = [...new Set([...hostUsed, ...remoteUsed])];
    const mfUnusedExports = hostUsage.consume_shared_modules['lodash-es'].unused_exports;

    // Create tree-shake config using MF's usage pattern
    const treeShakeConfig = {};
    mfCombinedUsed.forEach(exportName => {
      treeShakeConfig[exportName] = true;
    });
    mfUnusedExports.forEach(exportName => {
      if (!mfCombinedUsed.includes(exportName)) {
        treeShakeConfig[exportName] = false;
      }
    });

    config = {
      treeShake: {
        'lodash-es': treeShakeConfig
      }
    };
  }
  const configStr = JSON.stringify(config);

  console.log('='.repeat(80));
  console.log('🏗️  MODULE FEDERATION CHUNK - DOUBLE OPTIMIZATION');
  console.log('='.repeat(80));

  // Test Module Federation chunk
  const mfOriginal = fs.readFileSync(mfChunkPath, 'utf8');
  const mfOriginalSize = mfOriginal.length;

  console.log(`Original MF chunk size: ${(mfOriginalSize/1024).toFixed(1)}KB (${mfOriginalSize} bytes)`);

  // First optimization
  const startTime1 = Date.now();
  const mfFirstPass = optimizer.optimize(mfOriginal, configStr);
  const time1 = Date.now() - startTime1;
  const mfFirstSize = mfFirstPass.length;
  const mfFirstReduction = ((mfOriginalSize - mfFirstSize) / mfOriginalSize) * 100;

  console.log(`After 1st optimization: ${(mfFirstSize/1024).toFixed(1)}KB (${mfFirstSize} bytes)`);
  console.log(`First pass reduction: ${mfFirstReduction.toFixed(1)}% (${mfOriginalSize - mfFirstSize} bytes saved)`);
  console.log(`First pass time: ${time1}ms`);

  // Second optimization
  const startTime2 = Date.now();
  const mfSecondPass = optimizer.optimize(mfFirstPass, configStr);
  const time2 = Date.now() - startTime2;
  const mfSecondSize = mfSecondPass.length;
  const mfSecondReduction = ((mfFirstSize - mfSecondSize) / mfFirstSize) * 100;
  const mfTotalReduction = ((mfOriginalSize - mfSecondSize) / mfOriginalSize) * 100;

  console.log(`After 2nd optimization: ${(mfSecondSize/1024).toFixed(1)}KB (${mfSecondSize} bytes)`);
  console.log(`Second pass reduction: ${mfSecondReduction.toFixed(1)}% (${mfFirstSize - mfSecondSize} bytes saved)`);
  console.log(`Second pass time: ${time2}ms`);
  console.log(`Total reduction: ${mfTotalReduction.toFixed(1)}% (${mfOriginalSize - mfSecondSize} bytes saved)`);

  // Check if third pass would help
  const startTime3 = Date.now();
  const mfThirdPass = optimizer.optimize(mfSecondPass, configStr);
  const time3 = Date.now() - startTime3;
  const mfThirdSize = mfThirdPass.length;
  const mfThirdReduction = ((mfSecondSize - mfThirdSize) / mfSecondSize) * 100;

  console.log(`After 3rd optimization: ${(mfThirdSize/1024).toFixed(1)}KB (${mfThirdSize} bytes)`);
  console.log(`Third pass reduction: ${mfThirdReduction.toFixed(1)}% (${mfSecondSize - mfThirdSize} bytes saved)`);
  console.log(`Third pass time: ${time3}ms`);

  console.log('\n' + '='.repeat(80));
  console.log('📊 STANDARD WEBPACK CHUNK - DOUBLE OPTIMIZATION');
  console.log('='.repeat(80));

  // Test standard webpack chunk
  const standardOriginal = fs.readFileSync(standardChunkPath, 'utf8');
  const standardOriginalSize = standardOriginal.length;

  console.log(`Original standard chunk size: ${(standardOriginalSize/1024).toFixed(1)}KB (${standardOriginalSize} bytes)`);

  // First optimization
  const stdStartTime1 = Date.now();
  const standardFirstPass = optimizer.optimize(standardOriginal, configStr);
  const stdTime1 = Date.now() - stdStartTime1;
  const standardFirstSize = standardFirstPass.length;
  const standardFirstReduction = ((standardOriginalSize - standardFirstSize) / standardOriginalSize) * 100;

  console.log(`After 1st optimization: ${(standardFirstSize/1024).toFixed(1)}KB (${standardFirstSize} bytes)`);
  console.log(`First pass reduction: ${standardFirstReduction.toFixed(1)}% (${standardOriginalSize - standardFirstSize} bytes saved)`);
  console.log(`First pass time: ${stdTime1}ms`);

  // Second optimization
  const stdStartTime2 = Date.now();
  const standardSecondPass = optimizer.optimize(standardFirstPass, configStr);
  const stdTime2 = Date.now() - stdStartTime2;
  const standardSecondSize = standardSecondPass.length;
  const standardSecondReduction = ((standardFirstSize - standardSecondSize) / standardFirstSize) * 100;
  const standardTotalReduction = ((standardOriginalSize - standardSecondSize) / standardOriginalSize) * 100;

  console.log(`After 2nd optimization: ${(standardSecondSize/1024).toFixed(1)}KB (${standardSecondSize} bytes)`);
  console.log(`Second pass reduction: ${standardSecondReduction.toFixed(1)}% (${standardFirstSize - standardSecondSize} bytes saved)`);
  console.log(`Second pass time: ${stdTime2}ms`);
  console.log(`Total reduction: ${standardTotalReduction.toFixed(1)}% (${standardOriginalSize - standardSecondSize} bytes saved)`);

  console.log('\n' + '='.repeat(80));
  console.log('🔍 COMPARISON & ANALYSIS');
  console.log('='.repeat(80));

  console.log('MODULE FEDERATION RESULTS:');
  console.log(`  Single optimization: ${mfFirstReduction.toFixed(1)}%`);
  console.log(`  Double optimization: ${mfTotalReduction.toFixed(1)}%`);
  console.log(`  Additional benefit:  ${(mfTotalReduction - mfFirstReduction).toFixed(1)}% points`);
  console.log(`  Third pass benefit:  ${mfThirdReduction.toFixed(1)}% points`);

  console.log('\nSTANDARD WEBPACK RESULTS:');
  console.log(`  Single optimization: ${standardFirstReduction.toFixed(1)}%`);
  console.log(`  Double optimization: ${standardTotalReduction.toFixed(1)}%`);
  console.log(`  Additional benefit:  ${(standardTotalReduction - standardFirstReduction).toFixed(1)}% points`);

  console.log('\nTIME ANALYSIS:');
  console.log(`  MF first pass:  ${time1}ms`);
  console.log(`  MF second pass: ${time2}ms`);
  console.log(`  MF third pass:  ${time3}ms`);
  console.log(`  Standard first pass:  ${stdTime1}ms`);
  console.log(`  Standard second pass: ${stdTime2}ms`);

  // Determine if second pass is worthwhile
  const mfSecondPassBenefit = mfTotalReduction - mfFirstReduction;
  const standardSecondPassBenefit = standardTotalReduction - standardFirstReduction;

  console.log('\n💡 RECOMMENDATIONS:');
  if (mfSecondPassBenefit > 1.0) {
    console.log(`✅ Module Federation: Second pass is worthwhile (${mfSecondPassBenefit.toFixed(1)}% additional reduction)`);
  } else {
    console.log(`❌ Module Federation: Second pass minimal benefit (${mfSecondPassBenefit.toFixed(1)}% additional reduction)`);
  }

  if (standardSecondPassBenefit > 1.0) {
    console.log(`✅ Standard Webpack: Second pass is worthwhile (${standardSecondPassBenefit.toFixed(1)}% additional reduction)`);
  } else {
    console.log(`❌ Standard Webpack: Second pass minimal benefit (${standardSecondPassBenefit.toFixed(1)}% additional reduction)`);
  }

  if (mfThirdReduction > 0.1) {
    console.log(`⚠️  Module Federation: Third pass shows ${mfThirdReduction.toFixed(1)}% reduction - optimization may not be converging`);
  } else {
    console.log(`✅ Module Federation: Third pass shows convergence (${mfThirdReduction.toFixed(1)}% reduction)`);
  }

  console.log('\n✅ Double optimization analysis complete!');
}

// Run the main function
if (require.main === module) {
  main().catch(console.error);
}