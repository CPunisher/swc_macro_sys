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
    var processedData = _dataProcessor_ts__WEBPACK_IMPORTED_MODULE_1__/* .dataProcessor.processLargeDataset */.V.processLargeDataset([1, 2, 3, 4, 5]);
    return "FeatureA: Computed fibonacci(10)=".concat(result, ", processed ").concat(processedData.length, " items");
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
    }
};
}),

});

// The module cache
var __webpack_module_cache__ = {};

// The require function
/*#__NO_SIDE_EFFECTS__*/
function __webpack_require__(moduleId) {
var cachedModule = __webpack_module_cache__[moduleId];
if (cachedModule !== undefined) {
return cachedModule.exports;
}
var module = (__webpack_module_cache__[moduleId] = {
exports: {}
});
__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
return module.exports;
}

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

/************************************************************************/
var __webpack_exports__ = {};
(() => {
/* ESM import */var _featureA_ts__WEBPACK_IMPORTED_MODULE_0__ = /*#__PURE__*/ __webpack_require__(153);
// main.js - Entry point with only Feature A

console.log('=== Feature A Only Demo ===');
/* @common:if [condition="features.enableFeatureA"] */ 
console.log('Feature A enabled:', (0,_featureA_ts__WEBPACK_IMPORTED_MODULE_0__/* .featureA */.v)());
/* @common:endif */ 
console.log('Main application started - base functionality always included');
})();

})(); 