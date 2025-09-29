use wasm_bindgen::prelude::*;
use web_sys::{Element};

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

#[wasm_bindgen]
pub fn target(mut node: Option<Element>) -> Option<Element> {
    while let Some (ref x) = node {
        let attrs = x.attributes();
        if let Some(_) = attrs.get_named_item("__action") {
            return node;
        }
        node = x.parent_element(); 
    }
    None
}