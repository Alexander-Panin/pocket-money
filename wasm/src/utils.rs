use wasm_bindgen::prelude::*;
use crate::store::store;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let x = store();
    alert(&format!("Hello {}, {:?}!", name, x));
}