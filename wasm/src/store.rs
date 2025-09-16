use std::cmp::Reverse;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use js_sys::{JSON};
use web_sys::{window, console};

#[derive(Deserialize, Serialize, Debug)]
struct Day {
    date: u32,
    price: f32,
    tag: String,
    comment: String,
}

#[wasm_bindgen]
pub fn storage_by(id: usize) -> JsValue {
    let db = Storage::new();
    match db.by(id) {
        Ok(x) => x,
        Err(r) => { console::error_1(&r); JsValue::null() }
    }
}

#[wasm_bindgen]
pub fn storage_all() -> JsValue {
    let db = Storage::new();
    match db.all() {
        Ok(x) => x,
        Err(r) => { console::error_1(&r); JsValue::null() }
    }
}

#[wasm_bindgen]
pub fn storage_save(id: usize, price: f32) {
    let mut db = Storage::new();
    match db.save(id, price) {
        Ok(x) => x,
        Err(r) => console::error_1(&r),
    };
}

struct Storage {
    db: web_sys::Storage 
}

impl Storage {
    fn new() -> Self {
        Self { db: window().unwrap().local_storage().unwrap().unwrap() }
    }

    fn get(&self, key: &str) -> Result<String, JsValue> {
        self.db
            .get_item(key)?
            .ok_or(JsValue::from_str(&format!("not found storage[{key}]")))
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), JsValue> {
        self.db.set_item(key, value)
    }

    fn all(&self) -> Result<JsValue, JsValue> {
        let data = self.get("data")?;
        let x = JSON::parse(&data)?;
        let v: Vec<Day> = serde_wasm_bindgen::from_value(x)?;
        serde_wasm_bindgen::to_value(&self.prepare(v))
            .map_err(|_| JsValue::from_str("failed to serialize"))
    }

    fn prepare(&self, xs: Vec<Day>) -> Vec<(bool, usize, Day)> {
        let mut v: Vec<_> = vv.into_iter().zip(0..).collect();
        v.sort_by_key(|x| Reverse(x.0.date));
        let mut result = vec![];
        let mut current = 0;
        for (day, i) in v {
            let r = current != day.date;
            current = day.date;
            result.push( (r, i, day) );
        }
        result
    }

    fn by(&self, id: usize) -> Result<JsValue, JsValue> {
        let x = JSON::parse(&self.get("data")?)?;
        let v: Vec<Day> = serde_wasm_bindgen::from_value(x)?;
        serde_wasm_bindgen::to_value(&v[id])
            .map_err(|_| JsValue::from_str("failed to serialize"))
    }

    fn save(&mut self, id: usize, price: f32) -> Result<(), JsValue> {
        let obj = JSON::parse(&self.get("data")?)?;
        let mut v: Vec<Day> = serde_wasm_bindgen::from_value(obj)?;
        v[id].price = price;
        let x = serde_wasm_bindgen::to_value(&v)?;
        let s = &JSON::stringify(&x)?.as_string()
            .ok_or(JsValue::from_str("failed to string"))?;
        self.set("data", s)
    }
}



