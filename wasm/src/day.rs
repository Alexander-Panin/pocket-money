use crate::alloc::string::ToString;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{JsString};
use uuid::Uuid;
use crate::opfs::{read, read_in_worker, write_in_worker, noop_write};
use crate::local_storage::{read as fastread, write as fastwrite};
use crate::provider::{Provider};

#[wasm_bindgen]
pub async fn save_price(id: &JsString, price: &JsString) -> Result<(), JsString> {
    Provider{
        read: read_in_worker,
        write: write_in_worker,
    }.save_price(id.clone(), price.clone()).await
}

#[wasm_bindgen]
pub async fn save_price_fast(id: &JsString, price: &JsString) -> Result<(), JsString> {
    Provider{read: fastread, write: fastwrite}.save_price(id.clone(), price.clone()).await
}

#[wasm_bindgen]
pub async fn save_date(id: &JsString, date: &JsString) -> Result<(), JsString> {
    Provider{
        read: read_in_worker,
        write: write_in_worker,
    }.save_date(id.clone(), date.clone()).await
}

#[wasm_bindgen]
pub async fn save_date_fast(id: &JsString, date: &JsString) -> Result<(), JsString> {
    Provider{read: fastread, write: fastwrite}.save_date(id.clone(), date.clone()).await
}

#[wasm_bindgen]
pub async fn save_tag(id: &JsString, tag: &JsString) -> Result<(), JsString> {
    Provider{
        read: read_in_worker,
        write: write_in_worker,
    }.save_tag(id.clone(), tag.clone()).await
}

#[wasm_bindgen]
pub async fn save_tag_fast(id: &JsString, tag: &JsString) -> Result<(), JsString> {
    Provider{read: fastread, write: fastwrite}.save_tag(id.clone(), tag.clone()).await
}

#[wasm_bindgen]
pub async fn save_comment(id: &JsString, comment: &JsString) -> Result<(), JsString> {
    Provider{
        read: read_in_worker,
        write: write_in_worker,
    }.save_comment(id.clone(), comment.clone()).await
}

#[wasm_bindgen]
pub async fn save_comment_fast(id: &JsString, comment: &JsString) -> Result<(), JsString> {
    Provider{read: fastread, write: fastwrite}.save_comment(id.clone(), comment.clone()).await
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct Day {
    pub date: i32,
    pub price: f32,
    pub tag: JsString,
    pub comment: JsString,
    pub id: JsString,
}

impl Default for Day {
    fn default() -> Self {  
        Day {
            id: "".into(),
            date: 1,
            price: 0.0,
            comment: "".into(),  
            tag: "".into(),
        }
    }
}

#[wasm_bindgen]
impl Day {

    // ui - create new record
    pub fn new() -> Self { 
        Day { 
            id: Uuid::new_v4().to_string().into(),  
            ..Self::default() 
        }
    }

    // ui - placeholder for record    
    pub fn empty() -> Self { Self::default() }

    pub fn r#move(self) -> Self {
        Day{ id: Self::new().id, ..self }
    }

    // ui - fullfill record
    pub async fn fetch(id: &JsString) -> Self {
        Provider{read, write: noop_write}.fetch(id.clone()).await
    }
}

