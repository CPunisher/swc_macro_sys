import fs from 'fs';
import * as swcMacro from '../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

const antdChunk = fs.readFileSync('host/dist/node_modules_pnpm_antd_5_27_0_react-dom_18_3_1_react_18_3_1_node_modules_antd_es_index_js-_140c0.js', 'utf8');
const chunkInfo = JSON.parse(swcMacro.parse_webpack_chunk(antdChunk));

console.log('Total modules in antd chunk:', chunkInfo.module_count);
console.log('\nFirst 10 module keys:');
chunkInfo.module_keys.slice(0, 10).forEach(key => console.log('  -', key));

console.log('\nSearching for entry module ID...');
const targetId = '../../../node_modules/.pnpm/antd@5.27.0_react-dom@18.3.1_react@18.3.1/node_modules/antd/es/index.js';
const found = chunkInfo.module_keys.includes(targetId);
console.log('Entry module found:', found);

if (!found) {
  console.log('\nChecking for similar IDs (containing "antd" and "index"):');
  const similar = chunkInfo.module_keys.filter(key => key.includes('antd') && key.includes('index'));
  similar.slice(0, 5).forEach(key => console.log('  -', key));
}