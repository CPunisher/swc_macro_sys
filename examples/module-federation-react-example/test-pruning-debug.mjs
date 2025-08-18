import fs from 'fs';
import * as swcMacro from '../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Read the antd chunk
const antdChunk = fs.readFileSync('host/dist/node_modules_pnpm_antd_5_27_0_react-dom_18_3_1_react_18_3_1_node_modules_antd_es_index_js-_140c0.js', 'utf8');

// Read the share-usage config
const shareUsage = JSON.parse(fs.readFileSync('host/dist/share-usage.json', 'utf8'));

// Create the config with only antd tree-shake info
const config = {
  treeShake: {
    antd: shareUsage.treeShake.antd
  }
};

// Optimize with debug output - stderr will show debug messages
console.log('Running optimization with debug output...');
const result = swcMacro.optimize_with_prune_result_json(antdChunk, JSON.stringify(config));
const parsed = JSON.parse(result);

console.log('\nPrune result:');
console.log('  Original count:', parsed.prune_result.original_count);
console.log('  Pruned count:', parsed.prune_result.pruned_count);
console.log('  Skip reason:', parsed.prune_result.skip_reason);

// Parse to see actual module count
const chunkInfo = JSON.parse(swcMacro.parse_webpack_chunk(parsed.optimized_source));
console.log('\nAfter optimization:');
console.log('  Module count:', chunkInfo.module_count);