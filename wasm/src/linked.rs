use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use crate::opfs::{read};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

async fn next_id(id: &JsValue, set: &mut HashSet<String>) -> Result<JsValue, JsValue> {
    if !set.insert(id.as_string().unwrap()) { 
        alert("Corrupted data - cycle detected"); 
        return Err(JsValue::NULL);
    }
    read(&id, &"next".into()).await
} 

pub async fn collect_ids(ns: &JsValue) -> Vec<JsValue> {
    let mut result = vec![];
    let mut set: HashSet<String> = HashSet::new();
    let mut p = read(&ns, &"root".into()).await;
    while let Ok(id) = p {
        p = next_id(&id, &mut set).await;
        result.push(id); 
    }
    result
}