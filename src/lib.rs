
// wasm-bingen is used to communicate b/n Rust & JS
use wasm_bindgen::prelude::*;

// Calling external functions
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}