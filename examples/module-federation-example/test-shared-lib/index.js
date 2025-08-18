// Main entry point that exports everything from all modules
// This simulates a typical shared library index file

// Completely unused modules
export * from './unused-module-a.js';
export * from './unused-module-b.js';

// Partially used modules
export * from './partial-usage-a.js';
export * from './partial-usage-b.js';

// Fully used modules
export * from './core-utils.js';
export * from './main-feature.js';

// Nested modules
export * from './nested/used-shallow.js';
export * from './nested/deep/unused-deep.js';

// Modules with internal dependencies
export * from './dependent-module.js';
export * from './dependency-chain-a.js';
export * from './dependency-chain-b.js';

// Legacy modules (unused)
export * from './legacy-helper.js';
export * from './deprecated-utils.js';