use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foobar() -> JsValue {
    JsValue::NULL
}

#[wasm_bindgen]
pub fn cent(price: f32) -> String {
    if price.round() == price { 
        " ".to_owned() 
    } else { 
        format!("{}", ((price - price.floor()) * 100.0).round()) 
    }
}

#[wasm_bindgen]
pub fn euro(price: f32) -> String {
    format!("{:.0},", price.floor()) 
}

#[wasm_bindgen]
pub fn money(price: f32) -> String {
    format!("{:1}", price) 
}
