#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Test the exact same optimization pipeline as the build scripts
async function runJavaScriptParityTest() {
    console.log('\n=== JAVASCRIPT PARITY TEST ===');
    console.log('Testing same optimization pipeline as build scripts vs Rust test results\n');

    // Import the SWC macro WASM optimizer (requires --experimental-wasm-modules)
    let swcMacro;
    try {
        swcMacro = await import('../pkg/swc_macro_wasm.js');
        console.log('✅ SWC macro WASM module loaded successfully');
    } catch (error) {
        console.error('❌ Failed to load SWC macro WASM module:', error.message);
        console.log('Run with: node --experimental-wasm-modules crates/swc_macro_wasm/tests/javascript_parity_test.cjs');
        process.exit(1);
    }

    // Use the same fixtures as the Rust tests
    const fixturesDir = path.join(__dirname, 'fixtures');
    const mfChunkPath = path.join(fixturesDir, 'module_federation_lodash_chunk.js');
    const hostUsagePath = path.join(fixturesDir, 'module_federation_usage.json');
    const remoteUsagePath = path.join(fixturesDir, 'module_federation_remote_usage.json');

    // Check if fixtures exist
    if (!fs.existsSync(mfChunkPath)) {
        console.log('❌ Module Federation chunk fixture not found:', mfChunkPath);
        console.log('Run Rust tests first to generate fixtures');
        process.exit(1);
    }

    console.log('📁 Loading test fixtures...');
    const originalCode = fs.readFileSync(mfChunkPath, 'utf8');
    const originalSize = originalCode.length;

    console.log(`Original chunk size: ${originalSize} bytes (${(originalSize/1024).toFixed(2)} KB)`);

    // Load usage data from fixtures in current optimizer format with chunk_characteristics
    const hostUsage = JSON.parse(fs.readFileSync(hostUsagePath, 'utf8'));
    const remoteUsage = JSON.parse(fs.readFileSync(remoteUsagePath, 'utf8'));

    const hostCfg = hostUsage.treeShake['lodash-es'] || {};
    const remoteCfg = remoteUsage.treeShake['lodash-es'] || {};
    const hostUsed = Object.entries(hostCfg).filter(([k,v]) => k !== 'chunk_characteristics' && v === true).map(([k]) => k);
    const remoteUsed = Object.entries(remoteCfg).filter(([k,v]) => k !== 'chunk_characteristics' && v === true).map(([k]) => k);

    console.log(`Host app uses ${hostUsed.length} lodash exports:`, hostUsed);
    console.log(`Remote app uses ${remoteUsed.length} lodash exports:`, remoteUsed);

    // Merge used exports (union) exactly like the scripts
    const allUsedExports = new Set([...hostUsed, ...remoteUsed]);
    console.log(`Combined used exports: ${allUsedExports.size} total`);

    // Create tree shake config exactly like the scripts
    const treeShakeConfig = {};
    
    // Mark used exports as true, unused as false
    for (const exportName of allUsedExports) {
        treeShakeConfig[exportName] = true;
    }
    // Mark everything else we see in host config as false (excluding chunk_characteristics)
    for (const [exportName, val] of Object.entries(hostCfg)) {
        if (exportName === 'chunk_characteristics') continue;
        if (!allUsedExports.has(exportName) && val === false) treeShakeConfig[exportName] = false;
    }

    console.log(`Tree shake config includes ${Object.keys(treeShakeConfig).length} exports (${allUsedExports.size} used, ${Object.keys(treeShakeConfig).length - allUsedExports.size} unused)`);

    // === TEST 1: Single pass optimization (like Rust test) ===
    console.log('\n' + '='.repeat(60));
    console.log('🧪 TEST 1: Single pass optimization (replicating Rust test)');
    console.log('='.repeat(60));

    const characteristics = hostCfg.chunk_characteristics || remoteCfg.chunk_characteristics;
    if (!characteristics) {
        console.error('❌ Missing chunk_characteristics in fixtures. Add treeShake["lodash-es"].chunk_characteristics.');
        process.exit(1);
    }
    const config1 = {
        treeShake: {
            'lodash-es': {
                ...treeShakeConfig,
                chunk_characteristics: characteristics
            }
        }
    };

    const config1Str = JSON.stringify(config1);
    const optimized1 = swcMacro.optimize(originalCode, config1Str);
    const reduction1 = ((originalSize - optimized1.length) / originalSize) * 100;

    console.log(`Single pass result: ${reduction1.toFixed(2)}% reduction → ${(optimized1.length/1024).toFixed(2)} KB`);

    // === TEST 2: Multi-pass optimization (like build scripts) ===
    console.log('\n' + '='.repeat(60));
    console.log('🔄 TEST 2: Multi-pass optimization (replicating build scripts)');
    console.log('='.repeat(60));

    let currentCode = originalCode;
    let pass = 1;
    const maxPasses = 10;
    const convergenceThreshold = 0.1;

    while (pass <= maxPasses) {
        const beforeSize = currentCode.length;
        const passResult = swcMacro.optimize(currentCode, config1Str);
        const afterSize = passResult.length;
        
        const passReduction = ((beforeSize - afterSize) / beforeSize) * 100;
        const cumulativeReduction = ((originalSize - afterSize) / originalSize) * 100;
        
        console.log(`Pass ${pass}: ${passReduction.toFixed(1)}% reduction → ${(afterSize/1024).toFixed(2)} KB (cumulative: ${cumulativeReduction.toFixed(1)}%)`);
        
        if (passReduction < convergenceThreshold) {
            console.log(`✅ Converged after ${pass} passes (reduction < ${convergenceThreshold}%)`);
            break;
        }
        
        if (afterSize === beforeSize) {
            console.log(`✅ No further optimization possible after ${pass} passes`);
            break;
        }
        
        currentCode = passResult;
        pass++;
    }

    const finalReduction = ((originalSize - currentCode.length) / originalSize) * 100;

    // === TEST 3: Simulate real-world script configuration ===
    console.log('\n' + '='.repeat(60));
    console.log('🌍 TEST 3: Real-world script configuration');
    console.log('='.repeat(60));

    // This simulates the config that actually gets 39.8% in real scripts
    const realWorldConfig = {
        treeShake: {
            'lodash-es': {
                'uniq': true,
                'sortBy': true,
                'default': true,
                'omit': true,
                'capitalize': true,
                'pick': true,
                'groupBy': true,
                'throttle': true,
                'debounce': true,
                chunk_characteristics: characteristics
            }
        }
    };

    const realWorldResult = swcMacro.optimize(originalCode, JSON.stringify(realWorldConfig));
    const realWorldReduction = ((originalSize - realWorldResult.length) / originalSize) * 100;

    console.log(`Real-world config result: ${realWorldReduction.toFixed(2)}% reduction → ${(realWorldResult.length/1024).toFixed(2)} KB`);

    // === TEST 4: Progressive export testing ===
    console.log('\n' + '='.repeat(60));
    console.log('🔍 TEST 4: Progressive export testing');
    console.log('='.repeat(60));

    const progressiveConfigs = [
        { name: 'Empty config', config: {} },
        { name: 'Default only', config: { treeShake: { 'lodash-es': { 'default': true } } } },
        { name: 'Two exports', config: { treeShake: { 'lodash-es': { 'map': true, 'filter': true } } } },
        { name: 'All used exports', config: config1 }
    ];

    for (const { name, config } of progressiveConfigs) {
        const configStr = JSON.stringify(config);
        const cfg = name === 'Empty config' ? { treeShake: { 'lodash-es': { chunk_characteristics: characteristics } } } : JSON.parse(configStr);
        if (!cfg.treeShake['lodash-es'].chunk_characteristics) cfg.treeShake['lodash-es'].chunk_characteristics = characteristics;
        const result = swcMacro.optimize(originalCode, JSON.stringify(cfg));
        const reduction = ((originalSize - result.length) / originalSize) * 100;
        console.log(`${name}: ${reduction.toFixed(1)}% reduction → ${(result.length/1024).toFixed(2)} KB`);
    }

    // === ANALYSIS ===
    console.log('\n' + '='.repeat(60));
    console.log('📊 ANALYSIS');
    console.log('='.repeat(60));

    console.log(`Single pass optimization: ${reduction1.toFixed(1)}% reduction`);
    console.log(`Multi-pass optimization: ${finalReduction.toFixed(1)}% reduction`);
    console.log(`Real-world simulation: ${realWorldReduction.toFixed(1)}% reduction`);

    // Compare with expected Rust test results
    const expectedRustReduction = 99.93;
    const actualSinglePass = reduction1;
    const actualMultiPass = finalReduction;

    console.log(`\nExpected (Rust test): ${expectedRustReduction}%`);
    console.log(`Actual (JS single pass): ${actualSinglePass.toFixed(2)}%`);
    console.log(`Actual (JS multi-pass): ${actualMultiPass.toFixed(2)}%`);

    const singlePassDiff = Math.abs(expectedRustReduction - actualSinglePass);
    const multiPassDiff = Math.abs(expectedRustReduction - actualMultiPass);

    console.log(`\nSingle pass difference: ${singlePassDiff.toFixed(2)}% points`);
    console.log(`Multi-pass difference: ${multiPassDiff.toFixed(2)}% points`);

    // === DEBUGGING INFO ===
    console.log('\n' + '='.repeat(60));
    console.log('🔍 DEBUGGING INFO');
    console.log('='.repeat(60));

    // Check chunk structure
    console.log('CHUNK STRUCTURE:');
    console.log(`  Original contains 'exports.modules': ${originalCode.includes('exports.modules')}`);
    console.log(`  Optimized contains 'exports.modules': ${optimized1.includes('exports.modules')}`);
    console.log(`  Original contains '__webpack_require__': ${originalCode.includes('__webpack_require__')}`);

    // Module count analysis
    const originalModules = countLodashModules(originalCode);
    const optimizedModules = countLodashModules(optimized1);
    console.log(`\nMODULE ANALYSIS:`);
    console.log(`  Original lodash modules: ${originalModules}`);
    console.log(`  Optimized lodash modules: ${optimizedModules}`);
    console.log(`  Modules removed: ${originalModules - optimizedModules}`);

    // Key lodash modules check
    const keyModules = ['map.js', 'filter.js', 'default.js', 'uniq.js', 'sortBy.js', 'omit.js', 'capitalize.js', 'pick.js', 'groupBy.js', 'throttle.js', 'debounce.js'];
    const originalKeyModules = keyModules.filter(mod => originalCode.includes(mod));
    const optimizedKeyModules = keyModules.filter(mod => optimized1.includes(mod));
    
    console.log(`\nKEY LODASH MODULES:`);
    console.log(`  Original: ${originalKeyModules.length}/11 present: [${originalKeyModules.join(', ')}]`);
    console.log(`  Optimized: ${optimizedKeyModules.length}/11 present: [${optimizedKeyModules.join(', ')}]`);

    // === CONCLUSION ===
    console.log('\n' + '='.repeat(60));
    console.log('🎯 CONCLUSION');
    console.log('='.repeat(60));

    if (singlePassDiff < 1.0) {
        console.log('✅ PERFECT PARITY: JavaScript single pass matches Rust test exactly!');
    } else if (multiPassDiff < 1.0) {
        console.log('✅ PERFECT PARITY: JavaScript multi-pass matches Rust test exactly!');
    } else if (actualSinglePass > 90.0) {
        console.log('✅ EXCELLENT RESULT: JavaScript achieves high optimization similar to Rust');
    } else if (actualSinglePass > 70.0) {
        console.log('📊 GOOD RESULT: JavaScript achieves decent optimization');
    } else {
        console.log('❌ POOR RESULT: JavaScript optimization significantly differs from Rust test');
        console.log('This suggests the issue is in how the optimizer is called or configured in real scripts');
    }

    // Identify potential issues
    if (realWorldReduction < 50.0 && actualSinglePass > 90.0) {
        console.log('\n⚠️  ISSUE IDENTIFIED: Real-world config achieves much lower optimization');
        console.log('The discrepancy is likely due to different export configurations between tests and real scripts');
    }

    if (multiPassDiff > 10.0 && singlePassDiff < 5.0) {
        console.log('\n⚠️  ISSUE IDENTIFIED: Multi-pass optimization degrades results');
        console.log('This suggests the optimizer has convergence issues when run multiple times');
    }

    console.log('\n✅ JavaScript parity test completed!');
    console.log('This test helps identify why real-world results (39.8%) differ from test results (99.93%)');
}

function countLodashModules(code) {
    return (code.match(/node_modules\/\.pnpm\/lodash-es/g) || []).length;
}

// Run the test
if (require.main === module) {
    runJavaScriptParityTest().catch(console.error);
} else {
    module.exports = { runJavaScriptParityTest };
}