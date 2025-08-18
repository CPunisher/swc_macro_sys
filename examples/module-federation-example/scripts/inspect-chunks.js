#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

console.log('🔍 Inspecting optimized lodash chunks...\n');

function inspectChunk(name, chunkPath) {
  console.log(`\n📦 ${name.toUpperCase()} CHUNK ANALYSIS`);
  console.log('='.repeat(50));
  
  if (!fs.existsSync(chunkPath)) {
    console.log('❌ Chunk not found:', chunkPath);
    return;
  }
  
  const originalPath = chunkPath + '.original';
  const content = fs.readFileSync(chunkPath, 'utf8');
  const originalContent = fs.existsSync(originalPath) ? fs.readFileSync(originalPath, 'utf8') : null;
  
  // File sizes
  const size = fs.statSync(chunkPath).size;
  const originalSize = originalContent ? fs.statSync(originalPath).size : size;
  const reduction = originalContent ? ((originalSize - size) / originalSize * 100).toFixed(1) : 0;
  
  console.log(`📏 Size: ${(size/1024).toFixed(1)}KB`);
  if (originalContent) {
    console.log(`📏 Original: ${(originalSize/1024).toFixed(1)}KB (${reduction}% reduction)`);
  }
  
  // Function analysis
  console.log('\n🔧 LODASH FUNCTIONS ANALYSIS:');
  
  // Functions that should be present (used in our apps)
  const usedFunctions = [
    'capitalize', 'debounce', 'throttle', 'groupBy', 'omit', 'pick', 'uniq', 'sortBy'
  ];
  
  // Functions that should be removed (unused)
  const unusedFunctions = [
    'add', 'subtract', 'multiply', 'divide', 'mean', 'sum', 'times', 'random',
    'shuffle', 'sample', 'floor', 'ceil', 'round', 'max', 'min'
  ];
  
  console.log('\n✅ USED FUNCTIONS (should be present):');
  usedFunctions.forEach(fn => {
    const patterns = [
      `function ${fn}`,
      `"${fn}"`,
      `'${fn}'`,
      `${fn}:`,
      `${fn} =`,
      `${fn}(`
    ];
    
    const found = patterns.some(pattern => content.includes(pattern));
    const inOriginal = originalContent ? patterns.some(pattern => originalContent.includes(pattern)) : false;
    
    console.log(`  ${found ? '✅' : '❌'} ${fn.padEnd(12)} ${found ? 'PRESENT' : 'MISSING'}${inOriginal && !found ? ' (was in original)' : ''}`);
  });
  
  console.log('\n❌ UNUSED FUNCTIONS (should be removed):');
  unusedFunctions.forEach(fn => {
    const patterns = [
      `function ${fn}`,
      `"${fn}"`,
      `'${fn}'`,
      `${fn}:`,
      `${fn} =`,
      `${fn}(`
    ];
    
    const found = patterns.some(pattern => content.includes(pattern));
    const inOriginal = originalContent ? patterns.some(pattern => originalContent.includes(pattern)) : false;
    
    console.log(`  ${!found ? '✅' : '❌'} ${fn.padEnd(12)} ${!found ? 'REMOVED' : 'STILL PRESENT'}${inOriginal && found ? ' (was in original)' : ''}`);
  });
  
  // Code structure analysis
  console.log('\n🏗️  CODE STRUCTURE:');
  const lines = content.split('\n');
  console.log(`  Lines of code: ${lines.length}`);
  console.log(`  Contains webpack: ${content.includes('__webpack_require__') ? 'Yes' : 'No'}`);
  console.log(`  Contains lodash: ${content.includes('lodash') ? 'Yes' : 'No'}`);
  console.log(`  Minified: ${content.includes('\n\n') ? 'No' : 'Yes'}`);
  
  // Show a sample of the content
  console.log('\n📄 CONTENT SAMPLE (first 500 chars):');
  console.log('─'.repeat(50));
  console.log(content.substring(0, 500) + (content.length > 500 ? '...' : ''));
  console.log('─'.repeat(50));
  
  // Export analysis
  console.log('\n📤 EXPORT ANALYSIS:');
  const exportPatterns = [
    /exports\[["']([^"']+)["']\]/g,
    /exports\.([a-zA-Z_$][a-zA-Z0-9_$]*)/g,
    /__webpack_require__\.d\([^,]+,\s*["']([^"']+)["']/g
  ];
  
  const exports = new Set();
  exportPatterns.forEach(pattern => {
    let match;
    while ((match = pattern.exec(content)) !== null) {
      exports.add(match[1]);
    }
  });
  
  if (exports.size > 0) {
    console.log(`  Found ${exports.size} exports:`);
    Array.from(exports).slice(0, 20).forEach(exp => {
      console.log(`    • ${exp}`);
    });
    if (exports.size > 20) {
      console.log(`    ... and ${exports.size - 20} more`);
    }
  } else {
    console.log('  No clear exports found (may be webpack internal format)');
  }
  
  return {
    size,
    originalSize,
    reduction: parseFloat(reduction),
    usedFunctionsFound: usedFunctions.filter(fn => 
      [`function ${fn}`, `"${fn}"`, `'${fn}'`, `${fn}:`].some(pattern => content.includes(pattern))
    ).length,
    unusedFunctionsRemoved: unusedFunctions.filter(fn => 
      ![`function ${fn}`, `"${fn}"`, `'${fn}'`, `${fn}:`].some(pattern => content.includes(pattern))
    ).length,
    totalExports: exports.size
  };
}

// Inspect both chunks
const hostChunkPath = path.resolve(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
const remoteChunkPath = path.resolve(__dirname, '../remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');

const hostStats = inspectChunk('host', hostChunkPath);
const remoteStats = inspectChunk('remote', remoteChunkPath);

// Summary
console.log('\n\n📊 OVERALL SUMMARY');
console.log('='.repeat(50));
console.log(`Total size reduction: ${(hostStats.originalSize + remoteStats.originalSize - hostStats.size - remoteStats.size)/1024}KB`);
console.log(`Average reduction: ${((hostStats.reduction + remoteStats.reduction) / 2).toFixed(1)}%`);
console.log(`Used functions preserved: ${hostStats.usedFunctionsFound}/8 (host), ${remoteStats.usedFunctionsFound}/8 (remote)`);
console.log(`Unused functions removed: ${hostStats.unusedFunctionsRemoved}/15 (host), ${remoteStats.unusedFunctionsRemoved}/15 (remote)`);

console.log('\n🎯 OPTIMIZATION EFFECTIVENESS:');
if (hostStats.reduction > 30 && remoteStats.reduction > 30) {
  console.log('✅ Excellent optimization! Both chunks reduced by >30%');
} else if (hostStats.reduction > 20 && remoteStats.reduction > 20) {
  console.log('✅ Good optimization! Both chunks reduced by >20%');
} else {
  console.log('⚠️  Moderate optimization. Consider reviewing usage patterns.');
}

console.log('\n✨ Inspection complete!');