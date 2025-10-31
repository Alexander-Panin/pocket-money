use wasm_bindgen::prelude::*;
use uuid::Uuid;
use crate::opfs::{read, write};

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Day {
    pub date: i32,
    pub price: f32,
    pub tag: JsValue,
    pub comment: JsValue,
    pub id: JsValue,
}

#[wasm_bindgen]
impl Day {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { 
            id: Uuid::new_v4().to_string().into(), 
            date: 1,
            price: 0.0,
            tag: "".into(),
            comment: "".into(),
        }
    }

    pub fn new_with_date(date: i32) -> Self {
        Self { date, ..Self::new() }
    }

    pub async fn save(&self) -> Option<bool> {
        let id = &self.id.as_string()?;
        let _ = write(id, "price", &self.price.to_string()).await;
        let _ = write(id, "date", &self.date.to_string()).await;
        let _ = write(id, "tag", &self.tag.as_string()?).await; 
        let _ = write(id, "comment", &self.comment.as_string()?).await; 
        Some(true)
    }

    pub async fn fetch(id: &JsValue) -> Option<Self> {
        let id = &id.as_string()?;
        Some(Day {
            price: read(id, "price").await.ok()?.as_string()?.parse().ok()?,
            date: read(id, "date").await.ok()?.as_string()?.parse().ok()?,
            tag: read(id, "tag").await.ok()?,
            comment: read(id, "comment").await.ok()?,
            id: id.into(),
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
    let mut p = read(&ns.as_string()?, "root").await;
    let mut v = vec![];
    while let Ok(id) = p {
        v.push( Day::fetch(&id).await? );
        p = read(&id.as_string()?, "next").await;
    }
    Some(v)
}

#[wasm_bindgen]
impl Store {

    async fn all(ns: &JsValue) -> Option<Vec<Day>> {
        Self::all_with(ns, |x| x.date >= 0).await
    }

    async fn all_with<F: FnMut(&Day) -> bool>(ns: &JsValue, f: F) -> Option<Vec<Day>> {
        let mut v = store(ns).await?;
        v.retain(f);
        Some(v)
    }

    pub async fn append(ns: &JsValue, day: &Day) -> Option<bool> {
        let ns = &ns.as_string()?;
        let id = &day.id.as_string()?; 
        if let Ok(root) = read(ns, "root").await {
            let _ = write(id, "next", &root.as_string()?).await;
        }
        let _ = write(ns, "root", id).await;
        Some(true)  
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


