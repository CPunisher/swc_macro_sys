## SWC Macro Sys

This project is a comprehensive macro system implementation for [swc macro proposal](https://github.com/swc-project/swc/issues/10519), which is used for parsing and transforming JavaScript code based on [swc](https://github.com/swc-project/swc). It includes advanced webpack tree shaking capabilities specifically designed for Module Federation and modern bundler outputs.

**Key Features:**
- **Advanced Tree Shaking**: Reduces bundle sizes by 30-70% for typical applications
- **Module Federation Support**: Optimized for micro-frontend architectures
- **AST-Based Analysis**: Uses SWC for precise JavaScript parsing and dependency tracking
- **Production Ready**: Tested with real-world webpack/rspack bundles

## Build & Setup

### Quick Start

```bash
# Install dependencies
pnpm install

# Build all packages (Rust + WASM + JavaScript)
pnpm build

# Run all tests
pnpm test

# Run tests in CI mode
pnpm test:ci
```

### WASM binding

```sh
# Once: Add WASM target
rustup target add wasm32-unknown-unknown

# Build the WASM binding
pnpm build:wasm

# Or build everything
pnpm build

# Your wasm file will be in `crates/swc_macro_wasm/pkg/`
```

**Requirements:**
- Node.js v20+ recommended for best WASM support
- Use `--experimental-wasm-modules` flag for WASM optimization to work

## Project Structure

```
swc_macro_sys/
├── crates/
│   ├── swc_macro_condition_transform/  # Conditional macro transformation
│   ├── swc_macro_parser/               # Macro parsing capabilities
│   ├── swc_macro_wasm/                 # WASM bindings + tree shaking implementation
│   └── webpack_analyzer_v2/            # Pure webpack chunk analysis library
├── examples/                           # Various usage examples
│   └── module-federation-example/      # Complete Module Federation demo
├── test-cases/                         # Real-world test bundles
└── TESTING.md                          # Comprehensive testing guide
```

### Core Crates

- **`swc_macro_condition_transform`**: Conditional macro transformation for compile-time optimizations
- **`swc_macro_parser`**: Macro parsing capabilities for JavaScript/TypeScript code
- **`swc_macro_wasm`**: WASM bindings with complete tree shaking and optimization implementation
- **`webpack_analyzer_v2`**: Pure analysis library for webpack chunks (parsing, dependency graphs)

> **Note**: The `webpack_chunk_tree_shaker` crate has been removed. All tree shaking logic is now consolidated in `swc_macro_wasm/src/optimize.rs` for better maintainability.

## Examples

### Module Federation with Tree Shaking

Complete example demonstrating Module Federation with advanced tree shaking:

```bash
# Navigate to the example
cd examples/module-federation-example

# Install dependencies
pnpm install

# Build and optimize shared chunks
pnpm build:optimized

# Run the demo applications
pnpm dev
```

This demonstrates:
- **Module Federation setup** with lodash-es sharing
- **Tree shaking optimization** reducing bundle size by 30-70%
- **Real-world usage** with host/remote applications
- **Bundle analysis** with detailed size reduction reports

### Tree-Shaking Demo

Test tree-shaking with real webpack bundles:

```bash
# Build WASM module first (included in pnpm build)
pnpm build

# Run optimization on test cases
node --experimental-wasm-modules scripts/optimize-shared-chunks.js
```

This demonstrates:
- **AST-based analysis** using SWC for precise module detection
- **Dependency tracking** with webpack require analysis
- **Multiple webpack formats** (CommonJS, JSONP, WebpackModules)
- **Size analysis** showing bundle size reduction metrics

### Rust Transform Example

Check `crates/swc_macro_condition_transform` to see how this crate works to handle the macro annotations.

Run `cargo run --example transform` with the following input javascript code:

```js
/* @common:if [condition="featureFlags.enableNewFeature"] */
export function newFeature() {
  return "New feature is enabled!";
}
/* @common:endif */

const buildTarget =
  /* @common:define-inline [value="build.target" default="development"] */ "development";
```

The expected output is:

```js
const buildTarget = "production";
```

### Node.js JSX Demo

The `examples/jsx-test-server.mjs` demonstrates:

- **JSX Transformation**: Using SWC to transform JSX syntax to React.createElement calls
- **Macro Processing**: Applying conditional compilation and variable substitution
- **Component Rendering**: Server-side rendering of React components to HTML

Features demonstrated:
- Complex nested conditional blocks (`@common:if`/`@common:endif`)
- Platform-specific code paths (mobile/desktop)
- Feature flag conditional compilation
- A/B testing variants
- User type-based feature access
- Inline variable substitution (`@common:define-inline`)

Run the demo to see how the macro system can optimize bundle size by eliminating unused code paths at build time.

## Testing

This project includes comprehensive testing across Rust and JavaScript:

```bash
# Run all tests
pnpm test

# Run tests with coverage
pnpm test:coverage

# Run tests in watch mode
pnpm test:watch

# Run specific test types
pnpm test:unit        # Unit tests
pnpm test:integration # Integration tests
pnpm test:e2e         # End-to-end tests
```

For detailed testing information, see [TESTING.md](TESTING.md).

## Performance Results

Real-world performance improvements from the test suite:

| Bundle Type | Original Size | Optimized Size | Reduction |
|-------------|---------------|----------------|-----------|
| Lodash-ES Vendor | 5.3KB | 1.7KB | 67.9% |
| Module Federation | 1.4MB | 12.2KB | 99.2% |
| RSpack Bundle | 1.2MB | 69KB | 94.3% |

## API Reference

### JavaScript Usage (WASM)

```javascript
import { optimize } from 'swc_macro_wasm';

const optimizedCode = optimize(webpackBundle, {
  treeShake: {
    "lodash-es": {
      chunk_characteristics: { entry_module_id: "../../node_modules/lodash-es/lodash.js" }
    }
  }
    });
```

### Rust Usage

```rust
use webpack_analyzer_v2::tree_shaker::TreeShaker;
use webpack_analyzer_v2::{WebpackAnalyzer, chunk::ChunkCharacteristics};

let analyzer = WebpackAnalyzer::new();
let characteristics = ChunkCharacteristics { chunk_format: "require".into(), entry_module_id: Some("../../node_modules/lodash-es/lodash.js".into()), ..Default::default() };
let mut chunk = analyzer.analyze_chunk(&webpack_source, characteristics)?;
let shaker = TreeShaker::new();
let optimized_source = shaker.prune_source(&chunk.source, chunk.characteristics.clone())?;
```

