# Test Shared Library - Module Removal Testing

This directory contains a controlled test library designed to verify module removal and tree-shaking optimization. It includes various patterns to test different aspects of dead code elimination.

## Structure Overview

### Used Modules (Should be preserved)
- **`core-utils.js`** - Fully used core utilities (all exports used)
- **`main-feature.js`** - Main business logic (all exports used)
- **`nested/used-shallow.js`** - Used nested module (all exports used)

### Partially Used Modules (Mixed preservation)
- **`partial-usage-a.js`** - Only `formatDate` and `calculateAge` are used
- **`partial-usage-b.js`** - Only `isValidEmail` and `generateId` are used
- **`dependent-module.js`** - Mixed usage with dependency relationships

### Completely Unused Modules (Should be removed entirely)
- **`unused-module-a.js`** - No imports or usage
- **`unused-module-b.js`** - No imports or usage
- **`nested/deep/unused-deep.js`** - Deep nested unused module
- **`legacy-helper.js`** - Legacy code, completely unused
- **`deprecated-utils.js`** - Deprecated utilities, completely unused

### Dependency Chain Testing
- **`dependency-chain-a.js`** - Unused module that imports from chain-b
- **`dependency-chain-b.js`** - Unused module imported only by chain-a
- Both should be removed through cascading dependency elimination

## Testing Scenarios

### 1. Complete Module Removal
Modules that are never imported should be completely removed:
- `unused-module-a.js` 
- `unused-module-b.js`
- `nested/deep/unused-deep.js`
- `legacy-helper.js`
- `deprecated-utils.js`

### 2. Partial Export Removal
Modules with mixed usage should have unused exports removed:
- `partial-usage-a.js`: Keep `formatDate`, `calculateAge`; remove others
- `partial-usage-b.js`: Keep `isValidEmail`, `generateId`; remove others

### 3. Cascading Dependency Removal
Unused dependency chains should be completely removed:
- `dependency-chain-a.js` → `dependency-chain-b.js`
- Both modules should be removed since chain-a is never used

### 4. Nested Structure Handling
Test deep directory structures:
- `nested/used-shallow.js` - Should be preserved (used)
- `nested/deep/unused-deep.js` - Should be removed (unused)

### 5. Mixed Dependency Resolution
`dependent-module.js` tests complex scenarios:
- Has both used and unused functions
- Depends on both used and unused modules
- Should preserve used parts and their dependencies only

## Usage Pattern

The `consumer-example.js` file demonstrates realistic usage:
- Imports only specific functions that are actually needed
- Creates realistic application patterns
- Shows how tree-shaking should work in practice

## Expected Optimization Results

After tree-shaking optimization:

### Files that should be completely removed:
- `unused-module-a.js`
- `unused-module-b.js` 
- `nested/deep/unused-deep.js`
- `dependency-chain-a.js`
- `dependency-chain-b.js`
- `legacy-helper.js`
- `deprecated-utils.js`

### Files that should be partially optimized:
- `partial-usage-a.js` - Remove unused functions, keep `formatDate`, `calculateAge`
- `partial-usage-b.js` - Remove unused functions, keep `isValidEmail`, `generateId`
- `dependent-module.js` - Remove `processUnused` method and unused dependencies

### Files that should be fully preserved:
- `core-utils.js` - All exports used
- `main-feature.js` - All exports used  
- `nested/used-shallow.js` - All exports used
- `index.js` - Entry point (but exports should be filtered)

This structure provides comprehensive testing for:
- Dead code elimination
- Dependency graph analysis
- Nested module handling
- Partial tree-shaking
- Cascading removal effects