(() => { // webpackBootstrap
"use strict";
var __webpack_modules__ = ({
418: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  V: () => (dataProcessor)
});
// dataProcessor.js - Heavy data processing utilities (should be tree-shaken if not used)
var dataProcessor = {
    processLargeDataset (data) {
        console.log("Processing ".concat(data.length, " items - this is expensive data processing!"));
        // Simulate heavy data transformation
        return data.map((item)=>({
                id: item,
                processed: true,
                timestamp: Date.now(),
                metadata: {
                    processed: true,
                    heavy: 'computation'
                }
            }));
    },
    aggregateData (datasets) {
        console.log('Aggregating multiple datasets - heavy computation!');
        return datasets.reduce((acc, dataset)=>acc.concat(dataset), []);
    },
    transformComplexData (input) {
        console.log('Complex data transformation - should be tree-shaken if unused!');
        return {
            transformed: input,
            complexity: 'high'
        };
    }
};


}),
422: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  qu: () => (debugLog)
});
// debugUtils.js - Debug utilities (should be tree-shaken in production builds)
function debugLog(message) {
    console.log("[DEBUG] ".concat(new Date().toISOString(), ": ").concat(message));
    // Simulate expensive debug operations
    console.log('[DEBUG] Stack trace, memory usage, performance metrics...');
}
function debugPerformance() {
    console.log('[DEBUG] Performance monitoring - expensive debug code!');
    return {
        memory: '45MB',
        cpu: '12%',
        loadTime: '1.2s',
        operations: 1547
    };
}
function debugNetworkCalls() {
    console.log('[DEBUG] Network call debugging - should be tree-shaken in production!');
    return {
        calls: 23,
        errors: 0,
        avgTime: '180ms'
    };
}


}),
803: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  B: () => (expensiveUIUtils)
});
// expensiveUIUtils.js - Heavy UI utilities (should be tree-shaken if not used)
var expensiveUIUtils = {
    createComplexWidget () {
        console.log('Creating complex UI widgets - expensive DOM operations!');
        // Simulate heavy UI creation
        return {
            widgets: 42,
            complexity: 'high',
            renderTime: '150ms',
            elements: [
                'header',
                'body',
                'footer',
                'sidebar'
            ]
        };
    },
    renderHeavyComponents () {
        console.log('Rendering heavy components - should be tree-shaken if unused!');
        return {
            components: 15,
            rendered: true
        };
    },
    processUIEvents () {
        console.log('Processing complex UI events - expensive event handling!');
        return {
            events: [
                'click',
                'scroll',
                'resize'
            ],
            processed: 127
        };
    }
};


}),
153: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  v: () => (featureA)
});
/* ESM import */var _heavyMathUtils_ts__WEBPACK_IMPORTED_MODULE_0__ = /*#__PURE__*/ __webpack_require__(78);
/* ESM import */var _dataProcessor_ts__WEBPACK_IMPORTED_MODULE_1__ = /*#__PURE__*/ __webpack_require__(418);
// featureA.js - Feature A implementation that uses heavy utilities


function featureA() {
    console.log('FeatureA: Using heavy math utilities...');
    var result = _heavyMathUtils_ts__WEBPACK_IMPORTED_MODULE_0__/* .heavyMathUtils.fibonacci */.D.fibonacci(10);
    console.log('FeatureA: Processing complex data...');
    var processedData = _dataProcessor_ts__WEBPACK_IMPORTED_MODULE_1__/* .dataProcessor.processLargeDataset */.V.processLargeDataset([
        1,
        2,
        3,
        4,
        5
    ]);
    return "FeatureA: Computed fibonacci(10)=".concat(result, ", processed ").concat(processedData.length, " items");
}


}),
722: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  S: () => (featureB)
});
/* ESM import */var _expensiveUIUtils_ts__WEBPACK_IMPORTED_MODULE_0__ = /*#__PURE__*/ __webpack_require__(803);
/* ESM import */var _networkUtils_ts__WEBPACK_IMPORTED_MODULE_1__ = /*#__PURE__*/ __webpack_require__(812);
// featureB.js - Feature B implementation that uses different heavy utilities


function featureB() {
    console.log('FeatureB: Rendering complex UI components...');
    var uiResult = _expensiveUIUtils_ts__WEBPACK_IMPORTED_MODULE_0__/* .expensiveUIUtils.createComplexWidget */.B.createComplexWidget();
    console.log('FeatureB: Making network requests...');
    var networkResult = _networkUtils_ts__WEBPACK_IMPORTED_MODULE_1__/* .networkUtils.fetchAndCache */.t.fetchAndCache('https://api.example.com/data');
    return "FeatureB: Created ".concat(uiResult.widgets, " widgets, cached ").concat(networkResult.items, " network items");
}


}),
78: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  D: () => (heavyMathUtils)
});
// heavyMathUtils.js - Heavy mathematical computations (should be tree-shaken if not used)
var heavyMathUtils = {
    fibonacci (n) {
        console.log("Computing fibonacci(".concat(n, ") - this is expensive!"));
        if (n <= 1) return n;
        return this.fibonacci(n - 1) + this.fibonacci(n - 2);
    },
    primeFactors (n) {
        console.log("Computing prime factors of ".concat(n, " - another heavy operation!"));
        var factors = [];
        for(var i = 2; i <= n; i++){
            while(n % i === 0){
                factors.push(i);
                n /= i;
            }
        }
        return factors;
    },
    matrixMultiply (a, b) {
        console.log('Performing matrix multiplication - very expensive!');
        // Simulated heavy matrix operation
        return [
            [
                1,
                2
            ],
            [
                3,
                4
            ]
        ];
    }
};


}),
812: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  t: () => (networkUtils)
});
// networkUtils.js - Heavy networking utilities (should be tree-shaken if not used)
var networkUtils = {
    fetchAndCache (url) {
        console.log("Fetching and caching data from ".concat(url, " - expensive network operation!"));
        // Simulate heavy network and caching operations
        return {
            url,
            items: 73,
            cached: true,
            size: '2.4MB',
            compressionRatio: 0.65
        };
    },
    processNetworkRequests () {
        console.log('Processing batch network requests - should be tree-shaken if unused!');
        return {
            requests: 8,
            processed: true,
            totalTime: '340ms'
        };
    },
    optimizeConnections () {
        console.log('Optimizing network connections - expensive networking code!');
        return {
            connections: 12,
            optimized: true
        };
    }
};


}),

});
/************************************************************************/
// The module cache
var __webpack_module_cache__ = {};

// The require function
/*#__NO_SIDE_EFFECTS__*/
function __webpack_require__(moduleId) {

// Check if module is in cache
var cachedModule = __webpack_module_cache__[moduleId];
if (cachedModule !== undefined) {
return cachedModule.exports;
}
// Create a new module (and put it into the cache)
var module = (__webpack_module_cache__[moduleId] = {
exports: {}
});
// Execute the module function
__webpack_modules__[moduleId](module, module.exports, __webpack_require__);

// Return the exports of the module
return module.exports;

}

/************************************************************************/
// webpack/runtime/define_property_getters
(() => {
__webpack_require__.d = (exports, definition) => {
	for(var key in definition) {
        if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
            Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
        }
    }
};
})();
// webpack/runtime/has_own_property
(() => {
__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
})();
// webpack/runtime/rspack_version
(() => {
__webpack_require__.rv = () => ("1.3.12")
})();
// webpack/runtime/rspack_unique_id
(() => {
__webpack_require__.ruid = "bundler=rspack@1.3.12";

})();
/************************************************************************/
var __webpack_exports__ = {};
// This entry needs to be wrapped in an IIFE because it needs to be isolated against other modules in the chunk.
(() => {
/* ESM import */var _featureA_ts__WEBPACK_IMPORTED_MODULE_0__ = /*#__PURE__*/ __webpack_require__(153);
/* ESM import */var _featureB_ts__WEBPACK_IMPORTED_MODULE_1__ = /*#__PURE__*/ __webpack_require__(722);
/* ESM import */var _debugUtils_ts__WEBPACK_IMPORTED_MODULE_2__ = /*#__PURE__*/ __webpack_require__(422);
// main.js - Entry point demonstrating conditional macro tree shaking



console.log('=== Tree Shaking Demo ===');
/* @common:if [condition="features.enableFeatureA"] */ console.log('Feature A enabled:', (0,_featureA_ts__WEBPACK_IMPORTED_MODULE_0__/* .featureA */.v)());
/* @common:endif */ /* @common:if [condition="features.enableFeatureB"] */ console.log('Feature B enabled:', (0,_featureB_ts__WEBPACK_IMPORTED_MODULE_1__/* .featureB */.S)());
/* @common:endif */ /* @common:if [condition="features.enableDebugMode"] */ (0,_debugUtils_ts__WEBPACK_IMPORTED_MODULE_2__/* .debugLog */.qu)('Debug mode active - this should be tree-shaken in production');
/* @common:endif */ console.log('Main application started - base functionality always included');

})();

})()
;