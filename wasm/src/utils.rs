use wasm_bindgen::prelude::*;
use web_sys::{Element};

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

