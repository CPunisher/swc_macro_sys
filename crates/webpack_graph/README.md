# webpack_graph

A Rust crate for parsing webpack bundles and extracting module dependency graphs using **SWC AST parsing**. This crate provides robust analysis of webpack's `__webpack_modules__` structure and `__webpack_require__` function calls to reconstruct complete dependency relationships.

## Features

- **SWC-Powered Parsing**: Uses SWC's JavaScript AST parser for robust and accurate module extraction
- **Module Extraction**: Extracts all webpack modules from bundle files with their IDs and source code  
- **Dependency Analysis**: Identifies `__webpack_require__` calls to build complete module dependency relationships
- **Entry Point Detection**: Automatically identifies entry point modules using span-based AST analysis
- **Graph Operations**: Provides utilities for analyzing module reachability, dependency chains, and potential dead code
- **Tree Shaking Analysis**: Identifies unreachable modules that could be eliminated
- **Real-World Tested**: Validated against production webpack/rsbuild bundles

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
webpack_graph = { path = "./crates/webpack_graph" }
```

### Basic Example

```rust
use webpack_graph::{WebpackBundleParser, Result};

fn main() -> Result<()> {
    // Read webpack bundle
    let bundle_content = std::fs::read_to_string("path/to/bundle.js")?;
    
    // Parse the bundle using SWC
    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(&bundle_content)?;
    
    // Analyze the results
    println!("Found {} modules", graph.modules.len());
    println!("Entry points: {:?}", graph.entry_points);
    
    // Check for unreachable modules
    let unreachable = graph.get_unreachable_modules();
    if !unreachable.is_empty() {
        println!("Potential dead code: {:?}", unreachable);
    }
    
    Ok(())
}
```

### Advanced Analysis

```rust
use webpack_graph::{WebpackBundleParser, Result};

fn analyze_bundle(bundle_content: &str) -> Result<()> {
    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;
    
    // Analyze each module
    for (module_id, module) in &graph.modules {
        println!("Module {}: {} deps, {} dependents", 
                 module_id,
                 module.dependencies.len(),
                 module.dependents.len());
        
        // Get dependency chain
        let chain = graph.get_dependency_chain(module_id);
        println!("  Dependency chain: {:?}", chain);
    }
    
    // Reachability analysis
    let reachable = graph.get_reachable_modules();
    println!("Reachable modules: {}/{}", reachable.len(), graph.modules.len());
    
    Ok(())
}
```

## API Reference

### `WebpackBundleParser`

The main parser for webpack bundles, powered by SWC's JavaScript AST parser.

#### Methods

- `new() -> Result<Self>` - Create a new parser instance
- `parse_bundle(&self, source: &str) -> Result<ModuleGraph>` - Parse a webpack bundle and return the module graph

### `ModuleGraph`

Represents the complete module dependency graph.

#### Methods

- `get_module(&self, id: &str) -> Option<&ModuleNode>` - Get a module by ID
- `get_reachable_modules(&self) -> FxHashSet<String>` - Get all modules reachable from entry points
- `get_unreachable_modules(&self) -> Vec<String>` - Get modules that are not reachable (potential dead code)
- `get_dependency_chain(&self, module_id: &str) -> Vec<String>` - Get the dependency chain for a module
- `add_entry_point(&mut self, module_id: String)` - Add an entry point module

#### Fields

- `modules: FxHashMap<String, ModuleNode>` - Map of module ID to module node
- `entry_points: Vec<String>` - List of entry point module IDs

### `ModuleNode`

Represents a single module in the webpack bundle.

#### Fields

- `id: String` - The webpack module ID
- `source: String` - Raw JavaScript source code of the module
- `dependencies: FxHashSet<String>` - Direct dependencies (modules this module requires)
- `dependents: FxHashSet<String>` - Modules that depend on this module

## Examples

See the [examples](examples/) directory for complete usage examples:

- `analyze_bundle.rs` - Comprehensive bundle analysis example

Run examples with:

```bash
cargo run --example analyze_bundle
```

## How It Works

The crate uses **SWC AST parsing** for reliable and robust webpack module extraction:

### 🔧 **SWC-Based Parsing Pipeline**
1. **Parse JavaScript**: Uses SWC to parse the webpack bundle into a complete AST
2. **Locate webpack modules**: Finds `var/let/const __webpack_modules__ = ({...});` declarations
3. **Extract module definitions**: Traverses object properties to extract individual module functions
4. **Analyze dependencies**: Uses AST traversal to find `__webpack_require__(moduleId)` calls
5. **Build relationships**: Creates bidirectional dependency relationships between modules
6. **Identify entry points**: Uses span-based analysis to find entry point modules

### **Entry Point Detection Strategy**

The parser uses **span-based AST analysis** for accurate entry point detection:

#### **Primary Method: Span-Based Detection**
1. **Track __webpack_modules__ spans** during AST traversal
2. **Analyze call expression positions** relative to module definitions  
3. **Entry points**: `__webpack_require__` calls **outside** `__webpack_modules__` span
4. **Internal dependencies**: `__webpack_require__` calls **inside** `__webpack_modules__` span

```javascript
// ✅ Entry point (outside webpack_modules)
var _featureA = __webpack_require__(153);

var __webpack_modules__ = ({
  153: (function(...) {
    // ❌ NOT entry point (inside module)
    var _math = __webpack_require__(78);
  })
});
```

### 🛠️ **Supported Bundle Formats**

The parser handles various webpack bundle formats:

- **Variable declarations**: `var`, `let`, `const __webpack_modules__`
- **Object syntax**: With and without parentheses around object literals
- **Function formats**: Various webpack module function signatures
- **Module IDs**: Numeric and string-based module identifiers

## Real-World Testing

This crate is tested against **production webpack/rsbuild bundles** to ensure reliability:

### **Test Coverage**

- **Synthetic test cases**: Hand-crafted webpack bundles with known dependency structures
- **Real-world bundles**: Production rsbuild output with complex dependency graphs
- **Stats validation**: Compares parsed results against webpack's own `stats.json`
- **Edge cases**: Various webpack configurations and optimization levels

### **Validation Results**

Our test suite includes a real rsbuild project that demonstrates:

- **26 modules** successfully parsed from production bundle
- **36 dependency relationships** correctly identified
- **9 entry points** detected with **8 shared dependencies**
- **100% module coverage** - all modules reachable from entry points
- **Deep dependency chains** up to 22 levels analyzed

Run the real-world test:

```bash
# First build the test project
cd examples/rsbuild-project && pnpm install && pnpm build

# Then run validation
cd ../../crates/webpack_graph && cargo test test_real_world_rsbuild_bundle
```

## Requirements

- **Variable name**: Must be exactly `__webpack_modules__`
- **Declaration format**: Supports `var`, `let`, `const` declarations
- **Entry points**: `__webpack_require__` calls must exist outside module definitions
- **Module format**: Modules must be object properties with numeric or string keys

## Webpack Bundle Format

The parser is designed to work with webpack bundles that follow this structure:

```javascript
(() => {
"use strict";
var __webpack_modules__ = ({
  418: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    // Module 418 code here
    var dependency = __webpack_require__(78);
  }),
  422: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    // Module 422 code here  
  }),
  // ... more modules
});

// webpack runtime code...

// Entry point
(() => {
  var module_153 = __webpack_require__(153);
  var module_722 = __webpack_require__(722);
})();
})();
```

## Performance

- **Fast parsing**: SWC's optimized JavaScript parser
- **Memory efficient**: Minimal allocations during AST traversal
- **Scalable**: Handles large production bundles with hundreds of modules

## Error Handling

The crate provides comprehensive error handling:

- **Parse errors**: Invalid JavaScript syntax in bundles
- **Format errors**: Missing `__webpack_modules__` or invalid structure
- **Missing files**: Helpful error messages for missing bundle files

## License

This crate is part of the swc_macro_sys workspace. 
