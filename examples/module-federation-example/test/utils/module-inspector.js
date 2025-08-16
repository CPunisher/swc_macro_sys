const fs = require('fs');
const path = require('path');
const vm = require('vm');

/**
 * Module Inspector Utility
 * Provides functions to inspect webpack bundles and extract module information
 */

/**
 * Load a webpack chunk file and extract the modules object
 * @param {string} chunkPath - Path to the webpack chunk file
 * @returns {Object} Extracted modules object with metadata
 */
function loadWebpackChunk(chunkPath) {
  try {
    if (!fs.existsSync(chunkPath)) {
      throw new Error(`Chunk file not found: ${chunkPath}`);
    }

    const chunkContent = fs.readFileSync(chunkPath, 'utf8');
    
    // Create a mock exports object to capture the chunk data
    const mockExports = {
      ids: [],
      modules: {}
    };

    // Create a sandbox context for safe execution
    const sandbox = {
      exports: mockExports,
      require: () => {},
      module: { exports: mockExports },
      console: console,
      // Add webpack globals that might be referenced
      __webpack_require__: () => {},
      __webpack_modules__: {}
    };

    // Execute the chunk file in the sandbox
    vm.createContext(sandbox);
    vm.runInContext(chunkContent, sandbox);

    return {
      chunkPath,
      ids: mockExports.ids || [],
      modules: mockExports.modules || {},
      moduleCount: Object.keys(mockExports.modules || {}).length,
      chunkSize: chunkContent.length,
      lastModified: fs.statSync(chunkPath).mtime
    };

  } catch (error) {
    throw new Error(`Failed to load webpack chunk ${chunkPath}: ${error.message}`);
  }
}

/**
 * Extract modules from a main webpack bundle
 * @param {string} bundlePath - Path to the main webpack bundle
 * @returns {Object} Extracted modules information
 */
function loadMainBundle(bundlePath) {
  try {
    if (!fs.existsSync(bundlePath)) {
      throw new Error(`Bundle file not found: ${bundlePath}`);
    }

    const bundleContent = fs.readFileSync(bundlePath, 'utf8');
    
    // Look for __webpack_modules__ object in the bundle
    const modulesMatch = bundleContent.match(/__webpack_modules__\s*=\s*\({([\s\S]*?)\}\);?/);
    
    if (!modulesMatch) {
      // Try alternative pattern for different webpack versions
      const altMatch = bundleContent.match(/var\s+__webpack_modules__\s*=\s*\({([\s\S]*?)\}\);?/);
      if (!altMatch) {
        return {
          bundlePath,
          modules: {},
          moduleCount: 0,
          bundleSize: bundleContent.length,
          lastModified: fs.statSync(bundlePath).mtime,
          error: 'Could not extract modules from bundle'
        };
      }
    }

    // Extract module IDs by parsing the content
    const moduleIds = [];
    const modulePattern = /"([^"]+)":\s*\/?\*[\s\S]*?\*\/?\s*(?:function|\()/g;
    let match;
    
    while ((match = modulePattern.exec(bundleContent)) !== null) {
      moduleIds.push(match[1]);
    }

    return {
      bundlePath,
      modules: {}, // Would need more complex parsing to extract full modules
      moduleIds,
      moduleCount: moduleIds.length,
      bundleSize: bundleContent.length,
      lastModified: fs.statSync(bundlePath).mtime
    };

  } catch (error) {
    throw new Error(`Failed to load main bundle ${bundlePath}: ${error.message}`);
  }
}

/**
 * Count modules and list their IDs from a webpack chunk
 * @param {Object} chunkData - Data returned from loadWebpackChunk
 * @returns {Object} Module count and ID information
 */
function analyzeModules(chunkData) {
  const moduleIds = Object.keys(chunkData.modules || {});
  
  return {
    totalModules: moduleIds.length,
    moduleIds: moduleIds.sort(),
    moduleTypes: categorizeModules(moduleIds),
    chunkInfo: {
      ids: chunkData.ids,
      size: chunkData.chunkSize,
      lastModified: chunkData.lastModified
    }
  };
}

/**
 * Categorize modules by their path patterns
 * @param {string[]} moduleIds - Array of module IDs
 * @returns {Object} Categorized module information
 */
function categorizeModules(moduleIds) {
  const categories = {
    nodeModules: [],
    src: [],
    webpack: [],
    other: []
  };

  moduleIds.forEach(id => {
    if (id.includes('node_modules')) {
      categories.nodeModules.push(id);
    } else if (id.includes('/src/') || id.startsWith('src/')) {
      categories.src.push(id);
    } else if (id.includes('webpack')) {
      categories.webpack.push(id);
    } else {
      categories.other.push(id);
    }
  });

  return {
    ...categories,
    counts: {
      nodeModules: categories.nodeModules.length,
      src: categories.src.length,
      webpack: categories.webpack.length,
      other: categories.other.length
    }
  };
}

/**
 * Compare two webpack chunks and show which modules were removed
 * @param {Object} originalChunk - Original chunk data
 * @param {Object} optimizedChunk - Optimized chunk data
 * @returns {Object} Comparison results
 */
function compareChunks(originalChunk, optimizedChunk) {
  const originalModules = new Set(Object.keys(originalChunk.modules || {}));
  const optimizedModules = new Set(Object.keys(optimizedChunk.modules || {}));

  const removedModules = [...originalModules].filter(id => !optimizedModules.has(id));
  const addedModules = [...optimizedModules].filter(id => !originalModules.has(id));
  const commonModules = [...originalModules].filter(id => optimizedModules.has(id));

  const sizeDifference = (originalChunk.chunkSize || 0) - (optimizedChunk.chunkSize || 0);
  const reductionPercentage = originalChunk.chunkSize > 0 
    ? ((sizeDifference / originalChunk.chunkSize) * 100).toFixed(2)
    : 0;

  return {
    summary: {
      originalModuleCount: originalModules.size,
      optimizedModuleCount: optimizedModules.size,
      removedCount: removedModules.length,
      addedCount: addedModules.length,
      commonCount: commonModules.length,
      sizeDifference,
      reductionPercentage: `${reductionPercentage}%`
    },
    details: {
      removedModules: removedModules.sort(),
      addedModules: addedModules.sort(),
      commonModules: commonModules.sort()
    },
    categorizedRemovals: categorizeModules(removedModules),
    categorizedAdditions: categorizeModules(addedModules),
    // Convenience accessors
    removedModules: removedModules,
    sizeReduction: parseFloat(reductionPercentage)
  };
}

/**
 * Verify if specific modules exist in a chunk
 * @param {Object} chunkData - Chunk data to search
 * @param {string|string[]} modulePatterns - Module ID patterns to search for
 * @returns {Object} Verification results
 */
function verifyModulesExist(chunkData, modulePatterns) {
  const patterns = Array.isArray(modulePatterns) ? modulePatterns : [modulePatterns];
  const moduleIds = Object.keys(chunkData.modules || {});
  
  const results = patterns.map(pattern => {
    const isRegex = pattern.startsWith('/') && pattern.endsWith('/');
    let matches = [];
    
    if (isRegex) {
      const regex = new RegExp(pattern.slice(1, -1));
      matches = moduleIds.filter(id => regex.test(id));
    } else {
      matches = moduleIds.filter(id => id.includes(pattern));
    }
    
    return {
      pattern,
      exists: matches.length > 0,
      matchCount: matches.length,
      matches: matches.sort()
    };
  });

  return {
    chunkPath: chunkData.chunkPath,
    totalModules: moduleIds.length,
    verificationResults: results,
    allPatternsFound: results.every(r => r.exists)
  };
}

/**
 * Verify if specific modules do NOT exist in a chunk
 * @param {Object} chunkData - Chunk data to search
 * @param {string|string[]} modulePatterns - Module ID patterns that should be absent
 * @returns {Object} Verification results
 */
function verifyModulesAbsent(chunkData, modulePatterns) {
  const verificationResult = verifyModulesExist(chunkData, modulePatterns);
  
  return {
    chunkPath: chunkData.chunkPath,
    totalModules: verificationResult.totalModules,
    verificationResults: verificationResult.verificationResults.map(result => ({
      ...result,
      absent: !result.exists,
      shouldBeAbsent: true
    })),
    allPatternsAbsent: verificationResult.verificationResults.every(r => !r.exists)
  };
}

/**
 * Get detailed information about specific modules
 * @param {Object} chunkData - Chunk data
 * @param {string[]} moduleIds - Specific module IDs to get info for
 * @returns {Object} Detailed module information
 */
function getModuleDetails(chunkData, moduleIds) {
  const modules = chunkData.modules || {};
  
  return moduleIds.map(id => {
    const moduleExists = modules.hasOwnProperty(id);
    let moduleInfo = {
      id,
      exists: moduleExists,
      size: 0,
      dependencies: [],
      exports: []
    };

    if (moduleExists && typeof modules[id] === 'function') {
      const moduleSource = modules[id].toString();
      moduleInfo.size = moduleSource.length;
      
      // Extract basic dependency information
      const requireMatches = moduleSource.match(/__webpack_require__\(\s*["']([^"']+)["']\s*\)/g) || [];
      moduleInfo.dependencies = requireMatches.map(match => {
        const idMatch = match.match(/["']([^"']+)["']/);
        return idMatch ? idMatch[1] : null;
      }).filter(Boolean);

      // Extract export information
      const exportMatches = moduleSource.match(/__webpack_exports__\["([^"]+)"\]/g) || [];
      moduleInfo.exports = exportMatches.map(match => {
        const nameMatch = match.match(/"([^"]+)"/);
        return nameMatch ? nameMatch[1] : null;
      }).filter(Boolean);
    }

    return moduleInfo;
  });
}

/**
 * Scan a directory for webpack chunks and analyze all of them
 * @param {string} distDir - Directory containing webpack chunks
 * @returns {Object} Analysis of all chunks in the directory
 */
function scanChunksInDirectory(distDir) {
  try {
    const files = fs.readdirSync(distDir);
    const chunkFiles = files.filter(file => 
      file.endsWith('.js') && !file.includes('.map')
    );

    const results = {};
    
    chunkFiles.forEach(file => {
      const filePath = path.join(distDir, file);
      try {
        if (file === 'main.js') {
          results[file] = loadMainBundle(filePath);
        } else {
          results[file] = loadWebpackChunk(filePath);
        }
      } catch (error) {
        results[file] = {
          error: error.message,
          filePath
        };
      }
    });

    return {
      directory: distDir,
      totalChunks: chunkFiles.length,
      chunks: results,
      summary: {
        totalModules: Object.values(results).reduce((sum, chunk) => 
          sum + (chunk.moduleCount || 0), 0
        ),
        totalSize: Object.values(results).reduce((sum, chunk) => 
          sum + (chunk.chunkSize || chunk.bundleSize || 0), 0
        )
      }
    };
  } catch (error) {
    throw new Error(`Failed to scan directory ${distDir}: ${error.message}`);
  }
}

module.exports = {
  loadWebpackChunk,
  loadMainBundle,
  analyzeModules,
  categorizeModules,
  compareChunks,
  verifyModulesExist,
  verifyModulesAbsent,
  getModuleDetails,
  scanChunksInDirectory
};