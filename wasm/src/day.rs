use wasm_bindgen::prelude::*;
use uuid::Uuid;
use crate::opfs::{read, write};
use crate::local_storage::{read as fastread, write as fastwrite};
use crate::provider::{Provider};

#[wasm_bindgen]
pub async fn save_price(id: &JsValue, price: &JsValue) -> Result<(), JsValue> {
    Provider{read,write}.save_price(id.clone(), price.clone()).await?;
    Provider{read: fastread, write: fastwrite}.save_price(id.clone(), price.clone()).await
}

#[wasm_bindgen]
pub async fn save_date(id: &JsValue, date: &JsValue) -> Result<(), JsValue> {
    Provider{read,write}.save_date(id.clone(), date.clone()).await?;
    Provider{read: fastread, write: fastwrite}.save_date(id.clone(), date.clone()).await
}

#[wasm_bindgen]
pub async fn save_tag(id: &JsValue, tag: &JsValue) -> Result<(), JsValue> {
    Provider{read,write}.save_tag(id.clone(), tag.clone()).await?;
    Provider{read: fastread, write: fastwrite}.save_tag(id.clone(), tag.clone()).await
}

#[wasm_bindgen]
pub async fn save_comment(id: &JsValue, comment: &JsValue) -> Result<(), JsValue> {
    Provider{read,write}.save_comment(id.clone(), comment.clone()).await?;
    Provider{read: fastread, write: fastwrite}.save_comment(id.clone(), comment.clone()).await
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

    // ui - create new record
    pub fn new() -> Self {
        Self { 
            id: Uuid::new_v4().to_string().into(), 
            date: 1,
            ..Self::default()
        }
    }

    // ui - placeholder for record    
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn r#move(self) -> Self {
        Day{ id: Uuid::new_v4().to_string().into(), ..self }
    }

    // ui - fullfill record
    pub async fn fetch(id: &JsValue) -> Self {
        Provider{read, write}.fetch(id.clone()).await
    }
}

