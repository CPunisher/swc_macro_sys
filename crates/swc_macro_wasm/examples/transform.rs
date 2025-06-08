use std::fs;

use serde_json::json;

pub fn main() {
    let path = std::env::args().nth(1).unwrap_or("test.js".to_owned());
    let source = fs::read_to_string(path).unwrap();
    let config = json!({
        "build": {
            "target": "production"
        },
        "device": {
            "isMobile": false
        },
        "user": {
            "language": "en",
            "isLoggedIn": true
        },
        "experiment": {
            "group": "B"
        },
        "featureFlags": {
            "newMobileUI": true,
            "enableNewFeature": false,
            "newUserProfile": false
        }
    });

    let ret = swc_macro_wasm::optimize::optimize(source, config);
    println!("{}", ret);
}
