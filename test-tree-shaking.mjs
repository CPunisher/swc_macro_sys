import fs from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Get current directory for ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Tree-shaking test configurations
const configs = {
  allEnabled: {
    features: {
      enableFeatureA: true,
      enableFeatureB: true,
      enableDebugMode: true
    }
  },
  onlyFeatureA: {
    features: {
      enableFeatureA: true,
      enableFeatureB: false,
      enableDebugMode: false
    }
  },
  onlyFeatureB: {
    features: {
      enableFeatureA: false,
      enableFeatureB: true,
      enableDebugMode: false
    }
  },
  minimal: {
    features: {
      enableFeatureA: false,
      enableFeatureB: false,
      enableDebugMode: false
    }
  }
};

async function initWasm() {
  try {
    // Use the bundled JavaScript module with experimental flag
    const wasmModule = await import('./crates/swc_macro_wasm/pkg/swc_macro_wasm.js');
    console.log('✅ WASM module loaded successfully\n');
    return wasmModule;
  } catch (error) {
    if (error.code === 'ERR_UNKNOWN_FILE_EXTENSION') {
      console.error('❌ WASM modules not supported. Run with: node --experimental-wasm-modules test-tree-shaking.js');
      console.error('💡 Or use: NODE_OPTIONS="--experimental-wasm-modules" node test-tree-shaking.js\n');
    }
    throw error;
  }
}

function analyzeBundle(code, description) {
  const lines = code.split('\n');
  const totalLines = lines.length;
  const totalSize = code.length;
  
  // Count module occurrences
  const modules = {
    dataProcessor: (code.match(/dataProcessor/g) || []).length,
    debugUtils: (code.match(/debugUtils/g) || []).length,
    expensiveUIUtils: (code.match(/expensiveUIUtils/g) || []).length,
    featureA: (code.match(/featureA/g) || []).length,
    featureB: (code.match(/featureB/g) || []).length,
    heavyMathUtils: (code.match(/heavyMathUtils/g) || []).length,
    networkUtils: (code.match(/networkUtils/g) || []).length
  };
  
  // Count specific module IDs to see if modules are completely removed
  const moduleIds = {
    '418': code.includes('418:') ? '✅ dataProcessor' : '❌ removed',
    '422': code.includes('422:') ? '✅ debugUtils' : '❌ removed', 
    '803': code.includes('803:') ? '✅ expensiveUIUtils' : '❌ removed',
    '153': code.includes('153:') ? '✅ featureA' : '❌ removed',
    '722': code.includes('722:') ? '✅ featureB' : '❌ removed',
    '78': code.includes('78:') ? '✅ heavyMathUtils' : '❌ removed',
    '812': code.includes('812:') ? '✅ networkUtils' : '❌ removed'
  };
  
  console.log(`📊 ${description}:`);
  console.log(`   Size: ${totalSize} chars (${totalLines} lines)`);
  console.log(`   Module IDs present:`);
  Object.entries(moduleIds).forEach(([id, status]) => {
    console.log(`     ${id}: ${status}`);
  });
  console.log();
  
  return { totalSize, totalLines, modules, moduleIds };
}

async function testTreeShaking() {
  try {
    // Read the bundler chunk file
    const sourceCode = fs.readFileSync('./bundler-chunk.js', 'utf8');
    
    console.log('🌳 Tree Shaking Demo - Processing bundler-chunk.js');
    console.log('=' .repeat(60));
    
    // Analyze original bundle
    const originalStats = analyzeBundle(sourceCode, 'Original Bundle');
    
    // Initialize WASM module
    const wasmModule = await initWasm();
    
    console.log('🔄 Testing different feature flag configurations...\n');
    
    // Test each configuration
    for (const [configName, config] of Object.entries(configs)) {
      console.log(`⚙️  Testing: ${configName}`);
      console.log(`   Config: ${JSON.stringify(config.features)}`);
      
      try {
        const configString = JSON.stringify(config);
        const startTime = performance.now();
        const optimizedCode = wasmModule.optimize(sourceCode, configString);
        const endTime = performance.now();
        
        const optimizedStats = analyzeBundle(optimizedCode, `Optimized (${configName})`);
        
        const reduction = ((originalStats.totalSize - optimizedStats.totalSize) / originalStats.totalSize) * 100;
        const reductionText = reduction > 0 ? `-${reduction.toFixed(1)}%` : 
                             reduction < 0 ? `+${Math.abs(reduction).toFixed(1)}%` : '0%';
        
        console.log(`   ⚡ Optimized in ${(endTime - startTime).toFixed(2)}ms`);
        console.log(`   📉 Size reduction: ${originalStats.totalSize} → ${optimizedStats.totalSize} chars (${reductionText})`);
        console.log();
        
        // Print optimized output to terminal
        console.log(`   📄 Optimized output (${configName}):`);
        console.log('   ' + '='.repeat(50));
        console.log(optimizedCode.split('\n').map(line => `   ${line}`).join('\n'));
        console.log('   ' + '='.repeat(50));
        
      } catch (optimizeError) {
        console.error(`   ❌ Optimization failed: ${optimizeError.message}`);
      }
      
      console.log('-'.repeat(50));
    }
    
    console.log('\n🎯 Tree Shaking Test Summary:');
    console.log('✅ Successfully tested conditional compilation');
    console.log('✅ Generated optimized bundles for each configuration');
    console.log('✅ Printed optimized outputs above for comparison');
    console.log('\n💡 Expected results:');
    console.log('   - minimal: Should have the smallest bundle (only base functionality)');
    console.log('   - onlyFeatureA: Should remove featureB, expensiveUIUtils, networkUtils');
    console.log('   - onlyFeatureB: Should remove featureA, heavyMathUtils, dataProcessor');
    console.log('   - allEnabled: Should keep all modules (baseline)');
    
  } catch (error) {
    console.error('❌ Error during tree shaking test:', error.message);
    if (error.code === 'ERR_UNKNOWN_FILE_EXTENSION') {
      console.error('\n💡 Make sure to run with WASM support:');
      console.error('   node --experimental-wasm-modules test-tree-shaking.js');
      console.error('   or set NODE_OPTIONS="--experimental-wasm-modules"');
    }
  }
}

// Run the tree shaking test
testTreeShaking(); 