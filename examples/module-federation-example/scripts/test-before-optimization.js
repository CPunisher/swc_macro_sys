#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

console.log('🧪 Testing bundles BEFORE optimization...\n');

// Test host bundle
console.log('📦 Testing HOST bundle:');
const hostPath = path.resolve(__dirname, '../host/dist/main.js');

// Set up minimal globals for webpack
global.self = { webpackChunkhost: [] };

try {
  const hostMain = require(hostPath);
  console.log('✅ Host main.js loaded successfully');
  console.log('   Module type:', typeof hostMain);
  console.log('   Exports:', Object.keys(hostMain));
  
  // Check if the promise resolves
  if (hostMain && hostMain.default && typeof hostMain.default.then === 'function') {
    console.log('   Default export is a promise (dynamic import)');
    hostMain.default.then(bootstrap => {
      console.log('   ✅ Dynamic import resolved successfully');
      console.log('   Bootstrap exports:', Object.keys(bootstrap));
    }).catch(err => {
      console.log('   ❌ Dynamic import failed:', err.message);
    });
  }
} catch (error) {
  console.error('❌ Failed to load host bundle:', error.message);
  if (error.stack) {
    console.log('\nStack trace:');
    console.log(error.stack);
  }
}

// Check lodash chunk sizes
console.log('\n📏 Lodash chunk sizes:');
const chunks = [
  {
    name: 'Host lodash chunk',
    path: '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js'
  },
  {
    name: 'Remote lodash chunk', 
    path: '../remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js'
  }
];

chunks.forEach(chunk => {
  const fullPath = path.resolve(__dirname, chunk.path);
  if (fs.existsSync(fullPath)) {
    const size = fs.statSync(fullPath).size;
    console.log(`   ${chunk.name}: ${(size/1024).toFixed(1)} KB`);
    
    // Check for MACRO comments
    const content = fs.readFileSync(fullPath, 'utf8');
    const macroCount = (content.match(/\/\/ MACRO:/g) || []).length;
    console.log(`     MACRO comments found: ${macroCount}`);
  } else {
    console.log(`   ${chunk.name}: NOT FOUND`);
  }
});