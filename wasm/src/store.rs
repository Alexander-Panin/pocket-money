use wasm_bindgen::prelude::*;
use uuid::Uuid;
use crate::opfs::{read, write};

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone, Default)]
pub struct Day {
    pub date: i32,
    pub price: f32,
    pub tag: JsValue,
    pub comment: JsValue,
    pub id: JsValue,
}

#[wasm_bindgen]
pub async fn save_price(id: &JsValue, price: &JsValue) -> Result<(), JsValue> {
    write(id, &"price".into(), price).await
}

#[wasm_bindgen]
pub async fn save_date(id: &JsValue, date: &JsValue) -> Result<(), JsValue> {
    write(id, &"date".into(), date).await
}

#[wasm_bindgen]
pub async fn save_tag(id: &JsValue, tag: &JsValue) -> Result<(), JsValue> {
    write(id, &"tag".into(), tag).await
}

#[wasm_bindgen]
pub async fn save_comment(id: &JsValue, comment: &JsValue) -> Result<(), JsValue> {
    write(id, &"comment".into(), comment).await
}

async fn write_all(x: &Day) {
    let id = &x.id;
    let _ = write(id, &"price".into(), &x.price.to_string().into()).await;
    let _ = write(id, &"date".into(), &x.date.to_string().into()).await;
    let _ = write(id, &"tag".into(), &x.tag).await;
    let _ = write(id, &"comment".into(), &x.comment).await;
}

#[wasm_bindgen]
impl Day {

    pub fn new() -> Self {
        Self { 
            id: Uuid::new_v4().to_string().into(), 
            date: 1,
            ..Self::default()
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub async fn fetch(id: &JsValue) -> Self {
        Day {
            price: read(id, &"price".into()).await
                .map(|x| x.unchecked_into_f64() as f32)
                .unwrap_or(0.0),
            date: read(id, &"date".into()).await
                .map(|x| x.unchecked_into_f64() as i32)
                .unwrap_or(1),
            tag: read(id, &"tag".into()).await
                .unwrap_or("".into()),
            comment: read(id, &"comment".into()).await
                .unwrap_or("".into()),
            id: id.clone(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Row(pub bool, pub Day);

#[wasm_bindgen]
pub struct Stats { pub last_date: i32 }

#[wasm_bindgen]
pub struct Store {}

async fn store(ns: &JsValue) -> Vec<Day> {
    let mut p = read(&ns, &"root".into()).await;
    let mut result = vec![];
    while let Ok(id) = p {
        result.push( Day::fetch(&id).await );
        p = read(&id, &"next".into()).await;
    }
    result
}

#[derive(PartialEq)]
#[wasm_bindgen]
pub enum Sort { Asc, Desc }

#[wasm_bindgen]
impl Store {

    async fn all(ns: &JsValue) -> Vec<Day> {
        Self::all_with(ns, |x| x.date >= 0).await
    }    

    async fn all_with<F: FnMut(&Day) -> bool>(ns: &JsValue, f: F) -> Vec<Day> {
        let mut v = store(ns).await; v.retain(f); v
    }

    pub async fn append(ns: &JsValue, id: &JsValue) {
        if let Ok(root) = read(ns, &"root".into()).await {
            let _ = write(id, &"next".into(), &root).await;
        }
        let _ = write(ns, &"root".into(), id).await;
    }

    pub async fn tags(ns: &JsValue) -> Vec<JsValue> {
        Self::all(ns).await.into_iter().map(|x| x.tag).collect()
    }

    fn sort(mut days: Vec<Day>, ordering: Sort) -> Vec<Day> {
        days.sort_by(match ordering {
            Sort::Asc => |x: &Day, y: &Day| x.date.cmp(&y.date),
            Sort::Desc => |x: &Day, y: &Day| y.date.cmp(&x.date),
        });
        days
    }

    pub async fn regular(ns: &JsValue) -> Vec<Day> {
        Self::all_with(ns, |x| x.date == 0).await
    }

    async fn copy(ns: &JsValue, day: Day) -> Day {
        let x = Day{ id: Uuid::new_v4().to_string().into(), ..day }; 
        Self::append(ns, &x.id).await; 
        write_all(&x).await;
        return x;
    }

    pub async fn repeat_regular(ns: &JsValue, prev_ns: &JsValue) -> Vec<Day> {
        let mut result = vec![];
        for x in Self::regular(prev_ns).await {
            result.push(Self::copy(ns, x).await); 
        }
        result
    }

    pub fn transform(days: Vec<Day>) -> Vec<Row> {
        days.into_iter().scan(-1, |state, x| {
            let is_next = *state != x.date;
            *state = x.date;
            Some(Row(is_next, x))
        }).collect()
    }

    pub async fn select(ns: &JsValue, ordering: Sort) -> Vec<Row> {
        let days = Self::sort(Self::all(ns).await, ordering);
        Self::transform(days) 
    }

    pub async fn stats(ns: &JsValue) -> Option<Stats> {
        let mut days = Store::all_with(ns, |x| x.date > 0).await;
        days.sort_by_key(|x| std::cmp::Reverse(x.date));
        Some(Stats { last_date: days.first()?.date })
    }

    pub async fn sum(ns: &JsValue) -> f32 {
        let days = Store::all(ns).await;
        days.into_iter().map(|x| x.price).sum::<f32>().round()
    }
}














#[cfg(test)]
mod tests {

use super::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
async fn start() {
    let xs = [1.0, 10.0, 100.0];
    for x in 1..50 {
        let mut d = Day::new();
        d.date = x as i32;
        let _ = save_date(&d.id, &d.date.to_string().into()).await;
        let _ = save_price(&d.id, &xs[x % xs.len()].to_string().into()).await;
        Store::append(&"2025:august".into(), &d.id).await;
    }
}

}