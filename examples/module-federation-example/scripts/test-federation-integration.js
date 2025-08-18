#!/usr/bin/env node

const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

console.log('🧪 Running federation integration tests...\n');

// Change to the swc_macro_wasm crate directory
const crateDir = path.join(__dirname, '../../../crates/swc_macro_wasm');
process.chdir(crateDir);

// Ensure the test fixtures exist
const remoteDistPath = path.join(__dirname, '../remote/dist');
const chunkPath = path.join(remoteDistPath, 'vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
const configPath = path.join(remoteDistPath, 'share-usage.json');

if (!fs.existsSync(chunkPath) || !fs.existsSync(configPath)) {
    console.log('⚠️  Test fixtures not found. Building the module federation example first...\n');
    
    // Build the example
    process.chdir(path.join(__dirname, '..'));
    execSync('npm run build', { stdio: 'inherit' });
    
    // Go back to crate directory
    process.chdir(crateDir);
}

// Run the specific integration test
try {
    console.log('Running: cargo test federation_integration_test -- --nocapture\n');
    execSync('cargo test federation_integration_test -- --nocapture', { 
        stdio: 'inherit',
        env: { ...process.env, RUST_BACKTRACE: '1' }
    });
    
    console.log('\n✅ Integration tests passed!');
} catch (error) {
    console.error('\n❌ Integration tests failed!');
    process.exit(1);
}

// Also run a quick analysis of the current chunk
console.log('\n📊 Analyzing current chunk state...\n');

const chunk = fs.readFileSync(chunkPath, 'utf8');
const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));

// Count modules
const moduleMatches = chunk.match(/["'][^"']+["']:\s*\(function/g) || [];
console.log(`Total modules in chunk: ${moduleMatches.length}`);

// Count nullified exports
const nullExports = (chunk.match(/\.exports\s*=\s*null[,;]/g) || []).length;
console.log(`Nullified exports: ${nullExports}`);

// List kept exports
let keptExports = [];
if (config.treeShake && config.treeShake['lodash-es']) {
    // New dot notation format
    keptExports = Object.entries(config.treeShake['lodash-es'])
        .filter(([key, value]) => value === true && key !== 'chunk_characteristics')
        .map(([key]) => key);
} else if (config.exports) {
    // Old format
    keptExports = Object.keys(config.exports);
}
console.log(`\nKept exports (${keptExports.length}):`);
keptExports.forEach(exp => console.log(`  - ${exp}`));

// Sample of modules that should be removed
const sampleModulesToRemove = [
    'add.js', 'after.js', 'ary.js', 'assign.js', 'at.js',
    'attempt.js', 'before.js', 'bind.js', 'bindKey.js', 'camelCase.js'
];

console.log('\nChecking if sample modules exist (they should be removed):');
sampleModulesToRemove.forEach(module => {
    const pattern = new RegExp(`["'][^"']*${module}["']:\\s*\\(function`);
    const exists = pattern.test(chunk);
    console.log(`  ${exists ? '❌' : '✅'} ${module}`);
});