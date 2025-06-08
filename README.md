## SWC Macro Sys

This crate is a macro system implementation for [swc macro proposal](https://github.com/swc-project/swc/issues/10519), which is used for parsing and transforming the javascript code based on [swc](https://github.com/swc-project/swc)

**Warning: This crate is not recommented to use now**

## Build & Setup

### WASM binding

```sh
# Once: Add WASM target
rustup target add wasm32-unknown-unknown

# Build the WASM binding
(cd crates/swc_macro_wasm && wasm-pack build --release)

# Your wasm file will be in `crates/swc_macro_wasm/pkg/`
```

### Node.js Examples

```sh
# Install Node.js dependencies for examples
(cd examples && npm install)

# Run the JSX transformation demo
(cd examples && npm run jsx-demo)
# OR
(cd examples && node --experimental-wasm-modules jsx-test-server.mjs)
```

**Requirements:**
- Node.js v20+ recommended for best WASM support
- Use `--experimental-wasm-modules` flag for WASM optimization to work

## Examples

### Rust Examples

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


