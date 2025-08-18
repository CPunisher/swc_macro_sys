import fs from 'fs';
import * as swcMacro from '../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Read the actual icons chunk
const iconsChunk = fs.readFileSync('host/dist/node_modules_pnpm_ant-design_icons_5_6_1_react-dom_18_3_1_react_18_3_1_node_modules_ant-desig-f1ca590.js', 'utf8');

// Read the share-usage config
const shareUsage = JSON.parse(fs.readFileSync('host/dist/share-usage.json', 'utf8'));

// Create config with @ant-design/icons tree-shake info
const config = {
  treeShake: {
    "@ant-design/icons": shareUsage.treeShake["@ant-design/icons"]
  }
};

console.log('Optimizing real icons chunk...\n');

// Check how many icons are marked as true
let trueCount = 0;
let falseCount = 0;
for (const [key, value] of Object.entries(config.treeShake["@ant-design/icons"])) {
  if (key !== 'chunk_characteristics') {
    if (value === true) trueCount++;
    else falseCount++;
  }
}

console.log(`Config has ${trueCount} icons marked as true, ${falseCount} as false\n`);

// Optimize the chunk
const result = swcMacro.optimize(iconsChunk, JSON.stringify(config));

// Check if AccountBookFilled (which should be false) was replaced with null
const accountBookMatch = result.match(/AccountBookFilled:\s*\(\)\s*=>\s*([^,\n]+)/);
if (accountBookMatch) {
  console.log('AccountBookFilled export after optimization:');
  console.log('  ' + accountBookMatch[0]);
  const isNull = accountBookMatch[1].includes('null');
  console.log('  Is null?', isNull);
}

// Check if UserOutlined (which should be true) still has the module reference
const userMatch = result.match(/UserOutlined:\s*\(\)\s*=>\s*([^,\n]+)/);
if (userMatch) {
  console.log('\nUserOutlined export after optimization:');
  console.log('  ' + userMatch[0]);
  const hasModule = userMatch[1].includes('WEBPACK_IMPORTED_MODULE');
  console.log('  Has module reference?', hasModule);
}

// Count how many exports are null vs have module references
const exportMatches = result.matchAll(/(\w+):\s*\(\)\s*=>\s*([^,\n]+)/g);
let nullCount = 0;
let moduleCount = 0;
for (const match of exportMatches) {
  if (match[2].includes('null')) nullCount++;
  else if (match[2].includes('WEBPACK_IMPORTED_MODULE')) moduleCount++;
}

console.log('\nExport summary:');
console.log(`  Null exports: ${nullCount}`);
console.log(`  Module exports: ${moduleCount}`);