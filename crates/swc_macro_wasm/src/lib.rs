use wasm_bindgen::prelude::*;

mod dce;
pub mod optimize;

#[wasm_bindgen]
pub fn optimize(source: String, config: &str) -> String {
    let config: serde_json::Value =
        serde_json::from_str(config).expect("invalid config: must be a json object");
    optimize::optimize(source, config)
}
