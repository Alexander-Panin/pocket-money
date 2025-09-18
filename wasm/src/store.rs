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
    id: usize,
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
pub fn storage_save(obj: JsValue) {
    let mut db = Storage::new();
    match db.save(obj) {
        Ok(x) => x,
        Err(r) => console::error_1(&r),
    };
}

#[wasm_bindgen]
pub fn storage_tag(value: usize) -> String {
    let db = Storage::new();
    match db.tag(value) {
        Ok(x) => x,
        Err(r) => { console::error_1(&r); "not_found".to_owned() }
    }
}

struct Storage {
    db: web_sys::Storage 
}

impl Storage {
    fn new() -> Self {
        Self { db: window().unwrap().local_storage().unwrap().unwrap() }
    }

    fn get(&self, key: &str) -> Result<JsValue, JsValue> {
        JSON::parse(&self.db
            .get_item(key)?
            .ok_or(JsValue::from_str(&format!("not found storage[{key}]")))?)
    }

    fn set(&mut self, key: &str, value: &str) -> Result<(), JsValue> {
        self.db.set_item(key, value)
    }

    fn all(&self) -> Result<JsValue, JsValue> {
        let v = self.get("data")?;
        let v: Vec<Day> = serde_wasm_bindgen::from_value(v)?;
        serde_wasm_bindgen::to_value(&self.prepare(v))
            .map_err(|_| JsValue::from_str("failed to serialize"))
    }

    fn prepare(&self, mut v: Vec<Day>) -> Vec<(bool, Day)> {
        if v.is_empty() { return vec![]; }
        v.sort_by_key(|x| Reverse(x.date));
        let mut result = vec![];
        let mut current = 0;
        for day in v {
            let is_next = current != day.date;
            current = day.date;
            result.push( (is_next, day) );
        }
        result
    }

    fn by(&self, id: usize) -> Result<JsValue, JsValue> {
        let data = self.get("data")?;
        let v: Vec<Day> = serde_wasm_bindgen::from_value(data)?;
        let x = v.into_iter().find(|d| d.id == id).unwrap(); // todo unwrap
        serde_wasm_bindgen::to_value(&x)
            .map_err(|_| JsValue::from_str("failed to serialize"))
    }

    fn save(&mut self, day: JsValue) -> Result<(), JsValue> {
        let v = self.get("data")?;
        let mut v: Vec<Day> = serde_wasm_bindgen::from_value(v)?;
        let day: Day = serde_wasm_bindgen::from_value(day)?;
        let i = v.iter().position(|d| d.id == day.id).unwrap(); // todo unwrap
        v[i] = day;
        let x = serde_wasm_bindgen::to_value(&v)?;
        let s = &JSON::stringify(&x)?.as_string()
            .ok_or(JsValue::from_str("failed to string"))?;
        self.set("data", s)
    }

    fn tag(&self, value: usize) -> Result<String, JsValue> {
        // todo if empty
        let v = self.get("data")?;
        let v: Vec<Day> = serde_wasm_bindgen::from_value(v)?;
        let mut xs: Vec<String> = v.into_iter().map(|x| x.tag).collect();
        xs.dedup();
        xs.sort();
        Ok(xs.swap_remove(value % xs.len()))
    }
}



