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

/**
 * Enhanced optimization with convergence detection and optimal configuration
 */
async function optimizeWithConvergence(optimizer, sourceCode, baseConfig, options = {}) {
  const {
    maxPasses = 10,
    convergenceThreshold = 0.1, // Stop if reduction < 0.1%
    verbose = true
  } = options;

  let currentCode = sourceCode;
  let totalReduction = 0;
  const passResults = [];

  for (let pass = 1; pass <= maxPasses; pass++) {
    const passStartTime = Date.now();
    const beforeSize = currentCode.length;

    // Create enhanced configuration for this pass
    const enhancedConfig = createEnhancedConfig(baseConfig, pass);
    const configStr = JSON.stringify(enhancedConfig);

    try {
      const optimizedCode = optimizer.optimize(currentCode, configStr);
      const afterSize = optimizedCode.length;
      const passReduction = ((beforeSize - afterSize) / beforeSize) * 100;
      const passTime = Date.now() - passStartTime;

      // Calculate cumulative reduction
      const originalSize = sourceCode.length;
      const cumulativeReduction = ((originalSize - afterSize) / originalSize) * 100;

      const passResult = {
        pass,
        beforeSize,
        afterSize,
        passReduction,
        cumulativeReduction,
        passTime,
        bytesSaved: beforeSize - afterSize
      };

      passResults.push(passResult);

      if (verbose) {
        console.log(`Pass ${pass}: ${(beforeSize/1024).toFixed(1)}KB → ${(afterSize/1024).toFixed(1)}KB (${passReduction.toFixed(1)}% reduction, ${passTime}ms)`);
      }

      // Check convergence
      if (passReduction < convergenceThreshold) {
        if (verbose) {
          console.log(`✅ Converged after ${pass} passes (reduction < ${convergenceThreshold}%)`);
        }
        break;
      }

      // Check if no change occurred
      if (afterSize === beforeSize) {
        if (verbose) {
          console.log(`✅ No further optimization possible after ${pass} passes`);
        }
        break;
      }

      currentCode = optimizedCode;
    } catch (error) {
      console.error(`❌ Pass ${pass} failed:`, error.message);
      break;
    }
  }

  const finalReduction = ((sourceCode.length - currentCode.length) / sourceCode.length) * 100;
  const totalTime = passResults.reduce((sum, r) => sum + r.passTime, 0);

  return {
    optimizedCode: currentCode,
    finalReduction,
    totalTime,
    passResults,
    converged: passResults.length < maxPasses
  };
}

/**
 * Create enhanced configuration that exposes more optimization parameters
 */
function createEnhancedConfig(baseConfig, passNumber) {
  // Base tree-shake configuration
  const enhancedConfig = {
    ...baseConfig,
    
    // Add optimization hints that might be used by SWC internals
    optimization: {
      pass: passNumber,
      aggressive: true,
      convergence_mode: true,
      max_iterations: 10, // Higher than default 5
      remove_unused_imports: true,
      eliminate_dead_code: true
    },
    
    // Feature flags to disable optional functionality
    features: {
      // Disable all optional features for maximum tree shaking
      enableDebugging: false,
      enableLogging: false,
      enableDevMode: false,
      enableTestMode: false,
      enableAnalytics: false,
      enableMetrics: false,
      enableTracing: false,
      enableProfiling: false,
      enableWebpackHMR: false,
      enableSourceMaps: false,
      enableComments: false,
      enableLazyLoading: false,
      enableCodeSplitting: false,
      enableDynamicImports: false,
      enablePolyfills: false,
      enableBabelTransforms: false,
      enableMinification: false // Let SWC handle this separately
    },
    
    // Build configuration for production optimization
    build: {
      target: "production",
      mode: "aggressive",
      minify: false, // Keep false to avoid conflicts
      removeDeadCode: true,
      treeShake: true,
      removeUnusedImports: true,
      eliminateDeadBranches: true
    }
  };

  return enhancedConfig;
}

/**
 * Analyze optimization results and provide insights
 */
function analyzeOptimizationResults(results) {
  const { passResults, finalReduction, totalTime } = results;
  
  console.log('\n' + '='.repeat(80));
  console.log('📊 ENHANCED OPTIMIZATION ANALYSIS');
  console.log('='.repeat(80));

  console.log('\nPASS-BY-PASS BREAKDOWN:');
  passResults.forEach(result => {
    const efficiency = result.bytesSaved / result.passTime; // bytes per ms
    console.log(`  Pass ${result.pass}: ${result.passReduction.toFixed(1)}% (${result.bytesSaved} bytes in ${result.passTime}ms, ${efficiency.toFixed(1)} bytes/ms)`);
  });

  console.log('\nOPTIMIZATION INSIGHTS:');
  const firstPassReduction = passResults[0]?.passReduction || 0;
  const subsequentReduction = finalReduction - firstPassReduction;
  
  console.log(`  First pass reduction: ${firstPassReduction.toFixed(1)}%`);
  console.log(`  Subsequent passes:    ${subsequentReduction.toFixed(1)}%`);
  console.log(`  Total reduction:      ${finalReduction.toFixed(1)}%`);
  console.log(`  Total time:           ${totalTime}ms`);
  console.log(`  Passes used:          ${passResults.length}`);
  console.log(`  Converged:            ${results.converged ? '✅' : '❌'}`);

  const avgPassTime = totalTime / passResults.length;
  const totalBytesSaved = passResults.reduce((sum, r) => sum + r.bytesSaved, 0);
  
  console.log(`  Average pass time:    ${avgPassTime.toFixed(0)}ms`);
  console.log(`  Total bytes saved:    ${totalBytesSaved} bytes`);
  console.log(`  Efficiency:           ${(totalBytesSaved / totalTime).toFixed(1)} bytes/ms`);

  // Efficiency analysis
  const highEfficiencyPasses = passResults.filter(r => r.passReduction > 5);
  const lowEfficiencyPasses = passResults.filter(r => r.passReduction < 1);
  
  if (highEfficiencyPasses.length > 0) {
    console.log(`  High-efficiency passes: ${highEfficiencyPasses.map(r => r.pass).join(', ')}`);
  }
  if (lowEfficiencyPasses.length > 0) {
    console.log(`  Low-efficiency passes:  ${lowEfficiencyPasses.map(r => r.pass).join(', ')}`);
  }

  return {
    efficiency: totalBytesSaved / totalTime,
    convergenceQuality: results.converged ? 'Good' : 'Poor',
    recommendedPasses: Math.max(1, passResults.findIndex(r => r.passReduction < 1) + 1)
  };
}

async function main() {
  console.log('🚀 ENHANCED SWC OPTIMIZATION WITH CONVERGENCE DETECTION\n');

  // Paths
  const standardChunkPath = path.resolve(__dirname, '../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
  const mfChunkPath = path.resolve(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original');
  const mfUsagePath = path.resolve(__dirname, '../host/dist/share-usage.json');
  const remoteUsagePath = path.resolve(__dirname, '../remote/dist/share-usage.json');

  if (!fs.existsSync(standardChunkPath) || !fs.existsSync(mfChunkPath)) {
    console.log('❌ Required chunk files not found');
    process.exit(1);
  }

  // Load optimizer
  const optimizer = await loadOptimizer();

  // Read Module Federation usage data
  const hostUsage = JSON.parse(fs.readFileSync(mfUsagePath, 'utf8'));
  const remoteUsage = JSON.parse(fs.readFileSync(remoteUsagePath, 'utf8'));

  // Check if already in new format
  let baseConfig;
  if (hostUsage.treeShake && hostUsage.treeShake['lodash-es']) {
    // Already in new dot notation format
    baseConfig = hostUsage;
  } else {
    // Convert from old format
    const hostUsed = hostUsage.consume_shared_modules['lodash-es'].used_exports;
    const remoteUsed = remoteUsage.consume_shared_modules['lodash-es'].used_exports;
    const mfCombinedUsed = [...new Set([...hostUsed, ...remoteUsed])];
    const mfUnusedExports = hostUsage.consume_shared_modules['lodash-es'].unused_exports;

    // Create base tree-shake config
    const baseTreeShakeConfig = {};
    mfCombinedUsed.forEach(exportName => {
      baseTreeShakeConfig[exportName] = true;
    });
    mfUnusedExports.forEach(exportName => {
      if (!mfCombinedUsed.includes(exportName)) {
        baseTreeShakeConfig[exportName] = false;
      }
    });

    baseConfig = {
      treeShake: {
        'lodash-es': baseTreeShakeConfig
      }
    };
  }

  console.log('='.repeat(80));
  console.log('🔬 TESTING ENHANCED OPTIMIZATION ON STANDARD WEBPACK CHUNK');
  console.log('='.repeat(80));

  const standardChunk = fs.readFileSync(standardChunkPath, 'utf8');
  console.log(`Original size: ${(standardChunk.length/1024).toFixed(1)}KB (${standardChunk.length} bytes)\n`);

  const standardResults = await optimizeWithConvergence(
    optimizer,
    standardChunk,
    baseConfig,
    { maxPasses: 10, convergenceThreshold: 0.1, verbose: true }
  );

  analyzeOptimizationResults(standardResults);

  console.log('\n' + '='.repeat(80));
  console.log('🔬 TESTING ENHANCED OPTIMIZATION ON MODULE FEDERATION CHUNK');
  console.log('='.repeat(80));

  const mfChunk = fs.readFileSync(mfChunkPath, 'utf8');
  console.log(`Original size: ${(mfChunk.length/1024).toFixed(1)}KB (${mfChunk.length} bytes)\n`);

  const mfResults = await optimizeWithConvergence(
    optimizer,
    mfChunk,
    baseConfig,
    { maxPasses: 10, convergenceThreshold: 0.1, verbose: true }
  );

  analyzeOptimizationResults(mfResults);

  console.log('\n' + '='.repeat(80));
  console.log('🔍 COMPARATIVE ANALYSIS');
  console.log('='.repeat(80));

  console.log('STANDARD WEBPACK:');
  console.log(`  Final reduction: ${standardResults.finalReduction.toFixed(1)}%`);
  console.log(`  Passes used: ${standardResults.passResults.length}`);
  console.log(`  Converged: ${standardResults.converged ? '✅' : '❌'}`);

  console.log('\nMODULE FEDERATION:');
  console.log(`  Final reduction: ${mfResults.finalReduction.toFixed(1)}%`);
  console.log(`  Passes used: ${mfResults.passResults.length}`);
  console.log(`  Converged: ${mfResults.converged ? '✅' : '❌'}`);

  const improvementStandard = standardResults.finalReduction - 71.1; // vs simple single pass
  const improvementMF = mfResults.finalReduction - 39.8; // vs simple single pass

  console.log('\nIMPROVEMENT vs SINGLE PASS:');
  console.log(`  Standard webpack: +${improvementStandard.toFixed(1)}% points`);
  console.log(`  Module Federation: +${improvementMF.toFixed(1)}% points`);

  if (improvementMF > 5) {
    console.log('\n✅ Enhanced optimization provides significant improvement for Module Federation!');
  } else if (improvementMF > 1) {
    console.log('\n📊 Enhanced optimization provides moderate improvement for Module Federation.');
  } else {
    console.log('\n📋 Enhanced optimization provides minimal improvement - single pass is sufficient.');
  }

  console.log('\n✅ Enhanced optimization analysis complete!');
}

// Run the main function
if (require.main === module) {
  main().catch(console.error);
}

module.exports = { optimizeWithConvergence, createEnhancedConfig, analyzeOptimizationResults };