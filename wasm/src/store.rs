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

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Day {
    pub date: u32,
    pub price: f32,
    pub tag: String,
    pub comment: String,
    pub id: usize,
}

#[wasm_bindgen]
pub struct Store {}

#[wasm_bindgen(getter_with_clone)]
pub struct Row(pub bool, pub Day);

#[wasm_bindgen]
impl Store {
    fn db() -> web_sys::Storage {
        window().unwrap().local_storage().unwrap().unwrap()
    }

    fn set(key: &str, value: &str) {
        match Self::db().set_item(key, value) {
            Err(_) => {
                let tmp = &format!("can not saved storage[{key}]");
                console::error_1(&JsValue::from_str(tmp));
            },
            _ => (),
        }
    }

    fn get(key: &str) -> Option<String> {
        match Self::db().get_item(key) {
            Err(_) => {
                let tmp = &format!("not found storage[{key}]");
                console::error_1(&JsValue::from_str(tmp));
                None
            },
            Ok(x) => x,
        }
    }

    pub fn get_by(id: usize, key: &str) -> Option<String> {
        Self::get(&format!("{id}:{key}"))
    }

    fn root() -> Option<usize> {
        let root = Self::get("data:root")?;
        root.parse::<usize>().ok()
    }  

    fn value<T>(id: usize, key: &str) -> Option<T> 
        where T: FromStr, <T as FromStr>::Err: std::fmt::Debug
    {
        Self::get(&format!("data:{id}:{key}"))?.parse::<T>().ok()
    } 

    pub fn one(id: usize) -> Option<Day> {
        let price: f32 = Self::value(id, "price")?;
        let date: u32  = Self::value(id, "date")?;
        let tag: String  = Self::get_by(id, "tag")?;
        let comment: String  = Self::get_by(id, "comment")?;
        Some(Day { price, date, tag, id, comment })
    }

    pub fn all() -> Option<Vec<Day>> {
        let mut root = Self::root()?;
        let mut result = vec![Self::one(root)?];
        while let Some(next) = Self::value(root, "next") {
            result.push(Self::one(next)?);
            root = next;
        }
        Some(result)
    }

    pub fn save_price(id: usize, price: f32) 
        { Self::set(&format!("data:{id}:price"), &price.to_string()) }

    pub fn save_date(id: usize, date: u32) 
        { Self::set(&format!("data:{id}:date"), &date.to_string()) }

    pub fn save_tag(id: usize, tag: &str) 
        { Self::set(&format!("data:{id}:tag"), tag) }

    pub fn save_comment(id: usize, comment: &str) 
        { Self::set(&format!("data:{id}:comment"), comment) }

    pub fn prepare(mut days: Vec<Day>) -> Vec<Row> {
        days.sort_by_key(|x| std::cmp::Reverse(x.date));
        days.into_iter().scan(0, |state, x| {
            let is_next = *state != x.date;
            *state = x.date;
            Some(Row(is_next, x))
        }).collect()
    }

    pub fn tags() -> Option<Vec<String>> {
        Some(Self::all()?.into_iter().map(|x| x.tag).collect())
    }
}
