#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Load the SWC macro WASM optimizer
async function loadOptimizer() {
  try {
    const swcMacro = require('swc_macro_wasm');
    return swcMacro;
  } catch (error) {
    console.error('Failed to load SWC macro optimizer:', error.message);
    process.exit(1);
  }
}

// The share-usage format uses dot notation: treeShake[module][export] = true/false

async function testSharedModule(optimizer, moduleName, chunkFile, expectedRemovals, expectedKept) {
    console.log(`\n${'='.repeat(60)}`);
    console.log(`Testing ${moduleName} optimization`);
    console.log('='.repeat(60));
    
    const chunkPath = path.join(__dirname, '../remote/dist', chunkFile);
    const shareUsagePath = path.join(__dirname, '../remote/dist/share-usage.json');
    
    if (!fs.existsSync(chunkPath)) {
        console.error(`Chunk file not found: ${chunkFile}`);
        return;
    }
    
    const chunk = fs.readFileSync(chunkPath, 'utf8');
    const shareUsage = JSON.parse(fs.readFileSync(shareUsagePath, 'utf8'));
    
    console.log(`\nOriginal ${moduleName} chunk size: ${chunk.length.toLocaleString()} bytes`);
    
    // Show what exports are being kept
    const moduleConfig = shareUsage.treeShake?.[moduleName];
    if (!moduleConfig) {
        console.error(`No tree-shake config found for ${moduleName}`);
        return;
    }
    
    const keptExports = Object.entries(moduleConfig)
        .filter(([key, value]) => value === true && key !== 'chunk_characteristics')
        .map(([key]) => key);
    
    console.log(`Kept exports (${keptExports.length}):`, keptExports.join(', '));
    
    try {
        const result = optimizer.optimize(chunk, JSON.stringify(shareUsage));
        const reduction = ((1 - result.length / chunk.length) * 100).toFixed(1);
        
        console.log(`Optimized size: ${result.length.toLocaleString()} bytes`);
        console.log(`Reduction: ${reduction}%`);
        
        // Check if expected modules were removed
        console.log('\nChecking removed modules:');
        expectedRemovals.forEach(module => {
            const exists = result.includes(module);
            console.log(`  ${module}: ${exists ? '❌ Still present' : '✅ Removed'}`);
        });
        
        // Check if expected modules were kept
        console.log('\nChecking kept modules:');
        expectedKept.forEach(module => {
            const exists = result.includes(module);
            console.log(`  ${module}: ${exists ? '✅ Kept' : '❌ Missing!'}`);
        });
        
        // Save optimized version for inspection
        const optimizedPath = chunkPath.replace('.js', '.optimized.js');
        fs.writeFileSync(optimizedPath, result);
        console.log(`\nOptimized chunk saved to: ${path.basename(optimizedPath)}`);
        
    } catch (error) {
        console.error(`Error optimizing ${moduleName}:`, error.message);
    }
}

async function main() {
    const optimizer = await loadOptimizer();
    
    // Test lodash-es optimization
    await testSharedModule(
        optimizer,
        'lodash-es',
        'vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js',
        ['add.js', 'after.js', 'ary.js', 'assign.js', 'at.js'],  // Should be removed
        ['groupBy.js', 'debounce.js', 'throttle.js', 'pick.js', 'omit.js']  // Should be kept
    );
    
    // Test ramda optimization
    await testSharedModule(
        optimizer,
        'ramda',
        'vendors-node_modules_pnpm_ramda_0_31_3_node_modules_ramda_es_index_js.js',
        ['add.js', 'always.js', 'and.js', 'append.js', 'apply.js'],  // Should be removed
        ['compose.js', 'curry.js', 'map.js', 'filter.js', 'reduce.js']  // Should be kept
    );
    
    // Test date-fns optimization
    await testSharedModule(
        optimizer,
        'date-fns',
        'vendors-node_modules_pnpm_date-fns_4_1_0_node_modules_date-fns_index_js.js',
        ['addHours', 'addMinutes', 'addMonths', 'addQuarters', 'addSeconds'],  // Should be removed
        ['format', 'parseISO', 'addDays', 'subDays', 'isAfter']  // Should be kept
    );
    
    // Test 3: Create a minimal test case
    console.log('\n=== Test 3: Minimal test case ===');
    const minimalChunk = `
exports.modules = {
    "./add.js": function(module, exports, __webpack_require__) {
        var helper = __webpack_require__("./helper.js");
        exports.default = function(a, b) { return helper(a + b); };
    },
    "./subtract.js": function(module, exports, __webpack_require__) {
        exports.default = function(a, b) { return a - b; };
    },
    "./helper.js": function(module, exports, __webpack_require__) {
        exports.default = function(x) { return x; };
    }
};`;
    
    const minimalConfig = {
        treeShake: {
            'test-module': {
                'subtract': true  // Only keep subtract
            }
        }
    };
    
    try {
        const result3 = optimizer.optimize(minimalChunk, JSON.stringify(minimalConfig));
        console.log('Original:', minimalChunk.length, 'bytes');
        console.log('Optimized:', result3.length, 'bytes');
        console.log('\nOptimized content:');
        console.log(result3);
    } catch (error) {
        console.error('Error in test 3:', error);
    }
}

main().catch(console.error);