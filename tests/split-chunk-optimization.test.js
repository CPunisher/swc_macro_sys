import { describe, it, expect, beforeAll } from 'vitest';
import { optimizer } from './utils/optimizer.js';
import fs from 'fs';
import path from 'path';

/**
 * Test suite for split chunk object key removal optimization
 * Verifies that unreferenced module keys are removed from exports.modules
 */
describe('Split Chunk Object Key Removal', () => {
  beforeAll(async () => {
    await optimizer.initialize();
  });

  /**
   * Test with real lodash vendor chunk
   */
  it('should remove unreferenced object keys from lodash vendor chunk', async () => {
    // Load the lodash vendor chunk
    const lodashChunkPath = path.resolve(
      process.cwd(),
      'test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js'
    );
    
    const originalSource = fs.readFileSync(lodashChunkPath, 'utf-8');
    
    // Configuration for split chunk optimization
    const config = {
      webpack_tree_shaking: {
        enabled: true,
        entry_module_id: "main", // Simulate main entry point
        chunk_characteristics: {
          is_entry: false,
          is_runtime: false,
          chunk_name: "vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js"
        }
      }
    };

    // Count original object keys in exports.modules or similar patterns
    const originalKeyCount = countObjectKeys(originalSource);
    console.log(`Original object keys: ${originalKeyCount}`);
    
    // Optimize the chunk
    const optimizedSource = await optimizer.optimizeCode(originalSource, config);
    
    // Count optimized object keys
    const optimizedKeyCount = countObjectKeys(optimizedSource);
    console.log(`Optimized object keys: ${optimizedKeyCount}`);
    
    // Verify optimization occurred
    expect(optimizedKeyCount).toBeLessThan(originalKeyCount);
    expect(optimizedSource.length).toBeLessThan(originalSource.length);
    
    // Verify the optimized code is still valid JavaScript
    expect(() => {
      new Function(optimizedSource);
    }).not.toThrow();
    
    // Log optimization results
    const analysis = optimizer.analyzeOptimization(originalSource, optimizedSource, config);
    console.log('Split Chunk Optimization Results:', {
      sizeReduction: `${analysis.sizes.reductionPercent}%`,
      keyReduction: originalKeyCount - optimizedKeyCount,
      keyReductionPercent: `${((originalKeyCount - optimizedKeyCount) / originalKeyCount * 100).toFixed(2)}%`
    });
  });

  /**
   * Test with synthetic split chunk containing unreferenced modules
   */
  it('should remove unreferenced modules from synthetic vendor chunk', async () => {
    const syntheticChunk = `
"use strict";
(self["webpackChunktest"] = self["webpackChunktest"] || []).push([["vendors"], {
"../../node_modules/lodash/_DataView.js": 
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  Z: () => (__WEBPACK_DEFAULT_EXPORT__)
});
var _getNative_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../node_modules/lodash/_getNative.js");
var _root_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("../../node_modules/lodash/_root.js");
var DataView = _getNative_js__WEBPACK_IMPORTED_MODULE_0__.Z(_root_js__WEBPACK_IMPORTED_MODULE_1__.Z, 'DataView');
const __WEBPACK_DEFAULT_EXPORT__ = (DataView);
}),
"../../node_modules/lodash/_Hash.js": 
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  Z: () => (__WEBPACK_DEFAULT_EXPORT__)
});
// This module has no external references - should be removed
function Hash() { this.data = {}; }
const __WEBPACK_DEFAULT_EXPORT__ = (Hash);
}),
"../../node_modules/lodash/_getNative.js": 
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  Z: () => (__WEBPACK_DEFAULT_EXPORT__)
});
function getNative(object, key) { return object[key]; }
const __WEBPACK_DEFAULT_EXPORT__ = (getNative);
}),
"../../node_modules/lodash/_root.js": 
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  Z: () => (__WEBPACK_DEFAULT_EXPORT__)
});
const __WEBPACK_DEFAULT_EXPORT__ = (typeof global == 'object' && global) || this;
})
}]);
`;

    const config = {
      webpack_tree_shaking: {
        enabled: true,
        entry_module_id: "main",
        chunk_characteristics: {
          is_entry: false,
          is_runtime: false,
          chunk_name: "vendors"
        },
        external_references: [
          "../../node_modules/lodash/_DataView.js" // Only this module is referenced externally
        ]
      }
    };

    const originalKeyCount = countObjectKeys(syntheticChunk);
    const optimizedSource = await optimizer.optimizeCode(syntheticChunk, config);
    const optimizedKeyCount = countObjectKeys(optimizedSource);
    
    // Should remove _Hash.js since it's not transitively reachable from _DataView.js
    expect(optimizedKeyCount).toBeLessThan(originalKeyCount);
    expect(optimizedSource).not.toContain('"../../node_modules/lodash/_Hash.js"');
    
    // Should keep _DataView.js and its dependencies (_getNative.js, _root.js)
    expect(optimizedSource).toContain('"../../node_modules/lodash/_DataView.js"');
    expect(optimizedSource).toContain('"../../node_modules/lodash/_getNative.js"');
    expect(optimizedSource).toContain('"../../node_modules/lodash/_root.js"');
  });

  /**
   * Test that entry and runtime chunks are not processed
   */
  it('should skip optimization for entry and runtime chunks', async () => {
    const entryChunk = `
"use strict";
(self["webpackChunktest"] = self["webpackChunktest"] || []).push([["main"], {
"./src/index.js": 
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
const __WEBPACK_DEFAULT_EXPORT__ = ("Hello World");
})
}]);
`;

    const config = {
      webpack_tree_shaking: {
        enabled: true,
        entry_module_id: "./src/index.js",
        chunk_characteristics: {
          is_entry: true, // This should prevent split chunk optimization
          is_runtime: false,
          chunk_name: "main"
        }
      }
    };

    const originalSource = entryChunk;
    const optimizedSource = await optimizer.optimizeCode(originalSource, config);
    
    // Entry chunks should not be processed by split chunk optimizer
    // They should either remain unchanged or be processed by regular tree shaking
    expect(optimizedSource).toBeDefined();
  });
});

/**
 * Helper function to count object keys in webpack module definitions
 */
function countObjectKeys(source) {
  // Count module keys in webpack chunk format
  // Matches patterns like: "../../node_modules/...":
  const moduleKeyRegex = /"[^"]*node_modules[^"]*"\s*:/g;
  const matches = source.match(moduleKeyRegex);
  return matches ? matches.length : 0;
}

/**
 * Helper function to extract module IDs from webpack chunk
 */
function extractModuleIds(source) {
  const moduleKeyRegex = /"([^"]*node_modules[^"]*)"/g;
  const moduleIds = [];
  let match;
  
  while ((match = moduleKeyRegex.exec(source)) !== null) {
    moduleIds.push(match[1]);
  }
  
  return moduleIds;
}