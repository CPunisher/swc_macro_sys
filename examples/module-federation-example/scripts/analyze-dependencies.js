#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Read the original vendor chunk and analyze dependencies
function analyzeDependencies() {
  const chunkPath = path.resolve(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original');
  
  if (!fs.existsSync(chunkPath)) {
    console.error('Original chunk not found. Run build first.');
    return;
  }
  
  const content = fs.readFileSync(chunkPath, 'utf8');
  
  // Extract all module definitions
  const moduleRegex = /"([^"]+lodash-es[^"]+\.js)":\s*(?:\/\*[\s\S]*?\*\/\s*)?function/g;
  const modules = [];
  let match;
  
  while ((match = moduleRegex.exec(content)) !== null) {
    modules.push(match[1]);
  }
  
  console.log(`Found ${modules.length} total modules in vendor chunk`);
  
  // Find dependencies for specific exports
  const exportsToKeep = ['capitalize', 'debounce', 'groupBy', 'omit', 'pick', 'sortBy', 'throttle', 'uniq', 'lodash.default'];
  const requiredModules = new Set();
  
  // Always include the main entry
  requiredModules.add('../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js');
  
  // For each export, find its module and dependencies
  exportsToKeep.forEach(exportName => {
    const moduleId = `../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/${exportName}.js`;
    if (modules.includes(moduleId)) {
      requiredModules.add(moduleId);
      console.log(`\nAnalyzing ${exportName}.js dependencies...`);
      
      // Extract the module's function body
      const modulePattern = new RegExp(`"${moduleId.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}":[^{]*{([^}]+})`, 's');
      const moduleMatch = content.match(modulePattern);
      
      if (moduleMatch) {
        // Find all require calls in this module
        const requirePattern = /__webpack_require__\("[^"]+"\)/g;
        const requires = moduleMatch[1].match(requirePattern) || [];
        
        requires.forEach(req => {
          const reqMatch = req.match(/__webpack_require__\("([^"]+)"\)/);
          if (reqMatch) {
            requiredModules.add(reqMatch[1]);
            console.log(`  - Requires: ${reqMatch[1]}`);
          }
        });
      }
    }
  });
  
  console.log(`\n\nTotal required modules: ${requiredModules.size}`);
  console.log('\nRequired modules list:');
  Array.from(requiredModules).sort().forEach(mod => {
    console.log(`  ${mod}`);
  });
  
  // Save the analysis
  const analysisPath = path.resolve(__dirname, '../dist/dependency-analysis.json');
  fs.mkdirSync(path.dirname(analysisPath), { recursive: true });
  fs.writeFileSync(analysisPath, JSON.stringify({
    totalModules: modules.length,
    requiredModules: Array.from(requiredModules),
    exportsAnalyzed: exportsToKeep,
    timestamp: new Date().toISOString()
  }, null, 2));
  
  console.log(`\nAnalysis saved to: ${analysisPath}`);
}

analyzeDependencies();