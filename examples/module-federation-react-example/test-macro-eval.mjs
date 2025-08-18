import fs from 'fs';
import * as swcMacro from '../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Create a test chunk with macros
const testChunk = `
(self["webpackChunkmodule_federation_react_example_remote"] = self["webpackChunkmodule_federation_react_example_remote"] || []).push([["vendors-node_modules_pnpm_ant-design_icons"], {
  "../../../node_modules/.pnpm/@ant-design+icons@5.6.1/node_modules/@ant-design/icons/es/index.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    "use strict";
    __webpack_require__.r(__webpack_exports__);
    __webpack_require__.d(__webpack_exports__, {
      AccountBookFilled: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookFilled"] */ _icons__WEBPACK_IMPORTED_MODULE_0__.AccountBookFilled /* @common:endif */),
      AccountBookOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.AccountBookOutlined"] */ _icons__WEBPACK_IMPORTED_MODULE_0__.AccountBookOutlined /* @common:endif */),
      UserOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.UserOutlined"] */ _icons__WEBPACK_IMPORTED_MODULE_0__.UserOutlined /* @common:endif */),
      DeleteOutlined: () => (/* @common:if [condition="treeShake.@ant-design/icons.DeleteOutlined"] */ _icons__WEBPACK_IMPORTED_MODULE_0__.DeleteOutlined /* @common:endif */)
    });
  }
}]);
`;

// Create config where only UserOutlined and DeleteOutlined are true
const config = {
  treeShake: {
    "@ant-design/icons": {
      AccountBookFilled: false,
      AccountBookOutlined: false,
      UserOutlined: true,
      DeleteOutlined: true
    }
  }
};

console.log('Testing macro evaluation...\n');
console.log('Config:');
console.log('  AccountBookFilled: false');
console.log('  AccountBookOutlined: false');
console.log('  UserOutlined: true');
console.log('  DeleteOutlined: true\n');

// Optimize the test chunk
const result = swcMacro.optimize(testChunk, JSON.stringify(config));

// Check if false conditions were replaced with null
const hasAccountBookFilled = result.includes('AccountBookFilled: ()=>');
const hasAccountBookOutlined = result.includes('AccountBookOutlined: ()=>');
const hasUserOutlined = result.includes('UserOutlined: ()=>');
const hasDeleteOutlined = result.includes('DeleteOutlined: ()=>');

console.log('Results:');
console.log('  AccountBookFilled present:', hasAccountBookFilled);
console.log('  AccountBookOutlined present:', hasAccountBookOutlined);
console.log('  UserOutlined present:', hasUserOutlined);
console.log('  DeleteOutlined present:', hasDeleteOutlined);

// Show the actual export definitions
const exportsMatch = result.match(/__webpack_require__\.d\(__webpack_exports__, \{[\s\S]*?\}\);/);
if (exportsMatch) {
  console.log('\nActual exports definition:');
  console.log(exportsMatch[0]);
}