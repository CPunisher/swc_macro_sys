#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Read the optimized chunk
const chunkPath = path.join(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
const chunkContent = fs.readFileSync(chunkPath, 'utf-8');

// Read the merged config to know what we're keeping
const configPath = path.join(__dirname, '../dist/merged-tree-shake-config.json');
const config = JSON.parse(fs.readFileSync(configPath, 'utf-8'));

// Extract kept exports
const keptExports = Object.entries(config.treeShake['lodash-es'])
  .filter(([_, keep]) => keep === true)
  .map(([name, _]) => name);

console.log('=== KEPT EXPORTS ===');
console.log(keptExports);

// Parse all modules from the chunk
const moduleRegex = /"(\.\.\/\.\.\/node_modules\/\.pnpm\/lodash-es@4\.17\.21\/node_modules\/lodash-es\/([^"]+)\.js)": function/g;
const modules = [];
let match;

while ((match = moduleRegex.exec(chunkContent)) !== null) {
  modules.push({
    path: match[1],
    name: match[2]
  });
}

console.log(`\n=== TOTAL MODULES: ${modules.length} ===`);

// Categorize modules
const categories = {
  keptExports: [],
  helperModules: [],
  internalUtilities: [],
  otherExports: [],
  specialModules: []
};

modules.forEach(module => {
  const name = module.name;
  
  if (keptExports.includes(name)) {
    categories.keptExports.push(name);
  } else if (name.startsWith('_')) {
    categories.helperModules.push(name);
  } else if (name === 'lodash' || name === 'lodash.default' || name.includes('.default')) {
    categories.specialModules.push(name);
  } else if (['isBuffer', 'isMap', 'isSet', 'isArguments', 'isArray', 'isArrayBuffer', 'isDate', 'isError', 'isFunction', 'isNumber', 'isObject', 'isRegExp', 'isString', 'isTypedArray', 'isPlainObject', 'isEqual'].includes(name)) {
    categories.internalUtilities.push(name);
  } else {
    categories.otherExports.push(name);
  }
});

// Print categorized modules
console.log('\n=== MODULE CATEGORIES ===');
console.log(`Kept Exports (${categories.keptExports.length}):`, categories.keptExports);
console.log(`\nHelper Modules (${categories.helperModules.length}):`, categories.helperModules.length);
console.log(`\nInternal Utilities (${categories.internalUtilities.length}):`, categories.internalUtilities);
console.log(`\nSpecial Modules (${categories.specialModules.length}):`, categories.specialModules);
console.log(`\nOTHER EXPORTS THAT SHOULD BE REMOVED (${categories.otherExports.length}):`, categories.otherExports);

// Check for nullified exports that still have modules
const nullifiedExports = Object.entries(config.treeShake['lodash-es'])
  .filter(([_, keep]) => keep === false)
  .map(([name, _]) => name);

const nullifiedButPresent = modules
  .map(m => m.name)
  .filter(name => nullifiedExports.includes(name));

console.log(`\n=== NULLIFIED EXPORTS STILL PRESENT (${nullifiedButPresent.length}) ===`);
console.log(nullifiedButPresent);

// Analyze dependencies for kept exports
console.log('\n=== ANALYZING DEPENDENCIES ===');

function findModuleDependencies(moduleName) {
  const modulePattern = new RegExp(`"[^"]*/${moduleName}\\.js": function.*?\\{([^}]|\\}(?!,\\n))*\\}`, 's');
  const moduleMatch = chunkContent.match(modulePattern);
  
  if (!moduleMatch) {
    return [];
  }
  
  const moduleContent = moduleMatch[0];
  const requirePattern = /__webpack_require__\("([^"]+)"\)/g;
  const dependencies = [];
  let depMatch;
  
  while ((depMatch = requirePattern.exec(moduleContent)) !== null) {
    const depPath = depMatch[1];
    const depName = depPath.split('/').pop().replace('.js', '');
    if (!dependencies.includes(depName)) {
      dependencies.push(depName);
    }
  }
  
  return dependencies;
}

// Build complete dependency graph
const dependencyGraph = {};
const requiredModules = new Set();

// Start with kept exports
keptExports.forEach(exportName => {
  if (exportName === 'default') return;
  requiredModules.add(exportName);
  const deps = findModuleDependencies(exportName);
  dependencyGraph[exportName] = deps;
  deps.forEach(d => requiredModules.add(d));
});

// Add lodash main module
requiredModules.add('lodash');
const lodashDeps = findModuleDependencies('lodash');
dependencyGraph['lodash'] = lodashDeps;

// Trace all transitive dependencies
let changed = true;
while (changed) {
  changed = false;
  const currentRequired = [...requiredModules];
  
  currentRequired.forEach(moduleName => {
    if (!dependencyGraph[moduleName]) {
      const deps = findModuleDependencies(moduleName);
      if (deps.length > 0) {
        dependencyGraph[moduleName] = deps;
        deps.forEach(d => {
          if (!requiredModules.has(d)) {
            requiredModules.add(d);
            changed = true;
          }
        });
      }
    }
  });
}

console.log(`\nRequired modules (including dependencies): ${requiredModules.size}`);

// Find truly unused modules
const allModuleNames = modules.map(m => m.name);
const unusedModules = allModuleNames.filter(name => !requiredModules.has(name));

console.log(`\n=== TRULY UNUSED MODULES (${unusedModules.length}) ===`);
console.log('These modules have no incoming requires from kept exports:');
unusedModules.forEach(name => {
  console.log(`  - ${name}`);
});

// Calculate potential savings
const currentSize = chunkContent.length;
let unusedSize = 0;

unusedModules.forEach(moduleName => {
  const modulePattern = new RegExp(`"[^"]*/${moduleName}\\.js": function.*?\\{([^}]|\\}(?!,\\n))*\\},?\\n`, 's');
  const moduleMatch = chunkContent.match(modulePattern);
  if (moduleMatch) {
    unusedSize += moduleMatch[0].length;
  }
});

console.log(`\n=== POTENTIAL SAVINGS ===`);
console.log(`Current chunk size: ${(currentSize / 1024).toFixed(2)} KB`);
console.log(`Unused modules size: ${(unusedSize / 1024).toFixed(2)} KB`);
console.log(`Potential reduction: ${((unusedSize / currentSize) * 100).toFixed(2)}%`);
console.log(`Optimized size would be: ${((currentSize - unusedSize) / 1024).toFixed(2)} KB`);

// Check specific problematic modules
console.log('\n=== CHECKING SPECIFIC MODULES ===');
const problematicModules = ['add', 'after', 'ary', 'assign', 'chunk', 'clone', 'compact', 'concat'];
problematicModules.forEach(name => {
  const present = modules.some(m => m.name === name);
  const required = requiredModules.has(name);
  console.log(`${name}: present=${present}, required=${required}`);
});