## SWC Macro Sys

This crate is a macro system implementation for [swc macro proposal](https://github.com/swc-project/swc/issues/10519), which is used for parsing and transforming the javascript code based on [swc](https://github.com/swc-project/swc)

**Warning: This crate is not recommented to use now**

## Examples

Check `crates/swc_macro_condition_transform` to see how this crate works to handle the macro annotations.

Run `cargo run --example transform` with the following input javascript code:

```js
/* @swc:if [condition="featureFlags.enableNewFeature"] */
export function newFeature() {
  return "New feature is enabled!";
}
/* @swc:endif */

const buildTarget =
  /* @swc:define-inline [value="build.target" default="development"] */ "development";
```

The expected output is:

```js
const buildTarget = "production";
```
