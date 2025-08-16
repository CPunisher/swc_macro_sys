#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Helper functions similar to our Rust helpers
function analyzeModuleReferences(chunk) {
    const references = new Map();
    const requirePattern = /__webpack_require__\(["']([^"']+)["']\)/g;
    
    // Check if this is a module federation chunk with exports.modules format
    if (chunk.includes('exports.modules = {')) {
        // Extract the modules object
        const modulesStart = chunk.indexOf('exports.modules = {');
        const modulesEnd = chunk.lastIndexOf('};');
        
        if (modulesStart !== -1 && modulesEnd !== -1) {
            const modulesSection = chunk.substring(modulesStart, modulesEnd + 2);
            
            // Match module definitions
            const modulePattern = /"([^"]+)":\s*function\s*\([^)]*\)\s*\{/g;
            let match;
            
            while ((match = modulePattern.exec(modulesSection)) !== null) {
                const moduleName = match[1];
                const moduleStart = match.index + match[0].length;
                
                // Find the end of this module
                let braceCount = 1;
                let moduleEnd = moduleStart;
                
                for (let i = moduleStart; i < modulesSection.length && braceCount > 0; i++) {
                    if (modulesSection[i] === '{') braceCount++;
                    else if (modulesSection[i] === '}') braceCount--;
                    moduleEnd = i;
                }
                
                const moduleContent = modulesSection.substring(moduleStart, moduleEnd);
                
                // Extract dependencies
                const deps = [];
                let requireMatch;
                requirePattern.lastIndex = 0; // Reset regex
                
                while ((requireMatch = requirePattern.exec(moduleContent)) !== null) {
                    deps.push(requireMatch[1]);
                }
                
                references.set(moduleName, [...new Set(deps)]);
            }
        }
    }
    
    return references;
}

function findOrphanedModules(chunk, config) {
    const moduleRefs = analyzeModuleReferences(chunk);
    const exportedModules = new Set();
    const referencedModules = new Set();
    
    // Get all exported modules
    if (config.treeShake && config.treeShake['lodash-es']) {
        // New dot notation format
        const lodashConfig = config.treeShake['lodash-es'];
        
        // Map export names to their module paths
        const exportToModule = {
            'capitalize': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/capitalize.js',
            'groupBy': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/groupBy.js',
            'pick': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/pick.js',
            'throttle': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/throttle.js',
            'debounce': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/debounce.js',
            'omit': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/omit.js',
            'default': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.default.js'
        };
        
        Object.entries(lodashConfig).forEach(([exportName, isUsed]) => {
            if (isUsed === true && exportToModule[exportName]) {
                exportedModules.add(exportToModule[exportName]);
            }
        });
    } else if (config.exports) {
        // Old format for backwards compatibility
        Object.entries(config.exports).forEach(([exportName, exportInfo]) => {
            if (exportInfo.modules) {
                exportInfo.modules.forEach(module => exportedModules.add(module));
            }
        });
    }
    
    // Find all modules referenced by exported modules (transitive closure)
    const toProcess = [...exportedModules];
    const processed = new Set();
    
    while (toProcess.length > 0) {
        const current = toProcess.pop();
        if (processed.has(current)) continue;
        processed.add(current);
        
        const deps = moduleRefs.get(current) || [];
        deps.forEach(dep => {
            referencedModules.add(dep);
            if (!processed.has(dep)) {
                toProcess.push(dep);
            }
        });
    }
    
    // Find orphaned modules
    const orphaned = [];
    for (const [module] of moduleRefs) {
        if (!exportedModules.has(module) && !referencedModules.has(module)) {
            orphaned.push(module);
        }
    }
    
    return orphaned.sort();
}

// Main analysis
console.log('🔍 Module Federation Dependency Analysis\n');

const chunkPath = path.join(__dirname, '../remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
const configPath = path.join(__dirname, '../remote/dist/share-usage.json');

if (!fs.existsSync(chunkPath) || !fs.existsSync(configPath)) {
    console.error('❌ Required files not found. Run npm run build first.');
    process.exit(1);
}

const chunk = fs.readFileSync(chunkPath, 'utf8');
const config = JSON.parse(fs.readFileSync(configPath, 'utf8'));

// Basic stats
const moduleRefs = analyzeModuleReferences(chunk);
console.log(`📦 Total modules: ${moduleRefs.size}`);

// Handle different config formats
let exportedCount = 0;
if (config.treeShake && config.treeShake['lodash-es']) {
    // Count true values in the new format
    exportedCount = Object.values(config.treeShake['lodash-es'])
        .filter(v => v === true).length;
} else if (config.exports) {
    exportedCount = Object.keys(config.exports).length;
}
console.log(`📤 Exported functions: ${exportedCount}`);

// Find orphaned modules
const orphaned = findOrphanedModules(chunk, config);
console.log(`🗑️  Orphaned modules (should be removed): ${orphaned.length}\n`);

if (orphaned.length > 0) {
    console.log('First 20 orphaned modules:');
    orphaned.slice(0, 20).forEach(module => {
        const shortName = module.split('/').pop();
        console.log(`  - ${shortName} (${module})`);
    });
    
    if (orphaned.length > 20) {
        console.log(`  ... and ${orphaned.length - 20} more`);
    }
}

// Analyze specific problematic modules
console.log('\n🔎 Checking specific modules:');
const problematicModules = [
    'add.js',
    'after.js',
    'math.default.js',
    '_createMathOperation.js'
];

problematicModules.forEach(moduleName => {
    const fullPath = [...moduleRefs.keys()].find(k => k.endsWith(moduleName));
    if (fullPath) {
        const deps = moduleRefs.get(fullPath) || [];
        const isOrphaned = orphaned.includes(fullPath);
        console.log(`\n${moduleName}:`);
        console.log(`  Full path: ${fullPath}`);
        console.log(`  Status: ${isOrphaned ? '🗑️  Orphaned' : '✅ Referenced'}`);
        console.log(`  Dependencies: ${deps.length > 0 ? deps.map(d => d.split('/').pop()).join(', ') : 'none'}`);
        
        // Find who depends on this module
        const dependents = [];
        for (const [module, deps] of moduleRefs) {
            if (deps.includes(fullPath)) {
                dependents.push(module);
            }
        }
        console.log(`  Depended on by: ${dependents.length > 0 ? dependents.map(d => d.split('/').pop()).join(', ') : 'none'}`);
    } else {
        console.log(`\n${moduleName}: Not found in chunk`);
    }
});

// Show dependency chains for kept exports
console.log('\n📊 Dependency chains for kept exports:');
let keptExports = [];

if (config.treeShake && config.treeShake['lodash-es']) {
    // Get first 3 true exports from new format
    keptExports = Object.entries(config.treeShake['lodash-es'])
        .filter(([key, value]) => value === true && key !== 'chunk_characteristics')
        .slice(0, 3)
        .map(([key]) => key);
        
    const exportToModule = {
        'capitalize': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/capitalize.js',
        'groupBy': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/groupBy.js',
        'pick': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/pick.js',
        'throttle': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/throttle.js',
        'debounce': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/debounce.js',
        'omit': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/omit.js',
        'default': '../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.default.js'
    };
    
    keptExports.forEach(exportName => {
        if (exportToModule[exportName]) {
            console.log(`\n${exportName}:`);
            const module = exportToModule[exportName];
            console.log(`  Module: ${module.split('/').pop()}`);
            
            // Show direct dependencies
            const deps = moduleRefs.get(module) || [];
            if (deps.length > 0) {
                console.log(`  Direct deps: ${deps.map(d => d.split('/').pop()).join(', ')}`);
            }
        }
    });
} else if (config.exports) {
    // Old format
    keptExports = Object.keys(config.exports).slice(0, 3);
    keptExports.forEach(exportName => {
        const exportInfo = config.exports[exportName];
        if (exportInfo.modules && exportInfo.modules.length > 0) {
            console.log(`\n${exportName}:`);
            const module = exportInfo.modules[0];
            console.log(`  Module: ${module.split('/').pop()}`);
            
            // Show direct dependencies
            const deps = moduleRefs.get(module) || [];
            if (deps.length > 0) {
                console.log(`  Direct deps: ${deps.map(d => d.split('/').pop()).join(', ')}`);
            }
        }
    });
}

// Calculate potential savings
const chunkSize = chunk.length;
const orphanedSize = orphaned.reduce((total, module) => {
    const moduleStart = chunk.indexOf(`"${module}":`);
    if (moduleStart === -1) return total;
    
    let moduleEnd = moduleStart;
    let braceCount = 0;
    let inModule = false;
    
    for (let i = moduleStart; i < chunk.length; i++) {
        if (chunk[i] === '{') {
            braceCount++;
            inModule = true;
        } else if (chunk[i] === '}') {
            braceCount--;
            if (inModule && braceCount === 0) {
                moduleEnd = i + 1;
                break;
            }
        }
    }
    
    return total + (moduleEnd - moduleStart);
}, 0);

console.log(`\n💾 Potential savings:`);
console.log(`  Current size: ${(chunkSize / 1024).toFixed(1)} KB`);
console.log(`  Orphaned modules size: ~${(orphanedSize / 1024).toFixed(1)} KB`);
console.log(`  Potential reduction: ~${((orphanedSize / chunkSize) * 100).toFixed(1)}%`);