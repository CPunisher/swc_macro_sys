#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

console.log('🔍 Verifying exports and modules in optimized chunks...\n');

function analyzeChunk(chunkPath, chunkName) {
  console.log(`\n=== Analyzing ${chunkName} ===`);
  
  if (!fs.existsSync(chunkPath)) {
    console.log(`❌ Chunk not found: ${chunkPath}`);
    return;
  }
  
  const content = fs.readFileSync(chunkPath, 'utf8');
  const size = fs.statSync(chunkPath).size;
  
  console.log(`File size: ${(size/1024).toFixed(2)} KB`);
  
  // Extract module IDs
  const modulePattern = /"([^"]+\.js)":\s*\/\*!/g;
  const modules = [];
  let match;
  
  while ((match = modulePattern.exec(content)) !== null) {
    modules.push(match[1]);
  }
  
  console.log(`\nModules found (${modules.length}):`);
  modules.forEach(mod => {
    // Extract just the filename
    const filename = mod.split('/').pop();
    console.log(`  - ${filename}`);
  });
  
  // Look for exports
  const exportPattern = /(__webpack_require__\.d\(__webpack_exports__, {[^}]+})|"([^"]+)":\s*\(\)\s*=>\s*\(/g;
  const exports = new Set();
  
  // Find export definitions
  const exportDefPattern = /"([^"]+)":\s*\(\)\s*=>\s*\(/g;
  while ((match = exportDefPattern.exec(content)) !== null) {
    if (match[1] !== 'default' && !match[1].includes('node_modules')) {
      exports.add(match[1]);
    }
  }
  
  // Also look for webpack exports
  const webpackExportPattern = /__webpack_require__\.d\(__webpack_exports__, {([^}]+)}/g;
  while ((match = webpackExportPattern.exec(content)) !== null) {
    const exportBlock = match[1];
    const exportNames = exportBlock.match(/"([^"]+)":/g);
    if (exportNames) {
      exportNames.forEach(name => {
        const cleanName = name.replace(/[":]/g, '');
        if (cleanName !== 'default') {
          exports.add(cleanName);
        }
      });
    }
  }
  
  console.log(`\nExports found (${exports.size}):`);
  if (exports.size > 0) {
    Array.from(exports).sort().forEach(exp => {
      console.log(`  - ${exp}`);
    });
  } else {
    console.log('  No named exports found');
  }
  
  // Check for specific lodash functions that should be preserved
  const expectedExports = ['sortBy', 'uniq', 'capitalize', 'debounce', 'groupBy', 'omit', 'pick', 'throttle'];
  const missingExports = expectedExports.filter(exp => !content.includes(exp));
  
  if (missingExports.length > 0) {
    console.log(`\n⚠️  Expected exports not found in chunk:`);
    missingExports.forEach(exp => console.log(`  - ${exp}`));
  }
  
  // Look for the actual lodash re-export module
  const lodashExportPattern = /lodash\.js[^}]*__webpack_require__\.d\(__webpack_exports__, {([^}]+)}/s;
  const lodashMatch = content.match(lodashExportPattern);
  
  if (lodashMatch) {
    console.log('\n✅ Found lodash.js export definitions');
    // Count how many exports are defined
    const exportCount = (lodashMatch[1].match(/"/g) || []).length / 2;
    console.log(`   ${exportCount} exports defined in lodash.js module`);
  }
  
  // Sample the content to see what's actually there
  console.log('\nFirst 500 characters of chunk:');
  console.log(content.substring(0, 500) + '...');
  
  if (content.length < 2000) {
    console.log('\n⚠️  Chunk seems very small. Full content:');
    console.log(content);
  }
}

// Analyze both chunks
const hostChunk = path.resolve(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
const remoteChunk = path.resolve(__dirname, '../remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');

analyzeChunk(hostChunk, 'Host Lodash Chunk');
analyzeChunk(remoteChunk, 'Remote Lodash Chunk');

// Compare with original if available
const hostOriginal = hostChunk + '.original';
if (fs.existsSync(hostOriginal)) {
  const originalSize = fs.statSync(hostOriginal).size;
  const optimizedSize = fs.statSync(hostChunk).size;
  const reduction = ((originalSize - optimizedSize) / originalSize * 100).toFixed(2);
  
  console.log('\n📊 Optimization Summary:');
  console.log(`Original size: ${(originalSize/1024).toFixed(1)} KB`);
  console.log(`Optimized size: ${(optimizedSize/1024).toFixed(1)} KB`);
  console.log(`Reduction: ${reduction}%`);
}