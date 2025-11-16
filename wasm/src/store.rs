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
pub async fn save_comment(id: &JsValue, comment: &JsValue) -> Result<(), JsValue> {
    write(id, &"comment".into(), comment).await
}

#[wasm_bindgen]
pub async fn save_tag(id: &JsValue, tag: &JsValue) -> Result<(), JsValue> {
    write(id, &"tag".into(), tag).await
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

    pub async fn fetch(id: &JsValue) -> Option<Self> {
        Some(Day {
            price: read(id, &"price".into()).await.ok()
                .map(|x| x.unchecked_into_f64() as f32)
                .unwrap_or(0.0),
            date: read(id, &"date".into()).await.ok()
                .map(|x| x.unchecked_into_f64() as i32)
                .unwrap_or(1),
            tag: read(id, &"tag".into()).await.ok()
                .unwrap_or("".into()),
            comment: read(id, &"comment".into()).await.ok()
                .unwrap_or("".into()),
            id: id.clone(),
        })
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct Row(pub bool, pub Day);

#[wasm_bindgen]
pub struct Stats { pub last_date: i32 }

#[wasm_bindgen]
pub struct Store {}

async fn store(ns: &JsValue) -> Option<Vec<Day>> {
    let mut p = read(&ns, &"root".into()).await;
    let mut v = vec![];
    while let Ok(id) = p {
        v.push( Day::fetch(&id).await? );
        p = read(&id, &"next".into()).await;
    }
    Some(v)
}

#[wasm_bindgen]
impl Store {

    async fn all(ns: &JsValue) -> Option<Vec<Day>> {
        Self::all_with(ns, |x| x.date >= 0).await
    }

    async fn all_with<F: FnMut(&Day) -> bool>(ns: &JsValue, f: F) -> Option<Vec<Day>> {
        let mut v = store(ns).await?; v.retain(f); Some(v)
    }

    pub async fn append(ns: &JsValue, id: &JsValue) {
        if let Ok(root) = read(ns, &"root".into()).await {
            let _ = write(id, &"next".into(), &root).await;
        }
        let _ = write(ns, &"root".into(), id).await;
    }

    pub async fn tags(ns: &JsValue) -> Option<Vec<JsValue>> {
        Some(Self::all(ns).await?.into_iter().map(|x| x.tag).collect())
    }

    pub async fn select(ns: &JsValue) -> Option<Vec<Row>> {
        let mut days = Self::all(ns).await?;
        days.sort_by_key(|x| std::cmp::Reverse(x.date));
        Some(days.into_iter().scan(-1, |state, x| {
            let is_next = *state != x.date;
            *state = x.date;
            Some(Row(is_next, x))
        }).collect())
    }

    pub async fn stats(ns: &JsValue) -> Option<Stats> {
        let mut days = Store::all_with(ns, |x| x.date > 0).await?;
        days.sort_by_key(|x| std::cmp::Reverse(x.date));
        Some(Stats { last_date: days.first()?.date })
    }

    pub async fn sum(ns: &JsValue) -> Option<f32> {
        let days = Store::all(ns).await?;
        Some(days.into_iter().map(|x| x.price).sum::<f32>().round())
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