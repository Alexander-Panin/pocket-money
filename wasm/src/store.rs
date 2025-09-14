use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys::{JSON};
use web_sys::{window, console};


//  let price = &format!("{:.0},", day.price.floor());
// -    node.set_text_content(Some(&price));
// -    let node = content
// -        .query_selector("#money2")?
// -        .ok_or(JsValue::from_str("no #money2 in row-template"))?;
// -    let diff = day.price.round() == day.price;
// -    let price = if diff { " " }
// -        else { &format!("{}", ((day.price - day.price.floor()) * 100.0).round() ) };
// -    node.set_text_content(Some(price));

#[wasm_bindgen]
pub fn storage_by_id(row_id: &str) -> JsValue {
    match _storage_by_id(row_id) {
        Ok(x) => x,
        Err(r) => { console::error_1(&r); JsValue::null() }
    }
}

pub fn _storage_by_id(row_id: &str) -> Result<JsValue, JsValue> {
    let value = storage("data")?;
    let x = JSON::parse(&value)?;
    let xs: Vec<Day> = serde_wasm_bindgen::from_value(x)?;
    let id = row_id.parse::<usize>()
        .map_err(|_| JsValue::from_str(&format!("can not parse row_id[{row_id}]")))?;
    serde_wasm_bindgen::to_value(&xs[id])
        .map_err(|_| JsValue::from_str("can not serialize object"))
}

#[wasm_bindgen]
pub fn storage_all() -> JsValue {
    let data = storage("data").unwrap();
    JSON::parse(&data).unwrap()
}

#[derive(Deserialize, Serialize, Debug)]
struct Day {
    date: u32,
    price: f32,
    tag: String,
    comment: String,
}

fn storage(key: &str) -> Result<String, JsValue> {
    Ok(window()
        .ok_or(JsValue::from_str("not found window"))?
        .local_storage()?
        .ok_or(JsValue::from_str("not found local storage"))?
        .get_item(key)?
        .ok_or(JsValue::from_str(&format!("not found storage[{key}]")))?)
}
