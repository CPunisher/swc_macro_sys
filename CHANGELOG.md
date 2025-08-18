# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2025-11-13

### Changed

#### Major Architecture Refactoring
- **Consolidated tree shaking implementation** - All tree shaking logic now lives in `swc_macro_wasm/src/optimize.rs`
- **Removed webpack_chunk_tree_shaker crate** - No longer needed after consolidation

- **Test suite consolidation** - Removed 9 outdated test files, consolidated into 20 focused test files

#### Implementation Improvements
- **Explicit entry point policy** - No more filename-based inference, configuration-driven approach only
- **AST-based mutation strategy** - Direct AST manipulation instead of string reconstruction
- **Simplified architecture** - Reduced inter-crate dependencies and communication overhead
- **Enhanced error handling** - Comprehensive error types with recovery strategies
- **Performance optimizations** - Caching, convergence detection, and reduced allocations

### Added
- `swc_macro_wasm/README.md` - Comprehensive documentation for the optimization crate
- `cache.rs` module - AST emission caching for performance
- `convergence.rs` module - Optimization loop detection
- `performance.rs` module - Metrics collection and monitoring
- `error.rs` module - Structured error handling system
- `config.rs` module - Configuration management and validation
- `dce.rs` module - Advanced dead code elimination

### Removed
- `webpack_chunk_tree_shaker` crate - Functionality moved to swc_macro_wasm
- Test files (2,458 lines removed):
  - `federation_integration_test.rs`
  - `react_dom_regression_test.rs`
  - `split_chunk_tests.rs`
  - `tree_shaking_effectiveness_test.rs`
  - `tree_shaking_skip_no_entries.rs`
  - `verify_tree_shaker_effectiveness_test.rs`
  - `webpack_dependency_tracking_test.rs`

### Fixed

- Unsafe code patterns replaced with safe alternatives
- Test structure aligned with new architecture

## [0.2.0] - 2025-08-13

### Added
- Initial tree shaking design and implementation

- Module federation support
- Comprehensive test suite with 160+ tests

### Changed
- Migrated from regex-based to AST-based parsing
- Improved dependency graph algorithms

## [0.1.0] - 2025-07-01

### Added
- Initial project setup
- Basic macro parsing capabilities
- WASM bindings for JavaScript integration
- SWC integration for AST manipulation