use std::cmp::Reverse;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use web_sys::{window, console};

// #[wasm_bindgen]
// pub fn storage_by(id: usize) -> JsValue {
//     let db = Storage::new();
//     match db.by(id) {
//         Ok(x) => x,
//         Err(r) => { console::error_1(&r); JsValue::null() }
//     }
// }

// #[wasm_bindgen]
// pub fn storage_all() -> JsValue {
//     let db = Storage::new();
//     match db.all() {
//         Ok(x) => x,
//         Err(r) => { console::error_1(&r); JsValue::null() }
//     }
// }

// #[wasm_bindgen]
// pub fn storage_save(obj: JsValue) {
//     let mut db = Storage::new();
//     match db.save(obj) {
//         Ok(x) => x,
//         Err(r) => console::error_1(&r),
//     };
// }

// #[wasm_bindgen]
// pub fn storage_tag(value: usize) -> String {
//     let db = Storage::new();
//     match db.tag(value) {
//         Ok(x) => x,
//         Err(r) => { console::error_1(&r); "not_found".to_owned() }
//     }
// }

#[wasm_bindgen]
pub struct Day<'a> {
    pub date: u32,
    pub price: f32,
    tag: String,
    comment: String,
    pub id: usize,
}

#[wasm_bindgen]
pub struct Store {}

#[wasm_bindgen]
impl Store {
    fn db() -> web_sys::Storage {
        window().unwrap().local_storage().unwrap().unwrap()
    }

    fn set(key: &str, value: &str) -> Result<(), JsValue> {
        Self::db().set_item(key, value)
    }

    fn get(key: &str) -> Result<String, JsValue> {
        Store::db()
            .get_item(key)?
            .ok_or(JsValue::from_str(&format!("not found storage[{key}]")))
    }

    pub fn get_by(id: usize, key: &str) -> Result<String, JsValue> {
        Self::get(&format!("{id}:{key}"))
    }

    fn root() -> Result<usize, JsValue> {
        let root = Self::get("data:root")?;
        Ok(root.parse::<usize>().unwrap())
    }  

    fn value<T>(id: usize, key: &str) -> Result<T, JsValue> 
        where T: FromStr, <T as FromStr>::Err: std::fmt::Debug
    {
        Ok(Self::get(&format!("data:{id}:{key}"))?.parse::<T>().unwrap())
    } 

    pub fn one(id: usize) -> Result<Day, JsValue> {
        let price: f32 = Self::value(id, "price")?;
        let date: u32  = Self::value(id, "date")?;
        let tag: String  = Self::get_by(id, "tag")?;
        let comment: String  = Self::get_by(id, "comment")?;
        Ok(Day { price, date, tag, id, comment })
    }

    fn all() -> Result<Vec<Day>, JsValue> {
        let mut root = Self::root()?;
        let mut result = vec![Self::one(root)?];
        while let Ok(next) = Self::value(root, "next") {
            result.push(Self::one(next)?);
            root = next;
        }
        Ok(result)
    }

    fn save_price(id: usize, price: f32) -> Result<(), JsValue> 
        { Self::set(&format!("data:{id}:price"), &price.to_string()) }

    fn save_date(id: usize, date: u32) -> Result<(), JsValue> 
        { Self::set(&format!("data:{id}:date"), &date.to_string()) }

    fn save_tag(id: usize, tag: &str) -> Result<(), JsValue> 
        { Self::set(&format!("data:{id}:tag"), tag) }

    fn save_comment(id: usize, comment: &str) -> Result<(), JsValue> 
        { Self::set(&format!("data:{id}:comment"), comment) }

    fn prepare(&self, mut days: Vec<Day>) -> Vec<(bool, Day)> {
        days.sort_by_key(|x| Reverse(x.date));
        days.into_iter().scan(0, |state, x| {
            let is_next = *state != x.date;
            *state = x.date;
            Some((is_next, x))
        }).collect()
    }

    fn tags() -> Result<Vec<String>, JsValue> {
        Ok(Self::all()?.into_iter().map(|x| x.tag).collect())
    }
}
