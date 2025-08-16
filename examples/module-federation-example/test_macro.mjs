import { optimize } from '../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

const chunk = `
  /* @common:if [condition="treeShake.lodash-es.sortBy"] */
  exports.sortBy = __webpack_require__("sortBy.js").default;
  /* @common:endif */
  /* @common:if [condition="treeShake.lodash-es.map"] */
  exports.map = __webpack_require__("map.js").default;
  /* @common:endif */
`;

const config = {
  treeShake: {
    'lodash-es': { sortBy: true, map: false }
  }
};

const optimized = optimize(chunk, JSON.stringify(config));
console.log("Optimized output:");
console.log(optimized);
console.log("\nContains sortBy:", optimized.includes('sortBy'));
console.log("Contains map:", optimized.includes('map'));
