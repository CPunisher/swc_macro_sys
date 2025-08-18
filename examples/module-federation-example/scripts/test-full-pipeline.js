#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('🚀 Starting full build, test, optimize, test pipeline...\n');

async function runFullPipeline() {
  try {
    // Step 1: Build both apps
    console.log('📦 Step 1: Building host and remote apps...');
    execSync('pnpm run build', { stdio: 'inherit' });
    console.log('✅ Build completed\n');

    // Step 2: Test before optimization
    console.log('🧪 Step 2: Testing bundles BEFORE optimization...');
    
    // Set up minimal globals for webpack
    global.self = { webpackChunkhost: [] };
    
    const hostPath = path.resolve(__dirname, '../host/dist/main.js');
    const hostLodashPath = path.resolve(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
    const remoteLodashPath = path.resolve(__dirname, '../remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
    
    // Check lodash chunk sizes before optimization
    const hostLodashSize = fs.statSync(hostLodashPath).size;
    const remoteLodashSize = fs.statSync(remoteLodashPath).size;
    
    console.log(`Host lodash chunk: ${(hostLodashSize/1024).toFixed(1)} KB`);
    console.log(`Remote lodash chunk: ${(remoteLodashSize/1024).toFixed(1)} KB`);
    
    // Check for macro annotations
    const hostContent = fs.readFileSync(hostLodashPath, 'utf8');
    const macroCount = (hostContent.match(/@common:if/g) || []).length;
    console.log(`Macro annotations found: ${macroCount}`);
    
    // Test host bundle
    let bootstrapExports = null;
    try {
      const hostMain = require(hostPath);
      console.log('\n✅ Host main.js loaded successfully');
      console.log('   Module type:', typeof hostMain);
      console.log('   Exports:', Object.keys(hostMain));
      
      if (hostMain && hostMain.default && typeof hostMain.default.then === 'function') {
        const bootstrap = await hostMain.default;
        bootstrapExports = Object.keys(bootstrap);
        console.log('   ✅ Dynamic import resolved successfully');
        console.log('   Bootstrap exports:', bootstrapExports);
        
        // Test a function
        if (bootstrap.processItems) {
          const testItems = [
            { name: 'Apple', price: 1.99, category: 'fruit' },
            { name: 'Banana', price: 0.99, category: 'fruit' },
            { name: 'Carrot', price: 1.49, category: 'vegetable' }
          ];
          const result = bootstrap.processItems(testItems);
          console.log('   ✅ processItems() works:', 
            `${result.sorted.length} items sorted,`,
            `${result.categories.length} categories found`);
        }
      }
    } catch (error) {
      console.error('❌ Error testing host:', error.message);
    }
    
    // Step 3: Optimize
    console.log('\n⚡ Step 3: Running optimization...');
    
    // Read and merge share-usage.json files
    const hostUsagePath = path.resolve(__dirname, '../host/dist/share-usage.json');
    const remoteUsagePath = path.resolve(__dirname, '../remote/dist/share-usage.json');
    
    const hostUsage = JSON.parse(fs.readFileSync(hostUsagePath, 'utf8'));
    const remoteUsage = JSON.parse(fs.readFileSync(remoteUsagePath, 'utf8'));
    
    // Handle both old and new formats
    let mergedConfig;
    
    if (hostUsage.treeShake && hostUsage.treeShake['lodash-es']) {
      // Already in new dot notation format, just merge
      mergedConfig = hostUsage;
    } else {
      // Convert old format to new format
      const convertUsageToBoolean = (usage) => {
        const booleanMap = {};
        
        if (usage.consume_shared_modules && usage.consume_shared_modules['lodash-es']) {
          const lodashUsage = usage.consume_shared_modules['lodash-es'];
          
          // Mark used exports as true
          if (lodashUsage.used_exports) {
            lodashUsage.used_exports.forEach(exportName => {
              booleanMap[exportName] = true;
            });
          }
          
          // Mark unused exports as false
          if (lodashUsage.unused_exports) {
            lodashUsage.unused_exports.forEach(exportName => {
              booleanMap[exportName] = false;
            });
          }
        }
        
      return booleanMap;
    };
    
    // Convert and merge usage data
    const hostBooleanMap = convertUsageToBoolean(hostUsage);
    const remoteBooleanMap = convertUsageToBoolean(remoteUsage);
    
    // Merge the boolean maps - if any app uses an export, mark it as true
    const mergedUsage = {};
    const allExports = new Set([...Object.keys(hostBooleanMap), ...Object.keys(remoteBooleanMap)]);
    
    allExports.forEach(exportName => {
      mergedUsage[exportName] = hostBooleanMap[exportName] || remoteBooleanMap[exportName] || false;
    });
    
    const usedExports = Object.entries(mergedUsage).filter(([_, used]) => used).map(([name]) => name);
    console.log(`Merged usage: ${usedExports.length} exports used:`, usedExports.join(', '));
    
    // Create tree-shake configuration
    const treeShakeConfig = {
      treeShake: {
        'lodash-es': mergedUsage
      }
    };
    
    // Load SWC macro optimizer
    const optimizer = await import('../../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js');
    console.log('✅ SWC macro optimizer loaded');
    
    // Backup original chunks
    const hostBackup = hostLodashPath + '.original';
    const remoteBackup = remoteLodashPath + '.original';
    
    if (!fs.existsSync(hostBackup)) {
      fs.copyFileSync(hostLodashPath, hostBackup);
    }
    if (!fs.existsSync(remoteBackup)) {
      fs.copyFileSync(remoteLodashPath, remoteBackup);
    }
    
    // Optimize chunks
    const lodashChunks = [
      { name: 'host', path: hostLodashPath },
      { name: 'remote', path: remoteLodashPath }
    ];
    
    for (const chunk of lodashChunks) {
      const originalCode = fs.readFileSync(chunk.path, 'utf8');
      const originalSize = Buffer.byteLength(originalCode);
      
      const optimizedCode = optimizer.optimize(originalCode, JSON.stringify(treeShakeConfig));
      const optimizedSize = Buffer.byteLength(optimizedCode);
      
      fs.writeFileSync(chunk.path, optimizedCode);
      
      const reduction = ((originalSize - optimizedSize) / originalSize * 100).toFixed(2);
      console.log(`${chunk.name} optimized: ${(originalSize/1024).toFixed(1)}KB → ${(optimizedSize/1024).toFixed(1)}KB (${reduction}% reduction)`);
    }
    
    // Step 4: Test after optimization
    console.log('\n🧪 Step 4: Testing bundles AFTER optimization...');
    
    // Clear require cache
    delete require.cache[hostPath];
    delete require.cache[require.resolve(hostPath)];
    
    // Test host bundle again
    try {
      const hostMainAfter = require(hostPath);
      console.log('\n✅ Host main.js loaded successfully after optimization');
      console.log('   Module type:', typeof hostMainAfter);
      console.log('   Exports:', Object.keys(hostMainAfter));
      
      if (hostMainAfter && hostMainAfter.default && typeof hostMainAfter.default.then === 'function') {
        const bootstrapAfter = await hostMainAfter.default;
        console.log('   ✅ Dynamic import resolved successfully');
        console.log('   Bootstrap exports:', Object.keys(bootstrapAfter));
        
        // Test the same function
        if (bootstrapAfter.processItems) {
          const testItems = [
            { name: 'Apple', price: 1.99, category: 'fruit' },
            { name: 'Banana', price: 0.99, category: 'fruit' },
            { name: 'Carrot', price: 1.49, category: 'vegetable' }
          ];
          const result = bootstrapAfter.processItems(testItems);
          console.log('   ✅ processItems() still works:', 
            `${result.sorted.length} items sorted,`,
            `${result.categories.length} categories found`);
        }
      }
    } catch (error) {
      console.error('❌ Error testing optimized host:', error.message);
      if (error.stack && error.message.includes('__webpack_modules__')) {
        console.log('\n⚠️  This error indicates the optimization may have removed too much.');
        console.log('   The tree shaker should preserve modules used by the exports.');
      }
    }
    
    // Final summary
    console.log('\n📊 Final Summary:');
    console.log('=================');
    
    const hostOptimizedSize = fs.statSync(hostLodashPath).size;
    const remoteOptimizedSize = fs.statSync(remoteLodashPath).size;
    const totalOriginalSize = hostLodashSize + remoteLodashSize;
    const totalOptimizedSize = hostOptimizedSize + remoteOptimizedSize;
    const totalReduction = ((totalOriginalSize - totalOptimizedSize) / totalOriginalSize * 100).toFixed(1);
    
    console.log(`Total: ${(totalOriginalSize/1024).toFixed(1)}KB → ${(totalOptimizedSize/1024).toFixed(1)}KB (${totalReduction}% reduction)`);
    console.log(`Saved: ${((totalOriginalSize - totalOptimizedSize)/1024).toFixed(1)}KB`);
    
    if (totalReduction > 90) {
      console.log('\n⚠️  Very high reduction rate detected. Verify functionality is preserved.');
    }
    
  } catch (error) {
    console.error('\n❌ Pipeline failed:', error.message);
    process.exit(1);
  }
}

// Run the pipeline
runFullPipeline();