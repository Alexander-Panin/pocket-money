use wasm_bindgen::prelude::*;
use uuid::Uuid;
use crate::opfs::{read, write};

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

    pub fn r#move(self) -> Self {
        Day{ id: Uuid::new_v4().to_string().into(), ..self }
    }

    pub async fn fetch(id: &JsValue) -> Self {
        Day {
            price: read(id.clone(), "price".into()).await
                .map(|x| x.unchecked_into_f64() as f32)
                .unwrap_or(0.0),
            date: read(id.clone(), "date".into()).await
                .map(|x| x.unchecked_into_f64() as i32)
                .unwrap_or(1),
            tag: read(id.clone(), "tag".into()).await
                .unwrap_or("".into()),
            comment: read(id.clone(), "comment".into()).await
                .unwrap_or("".into()),
            id: id.clone(),
        }
    }

    pub async fn save(&self) {
        let id = &self.id;
        let _ = write(id, &"price".into(), &self.price.to_string().into()).await;
        let _ = write(id, &"date".into(), &self.date.to_string().into()).await;
        let _ = write(id, &"tag".into(), &self.tag).await;
        let _ = write(id, &"comment".into(), &self.comment).await;
    }
}

