## SWC Macro Sys

This crate is a macro system implementation for [swc macro proposal](https://github.com/swc-project/swc/issues/10519), which is used for parsing and transforming the javascript code based on [swc](https://github.com/swc-project/swc)

**Warning: This crate is not recommented to use now**

## Wasm binding

```sh
# Once
rustup target add wasm32-unknown-unknown

# Build the wasm binding
(cd crates/swc_macro_wasm && wasm-pack build --release)

# Your wasm file will be in `target/wasm32-unknown-unknown/release/swc_macro_wasm.wasm`
```

## Examples

### Tree-Shaking Demo

Test conditional compilation and tree-shaking with a webpack bundle:

```sh
# Build WASM module first
(cd crates/swc_macro_wasm && wasm-pack build --release)

# Run tree-shaking test on bundler output
node --experimental-wasm-modules test-tree-shaking.js
```

This demonstrates:
- **Conditional compilation** - Code blocks removed based on feature flags
- **Bundle optimization** - Minification and dead code elimination  
- **Multiple configurations** - Tests different feature flag combinations
- **Size analysis** - Shows bundle size reduction for each configuration

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


