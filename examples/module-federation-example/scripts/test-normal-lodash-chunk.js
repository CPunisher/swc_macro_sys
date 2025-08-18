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
 * Enhanced optimization with convergence detection
 */
async function optimizeWithConvergence(optimizer, sourceCode, baseConfig, options = {}) {
  const {
    maxPasses = 10,
    convergenceThreshold = 0.1,
    verbose = true
  } = options;

  let currentCode = sourceCode;
  const passResults = [];

  for (let pass = 1; pass <= maxPasses; pass++) {
    const passStartTime = Date.now();
    const beforeSize = currentCode.length;

    const configStr = JSON.stringify(baseConfig);

    try {
      const optimizedCode = optimizer.optimize(currentCode, configStr);
      const afterSize = optimizedCode.length;
      const passReduction = ((beforeSize - afterSize) / beforeSize) * 100;
      const passTime = Date.now() - passStartTime;

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

      if (passReduction < convergenceThreshold) {
        if (verbose) {
          console.log(`✅ Converged after ${pass} passes (reduction < ${convergenceThreshold}%)`);
        }
        break;
      }

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

async function main() {
  console.log('🧪 TESTING NORMAL LODASH CHUNK WITH VARIOUS CONFIGURATIONS\n');

  // Paths
  const standardChunkPath = path.resolve(__dirname, '../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
  const standardUsagePath = path.resolve(__dirname, '../../test-cases/rspack-annotated-output/share-usage.json');

  if (!fs.existsSync(standardChunkPath) || !fs.existsSync(standardUsagePath)) {
    console.log('❌ Required files not found');
    console.log('Standard chunk:', standardChunkPath);
    console.log('Standard usage:', standardUsagePath);
    process.exit(1);
  }

  // Load optimizer
  const optimizer = await loadOptimizer();

  // Read the normal lodash chunk
  const standardChunk = fs.readFileSync(standardChunkPath, 'utf8');
  console.log(`Original lodash chunk size: ${(standardChunk.length/1024).toFixed(1)}KB (${standardChunk.length} bytes)\n`);

  // Read the normal usage pattern (4 exports: map, filter, VERSION, default)
  const standardUsage = JSON.parse(fs.readFileSync(standardUsagePath, 'utf8'));
  
  let standardUsed, standardUnused;
  if (standardUsage.treeShake && standardUsage.treeShake['lodash-es']) {
    // New dot notation format
    standardUsed = Object.entries(standardUsage.treeShake['lodash-es'])
      .filter(([key, value]) => value === true && key !== 'chunk_characteristics')
      .map(([key]) => key);
    standardUnused = Object.entries(standardUsage.treeShake['lodash-es'])
      .filter(([key, value]) => value === false)
      .map(([key]) => key);
  } else {
    // Old format
    standardUsed = standardUsage.consume_shared_modules['lodash-es'].used_exports;
    standardUnused = standardUsage.consume_shared_modules['lodash-es'].unused_exports;
  }

  console.log('='.repeat(80));
  console.log('📊 CONFIGURATION 1: NORMAL LODASH USAGE PATTERN');
  console.log('='.repeat(80));
  console.log(`Used exports (${standardUsed.length}): [${standardUsed.join(', ')}]`);
  console.log(`Unused exports: ${standardUnused.length}\n`);

  // Create tree-shake config for normal usage
  const normalTreeShakeConfig = {};
  standardUsed.forEach(exportName => {
    normalTreeShakeConfig[exportName] = true;
  });
  standardUnused.forEach(exportName => {
    normalTreeShakeConfig[exportName] = false;
  });

  const normalConfig = {
    treeShake: {
      'lodash-es': normalTreeShakeConfig
    }
  };

  const normalResults = await optimizeWithConvergence(
    optimizer,
    standardChunk,
    normalConfig,
    { maxPasses: 10, convergenceThreshold: 0.1, verbose: true }
  );

  console.log('\n' + '='.repeat(80));
  console.log('📊 CONFIGURATION 2: MODULE FEDERATION USAGE PATTERN');
  console.log('='.repeat(80));

  // Module Federation usage pattern (9 exports)
  const mfUsed = ['uniq', 'sortBy', 'default', 'omit', 'capitalize', 'pick', 'groupBy', 'throttle', 'debounce'];
  console.log(`Used exports (${mfUsed.length}): [${mfUsed.join(', ')}]`);

  const mfTreeShakeConfig = {};
  mfUsed.forEach(exportName => {
    mfTreeShakeConfig[exportName] = true;
  });
  standardUnused.forEach(exportName => {
    if (!mfUsed.includes(exportName)) {
      mfTreeShakeConfig[exportName] = false;
    }
  });

  const mfConfig = {
    treeShake: {
      'lodash-es': mfTreeShakeConfig
    }
  };

  console.log('');
  const mfResults = await optimizeWithConvergence(
    optimizer,
    standardChunk,
    mfConfig,
    { maxPasses: 10, convergenceThreshold: 0.1, verbose: true }
  );

  console.log('\n' + '='.repeat(80));
  console.log('📊 CONFIGURATION 3: MINIMAL USAGE (ONLY DEFAULT)');
  console.log('='.repeat(80));

  const minimalConfig = {
    treeShake: {
      'lodash-es': {
        'default': true,
        // Mark all others as false
        ...Object.fromEntries(standardUnused.map(exp => [exp, false])),
        ...Object.fromEntries(standardUsed.filter(exp => exp !== 'default').map(exp => [exp, false]))
      }
    }
  };

  console.log('Used exports (1): [default]');
  console.log('');
  const minimalResults = await optimizeWithConvergence(
    optimizer,
    standardChunk,
    minimalConfig,
    { maxPasses: 10, convergenceThreshold: 0.1, verbose: true }
  );

  console.log('\n' + '='.repeat(80));
  console.log('📊 CONFIGURATION 4: NO TREE SHAKING (BASELINE)');
  console.log('='.repeat(80));

  const noTreeShakeConfig = {};
  console.log('No tree-shake configuration applied');
  console.log('');
  const baselineResults = await optimizeWithConvergence(
    optimizer,
    standardChunk,
    noTreeShakeConfig,
    { maxPasses: 10, convergenceThreshold: 0.1, verbose: true }
  );

  console.log('\n' + '='.repeat(80));
  console.log('🔍 COMPARATIVE ANALYSIS');
  console.log('='.repeat(80));

  const configs = [
    { name: 'Normal lodash (4 exports)', results: normalResults, exports: standardUsed.length },
    { name: 'Module Federation (9 exports)', results: mfResults, exports: mfUsed.length },
    { name: 'Minimal (1 export)', results: minimalResults, exports: 1 },
    { name: 'No tree shaking', results: baselineResults, exports: 'all' }
  ];

  console.log('CONFIGURATION COMPARISON:');
  configs.forEach(config => {
    console.log(`${config.name}:`);
    console.log(`  Exports used: ${config.exports}`);
    console.log(`  Final reduction: ${config.results.finalReduction.toFixed(1)}%`);
    console.log(`  Final size: ${(standardChunk.length * (100 - config.results.finalReduction) / 100 / 1024).toFixed(1)}KB`);
    console.log(`  Passes used: ${config.results.passResults.length}`);
    console.log(`  Total time: ${config.results.totalTime}ms`);
    console.log(`  Converged: ${config.results.converged ? '✅' : '❌'}`);
    console.log('');
  });

  console.log('KEY INSIGHTS:');
  const normalReduction = normalResults.finalReduction;
  const mfReduction = mfResults.finalReduction;
  const minimalReduction = minimalResults.finalReduction;
  const baselineReduction = baselineResults.finalReduction;

  console.log(`• Normal lodash pattern achieves ${normalReduction.toFixed(1)}% reduction`);
  console.log(`• Module Federation pattern achieves ${mfReduction.toFixed(1)}% reduction`);
  console.log(`• Difference: ${(normalReduction - mfReduction).toFixed(1)}% points (${((mfReduction / normalReduction - 1) * 100).toFixed(1)}% relative impact)`);
  console.log(`• Minimal usage achieves ${minimalReduction.toFixed(1)}% reduction`);
  console.log(`• Baseline (no tree shaking) achieves ${baselineReduction.toFixed(1)}% reduction`);

  if (normalReduction > 90) {
    console.log(`✅ Normal lodash chunk optimizes extremely well (${normalReduction.toFixed(1)}%)`);
  } else if (normalReduction > 70) {
    console.log(`✅ Normal lodash chunk optimizes well (${normalReduction.toFixed(1)}%)`);
  } else {
    console.log(`📊 Normal lodash chunk shows moderate optimization (${normalReduction.toFixed(1)}%)`);
  }

  const exportImpact = (mfReduction - normalReduction) / (mfUsed.length - standardUsed.length);
  console.log(`📊 Each additional export costs approximately ${Math.abs(exportImpact).toFixed(1)}% points in optimization`);

  console.log('\n✅ Normal lodash chunk analysis complete!');
}

// Run the main function
if (require.main === module) {
  main().catch(console.error);
}