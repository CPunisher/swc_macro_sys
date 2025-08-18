import fs from 'fs';
import path from 'path';
import { execFileSync } from 'child_process';

/**
 * Analyzes a webpack chunk and returns metrics
 */
export function analyzeChunk(chunkPath) {
  const content = fs.readFileSync(chunkPath, 'utf-8');
  const size = fs.statSync(chunkPath).size;
  
  // Count modules
  const moduleCount = (content.match(/".+\.js":\s*function/g) || []).length;
  
  // Find exports
  const exports = [];
  const exportMatches = content.matchAll(/exports\.(\w+)\s*=/g);
  for (const match of exportMatches) {
    exports.push(match[1]);
  }
  
  // Check format
  const isCommonJS = content.includes('exports.modules');
  const isWebpackBundle = content.includes('__webpack_modules__');
  
  return {
    size,
    sizeKB: (size / 1024).toFixed(2),
    moduleCount,
    exports,
    format: isCommonJS ? 'commonjs' : isWebpackBundle ? 'webpack' : 'unknown'
  };
}

/**
 * Converts share-usage.json to optimization config
 */
export function shareUsageToConfig(shareUsagePath) {
  // The new schema already has the correct structure, so just return it directly
  return JSON.parse(fs.readFileSync(shareUsagePath, 'utf-8'));
}

/**
 * Optimizes a chunk with the given config by spawning a new Node process
 * with --experimental-wasm-modules to load the WASM optimizer reliably.
 */
export function optimizeChunk(chunkPath, config) {
  const source = fs.readFileSync(chunkPath, 'utf-8');
  const configStr = typeof config === 'string' ? config : JSON.stringify(config);

  const runner = `
    (async () => {
      const fs = require('fs');
      const optimizer = await import('../../../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js');
      const src = fs.readFileSync(${JSON.stringify(chunkPath)}, 'utf8');
      const cfg = ${JSON.stringify(configStr)};
      const out = optimizer.optimize(src, cfg);
      process.stdout.write(out);
    })().catch(e => { console.error(e); process.exit(1); });
  `;

  const output = execFileSync(process.execPath, ['--experimental-wasm-modules', '-e', runner], {
    encoding: 'utf8'
  });
  return output;
}

/**
 * Compares two chunks and returns metrics
 */
export function compareChunks(originalPath, optimizedPath) {
  const original = analyzeChunk(originalPath);
  const optimized = analyzeChunk(optimizedPath);
  
  const reduction = ((original.size - optimized.size) / original.size) * 100;
  const moduleReduction = ((original.moduleCount - optimized.moduleCount) / original.moduleCount) * 100;
  
  return {
    original,
    optimized,
    reduction: reduction.toFixed(2),
    moduleReduction: moduleReduction.toFixed(2),
    sizeSaved: original.size - optimized.size,
    sizeSavedKB: ((original.size - optimized.size) / 1024).toFixed(2)
  };
}

/**
 * Merges usage data from multiple apps
 */
export function mergeUsageData(...shareUsagePaths) {
  const mergedConfig = {
    treeShake: {}
  };
  
  shareUsagePaths.forEach(usagePath => {
    const usage = JSON.parse(fs.readFileSync(usagePath, 'utf-8'));
    
    Object.entries(usage.treeShake).forEach(([lib, exports]) => {
      if (!mergedConfig.treeShake[lib]) {
        mergedConfig.treeShake[lib] = { ...exports };
      } else {
        Object.entries(exports).forEach(([exportName, isUsed]) => {
          if (exportName === 'chunk_characteristics') {
            if (!mergedConfig.treeShake[lib].chunk_characteristics) {
              mergedConfig.treeShake[lib].chunk_characteristics = isUsed;
            }
          } else if (isUsed === true) {
            mergedConfig.treeShake[lib][exportName] = true;
          }
        });
      }
    });
  });
  
  return mergedConfig;
}