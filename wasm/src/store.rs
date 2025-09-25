use std::str::FromStr;
use wasm_bindgen::prelude::*;
use web_sys::{window, console};

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
impl Day {
    pub fn save_price(&mut self, price: f32)
    { self.price = price; Store::save_price(self.id, price); }

    pub fn save_date(&mut self, date: u32)
    { self.date = date; Store::save_date(self.id, date); }

    pub fn save_tag(&mut self, tag: String)
    { Store::save_tag(self.id, &tag); self.tag = tag; }

    pub fn save_comment(&mut self, comment: String)
    { Store::save_comment(self.id, &comment); self.comment = comment;  }
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
            Ok(_) => (),
            _ => {
                let tmp = &format!("can not saved storage[{key}]");
                console::error_1(&JsValue::from_str(tmp));
            },
        }
    }

    fn get(key: &str, log: bool) -> Option<String> {
        match Self::db().get_item(key) {
            Ok(Some(x)) => Some(x),
            _ => {
                let tmp = &format!("not found storage[{key}]");
                if log { console::error_1(&JsValue::from_str(tmp)); }
                None
            },
        }
    }

    fn root() -> Option<usize> {
        let root = Self::get("data:root", true)?;
        root.parse::<usize>().ok()
    }  

    fn num<T>(id: usize, key: &str, log: bool) -> Option<T> 
        where T: FromStr, <T as FromStr>::Err: std::fmt::Debug
    {
        Self::get(&format!("data:{id}:{key}"), log)?.parse::<T>().ok()
    }    

    fn value(id: usize, key: &str) -> Option<String> {
        Self::get(&format!("data:{id}:{key}"), true)
    } 

    pub fn one(id: usize) -> Option<Day> {
        let price: f32 = Self::num(id, "price", true)?;
        let date: u32  = Self::num(id, "date", true)?;
        let tag: String  = Self::value(id, "tag")?;
        let comment: String  = Self::value(id, "comment")?;
        Some(Day { price, date, tag, id, comment })
    }

    pub fn all() -> Option<Vec<Day>> {
        let mut root = Self::root()?;
        let mut result = vec![Self::one(root)?];
        while let Some(next) = Self::num(root, "next", false) {
            result.push(Self::one(next)?);
            root = next;
        }
        Some(result)
    }

    pub fn save_price(id: usize, price: f32) 
        { Self::set(&format!("data:{id}:price"), &price.to_string()) }

    pub fn save_date(id: usize, date: u32) 
        { Self::set(&format!("data:{id}:date"), &date.to_string()) }

    // todo, save tags for future use
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
