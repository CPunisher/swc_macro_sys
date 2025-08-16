#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import the SWC macro WASM optimizer
async function loadOptimizer() {
  try {
    // Load directly from the pkg directory
    const wasmPath = path.resolve(__dirname, '../../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js');
    const swcMacro = await import(wasmPath);
    console.log('Available functions:', Object.keys(swcMacro).filter(k => typeof swcMacro[k] === 'function'));
    return swcMacro;
  } catch (error) {
    console.error('Failed to load SWC macro optimizer:', error.message);
    console.log('Please ensure:');
    console.log('1. WASM package is built: pnpm build:wasm (or pnpm build)');
    console.log('2. Script is run with: node --experimental-wasm-modules');
    process.exit(1);
  }
}

/**
 * Read and parse share-usage.json files from dist directories
 */
function readShareUsageFiles() {
  const hostJsonPath = path.resolve(__dirname, '../host/dist/share-usage.json');
  const remoteJsonPath = path.resolve(__dirname, '../remote/dist/share-usage.json');
  
  const files = [];
  
  if (fs.existsSync(hostJsonPath)) {
    const hostData = JSON.parse(fs.readFileSync(hostJsonPath, 'utf8'));
    files.push({ name: 'host', data: hostData, path: hostJsonPath });
  }
  
  if (fs.existsSync(remoteJsonPath)) {
    const remoteData = JSON.parse(fs.readFileSync(remoteJsonPath, 'utf8'));
    files.push({ name: 'remote', data: remoteData, path: remoteJsonPath });
  }
  
  if (files.length === 0) {
    throw new Error('No share-usage.json files found. Please run build first.');
  }
  
  return files;
}

/**
 * Merge usage data from multiple apps into combined tree-shake flags
 * The new schema already contains treeShake structure with boolean values
 */
function mergeUsageData(files, targetApp) {
  const mergedTreeShake = {};

  // OR merge export usage across all apps
  files.forEach(({ data }) => {
    if (!data.treeShake) return;
    Object.entries(data.treeShake).forEach(([moduleKey, moduleExports]) => {
      if (!mergedTreeShake[moduleKey]) {
        mergedTreeShake[moduleKey] = {};
      }
      Object.entries(moduleExports).forEach(([exportName, isUsed]) => {
        if (exportName === 'chunk_characteristics') return; // skip metadata
        if (mergedTreeShake[moduleKey][exportName] !== true) {
          mergedTreeShake[moduleKey][exportName] = Boolean(isUsed);
        }
      });
    });
  });

  return {
    treeShake: mergedTreeShake,
    metadata: {
      timestamp: new Date().toISOString(),
      apps: files.map(f => f.name),
      modules: Object.keys(mergedTreeShake)
    }
  };
}

/**
 * Find shared library chunk files in dist directories
 */
function findSharedChunks(files) {
  const chunks = [];
  const distDirs = {
    host: path.resolve(__dirname, '../host/dist'),
    remote: path.resolve(__dirname, '../remote/dist')
  };

  files.forEach(({ name: app, data }) => {
    const distDir = distDirs[app];
    if (!distDir || !fs.existsSync(distDir)) return;

    if (!data.treeShake) return;

    Object.entries(data.treeShake).forEach(([library, moduleData]) => {
      const chunkFiles = moduleData?.chunk_characteristics?.chunk_files;
      if (!Array.isArray(chunkFiles)) return;

      chunkFiles.forEach(chunkFile => {
        if (typeof chunkFile !== 'string' || !chunkFile.endsWith('.js')) return;

        const fullPath = path.join(distDir, chunkFile);
        if (!fs.existsSync(fullPath)) return;

        // Skip maps, originals, optimized
        if (chunkFile.endsWith('.map') || chunkFile.endsWith('.original') || chunkFile.endsWith('.optimized.js')) return;

        chunks.push({
          path: fullPath,
          mapPath: fullPath + '.map',
          app,
          filename: chunkFile,
          library
        });
      });
    });
  });

  return chunks;
}

/**
 * Optimize a shared library chunk using SWC macro with tree-shake flags
 */
async function optimizeChunk(chunkPath, library, treeShakeConfig, optimizer, chunkCharacteristics) {
  console.log(`Optimizing chunk: ${path.basename(chunkPath)}`);
  
  try {
    // Enforce: require chunk characteristics only (entryModules removed)
    const hasChunkCharacteristics = Boolean(chunkCharacteristics?.entry_module_id);
    if (!hasChunkCharacteristics) {
      console.log(`Skipping ${path.basename(chunkPath)} for '${library}' - missing chunk characteristics`);
      return null;
    }

    const sourceCode = fs.readFileSync(chunkPath, 'utf8');
    
    // Create optimization config for the library - only include exports marked as true
    const libraryKeepFlags = {};
    if (treeShakeConfig[library]) {
      const exports = Object.entries(treeShakeConfig[library]).filter(([key]) => key !== 'chunk_characteristics');
      exports.forEach(([exportName, shouldKeep]) => {
        if (shouldKeep === true) {
          libraryKeepFlags[exportName] = true;
        }
      });
    }
    // Conservative skip: if there are no explicit export flags, do not attempt optimization
    if (Object.keys(libraryKeepFlags).length === 0) {
      console.log(`Skipping ${path.basename(chunkPath)} for '${library}' - no explicit export flags to keep`);
      return null;
    }
    
    // Build final treeShake config for THIS library only, attach per-app chunk characteristics
    const treeShakeLib = { ...libraryKeepFlags };
    if (chunkCharacteristics) {
      treeShakeLib.chunk_characteristics = chunkCharacteristics;
    }

    const config = { treeShake: { [library]: treeShakeLib } };
    
    const configJson = JSON.stringify(config);
    console.log(`Tree-shake config for ${library}:`, Object.keys(libraryKeepFlags).length, 'exports to keep');
    console.log('Exports to keep:', Object.keys(libraryKeepFlags).join(', '));
    
    // Run SWC macro optimization
    console.log(`DEBUG: Calling optimizer with config:`, JSON.stringify(config, null, 2));
    const result = optimizer.optimize(sourceCode, configJson);
    console.log(`DEBUG: Optimizer returned: type=${typeof result}, length=${result?.length}, changed=${result !== sourceCode}`);
    
    if (result && typeof result === 'string' && result !== sourceCode) {
      // Backup original
      if (!fs.existsSync(chunkPath + '.original')) {
        fs.writeFileSync(chunkPath + '.original', sourceCode);
      }
      
      // Write optimized code
      fs.writeFileSync(chunkPath, result);
      
      console.log(`✅ Optimized ${path.basename(chunkPath)}`);
      console.log(`   Original size: ${sourceCode.length} bytes`);
      console.log(`   Optimized size: ${result.length} bytes`);
      console.log(`   Size reduction: ${((sourceCode.length - result.length) / sourceCode.length * 100).toFixed(2)}%`);
      
      return {
        original_size: sourceCode.length,
        optimized_size: result.length,
        reduction: ((sourceCode.length - result.length) / sourceCode.length * 100)
      };
    } else {
      console.log(`ℹ️  No optimization applied for ${path.basename(chunkPath)}`);
      console.log(`   Result type: ${typeof result}`);
      console.log(`   Result equals source: ${result === sourceCode}`);
      return null;
    }
  } catch (error) {
    console.error(`❌ Failed to optimize ${path.basename(chunkPath)}:`, error.message);
    return null;
  }
}

/**
 * Main optimization function
 */
async function main() {
  console.log('🚀 Starting Module Federation chunk optimization...\n');
  
  try {
    // Load SWC macro optimizer
    console.log('Loading SWC macro optimizer...');
    const optimizer = await loadOptimizer();
    console.log('✅ SWC macro optimizer loaded\n');
    
    // Read share usage files
    console.log('Reading share-usage.json files...');
    const files = readShareUsageFiles();
    console.log(`✅ Found ${files.length} share-usage files: ${files.map(f => f.name).join(', ')}\n`);
    
    // Merge export usage across apps, but NOT chunk characteristics
    console.log('Merging export usage across apps (flags only)...');
    const mergedFlags = mergeUsageData(files).treeShake;
    console.log(`✅ Prepared merged export flags for modules: ${Object.keys(mergedFlags).join(', ')}\n`);
    
    // Find shared library chunks
    console.log('Finding shared library chunks...');
    const chunks = findSharedChunks(files);

    console.log(`✅ Found ${chunks.length} shared library chunks\n`);
    
    if (chunks.length === 0) {
      console.log('No shared library chunks found to optimize.');
      return;
    }
    
    // Do not group libraries; handle each chunk independently
    
    // Optimize each chunk individually per library (no context from other chunks)
    console.log('Optimizing chunks...');
    const results = [];
    for (const chunk of chunks) {
      const appData = files.find(f => f.name === chunk.app)?.data;
      const chunkCharacteristics = appData?.treeShake?.[chunk.library]?.chunk_characteristics;

      // Build single-lib flags from merged export usage across apps,
      // but attach ONLY this app's chunk_characteristics
      const singleLibFlags = {};
      const mergedLibFlags = mergedFlags[chunk.library] || {};
      singleLibFlags[chunk.library] = { ...mergedLibFlags };
      if (chunkCharacteristics) {
        singleLibFlags[chunk.library].chunk_characteristics = chunkCharacteristics;
      }

      const result = await optimizeChunk(
        chunk.path,
        chunk.library,
        singleLibFlags,
        optimizer,
        chunkCharacteristics
      );
      if (result) {
        results.push({
          app: chunk.app,
          filename: chunk.filename,
          library: chunk.library,
          ...result
        });
      }
    }
    
    // Summary
    console.log('\n📊 Optimization Summary:');
    console.log('========================');
    
    if (results.length > 0) {
      const totalOriginal = results.reduce((sum, r) => sum + r.original_size, 0);
      const totalOptimized = results.reduce((sum, r) => sum + r.optimized_size, 0);
      const totalReduction = ((totalOriginal - totalOptimized) / totalOriginal * 100);
      
      // Group results by library for better display
      const resultsByLibrary = results.reduce((acc, result) => {
        if (!acc[result.library]) acc[result.library] = [];
        acc[result.library].push(result);
        return acc;
      }, {});
      
      Object.entries(resultsByLibrary).forEach(([library, libResults]) => {
        console.log(`\n${library}:`);
        libResults.forEach(result => {
          console.log(`  ${result.app}/${result.filename}:`);
          console.log(`    Size reduction: ${result.reduction.toFixed(2)}% (${result.original_size.toLocaleString()} → ${result.optimized_size.toLocaleString()} bytes)`);
        });
      });
      
      console.log(`\nTotal size reduction: ${totalReduction.toFixed(2)}% (${totalOriginal} → ${totalOptimized} bytes)`);
      console.log(`Total bytes saved: ${totalOriginal - totalOptimized}`);
    } else {
      console.log('No chunks were optimized.');
    }
    
    console.log('\n✅ Module Federation optimization complete!');
    
  } catch (error) {
    console.error('❌ Optimization failed:', error.message);
    process.exit(1);
  }
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(console.error);
}

export { main, mergeUsageData, readShareUsageFiles };