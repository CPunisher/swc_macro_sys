import { optimize } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// this string content is immutable and cannot be changed at all.
const webpackSample = `
(() => {
    var __webpack_modules__ = ({
        // Module 418 - dataProcessor with nested dependency
        418: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            var _helper_ts__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(153);
            __webpack_require__.d(__webpack_exports__, {
                V: () => (dataProcessor(_helper_ts__WEBPACK_IMPORTED_MODULE_0__.v))
            });
        }),

        // Module 153 - helper module
        153: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.d(__webpack_exports__, {
                v: () => (featureA)
            });
        }),

        // Module 722 - featureB
        722: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.d(__webpack_exports__, {
                S: () => (featureB)
            });
        }),

        // Module 422 - debugUtils
        422: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.d(__webpack_exports__, {
                qu: () => (debugLog)
            });
        })
    });

    (() => {
        /* ESM import */var _featureA_ts__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(153);
        /* ESM import */var _featureB_ts__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(722);
        /* ESM import */var _debugUtils_ts__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(422);
        // main.js - Entry point demonstrating conditional macro tree shaking

        console.log('=== Tree Shaking Demo ===');
        /* @common:if [condition="features.enableFeatureA"] */ console.log('Feature A enabled:', (0,_featureA_ts__WEBPACK_IMPORTED_MODULE_0__/* .featureA */.v)());
        /* @common:endif */ /* @common:if [condition="features.enableFeatureB"] */ console.log('Feature B enabled:', (0,_featureB_ts__WEBPACK_IMPORTED_MODULE_1__/* .featureB */.S)());
        /* @common:endif */ /* @common:if [condition="features.enableDebugMode"] */ (0,_debugUtils_ts__WEBPACK_IMPORTED_MODULE_2__/* .debugLog */.qu)('Debug mode active - this should be tree-shaken in production');
        /* @common:endif */ console.log('Main application started - base functionality always included');
    })();
})();
`;

console.log('📝 Input code:');
console.log(webpackSample);

try {
  const config = { features: { enableFeatureA: true, enableFeatureB: false } };
  const optimized = optimize(webpackSample, JSON.stringify(config));
  
  // Calculate size reduction
  const originalSize = webpackSample.length;
  const optimizedSize = optimized.length;
  const reduction = originalSize - optimizedSize;
  const reductionPercentage = ((reduction / originalSize) * 100).toFixed(1);
  
  console.log('\n📊 Size Analysis:');
  console.log(`  📏 Original size: ${originalSize} bytes`);
  console.log(`  📏 Optimized size: ${optimizedSize} bytes`);
  console.log(`  📉 Reduction: ${reduction} bytes (${reductionPercentage}%)`);
  
  console.log('\n⚡ After optimization:');
  console.log(optimized);
} catch (error) {
  console.error('Error:', error);
} 