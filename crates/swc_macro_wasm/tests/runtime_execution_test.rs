use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use serde_json::json;
use swc_macro_wasm::optimize;

fn node_available() -> bool {
    Command::new("node").arg("-v").output().is_ok()
}

fn write_temp_js(contents: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    let filename = format!("swc_macro_wasm_rt_{}.js", uuid::Uuid::new_v4());
    p.push(filename);
    let mut f = fs::File::create(&p).expect("create temp js file");
    f.write_all(contents.as_bytes()).expect("write temp js");
    p
}

#[test]
fn test_execute_optimized_cjs_chunk_with_node_runtime_stub() {
    if !node_available() {
        eprintln!("Skipping runtime execution test: node not found");
        return;
    }

    // Minimal CJS-style split chunk that does not rely on __webpack_require__.r/d helpers
    let cjs_chunk = r#"
"use strict";
exports.ids = ["vendors-test"];
exports.modules = {
  "a.js": function(module, exports, __webpack_require__) {
    exports.a = "A";
  },
  "b.js": function(module, exports, __webpack_require__) {
    var a = __webpack_require__("a.js");
    exports.b = "B" + a.a;
  },
  "entry.js": function(module, exports, __webpack_require__) {
    var b = __webpack_require__("b.js");
    module.exports = { vv: b.b };
  }
};
"#;

    // Provide required ChunkCharacteristics strictly via treeShake payload
    let config = json!({
        "treeShake": {
            "test": {
                "chunk_characteristics": {
                    "entry_module_id": "entry.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "async-node",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["vendors-test.js"],
                    "is_shared_chunk": false,
                    "shared_modules": []
                }
            }
        }
    });

    let optimized = optimize(cjs_chunk.to_string(), &config.to_string());

    // Compose a Node runner script that evaluates the optimized code in an IIFE with a local `exports`
    // then wires a minimal __webpack_require__ to execute the entry and validate behavior
    let runner = format!(
        r#"(function() {{
  var exports = {{}};
  // Optimized bundle
  {optimized}
  if (!exports.modules) throw new Error('modules object missing');
  globalThis.__modules = exports.modules;
}})();

const __cache = Object.create(null);
function __webpack_require__(id) {{
  if (__cache[id]) return __cache[id].exports;
  const fn = globalThis.__modules[id];
  if (!fn) throw new Error('Module not found: '+id);
  const module = {{ exports: {{}} }};
  __cache[id] = module;
  fn(module, module.exports, __webpack_require__);
  return module.exports;
}}

const result = __webpack_require__('entry.js');
if (!result || result.vv !== 'BA') {{
  console.error('Unexpected result', result);
  process.exit(1);
}}
console.log('OK');
process.exit(0);
"#,
        optimized = optimized
    );

    let script_path = write_temp_js(&runner);
    let out = Command::new("node")
        .arg(script_path.as_os_str())
        .output()
        .expect("run node");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    eprintln!("node stdout: {}", stdout);
    eprintln!("node stderr: {}", stderr);
    assert!(out.status.success(), "node runner failed");
    assert!(stdout.contains("OK"));

    // Cleanup
    let _ = fs::remove_file(script_path);
}


