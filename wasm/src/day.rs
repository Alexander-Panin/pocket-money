use wasm_bindgen::prelude::*;
use web_sys::js_sys::{JsString};
use uuid::Uuid;
use crate::opfs::{read, write};
use crate::local_storage::{read as fastread, write as fastwrite};
use crate::provider::{Provider};

#[wasm_bindgen]
pub async fn save_price(id: &JsString, price: &JsString) -> Result<(), JsString> {
    Provider{read,write}.save_price(id.clone(), price.clone()).await?;
    Provider{read: fastread, write: fastwrite}.save_price(id.clone(), price.clone()).await
}

#[wasm_bindgen]
pub async fn save_date(id: &JsString, date: &JsString) -> Result<(), JsString> {
    Provider{read,write}.save_date(id.clone(), date.clone()).await?;
    Provider{read: fastread, write: fastwrite}.save_date(id.clone(), date.clone()).await
}

#[wasm_bindgen]
pub async fn save_tag(id: &JsString, tag: &JsString) -> Result<(), JsString> {
    Provider{read,write}.save_tag(id.clone(), tag.clone()).await?;
    Provider{read: fastread, write: fastwrite}.save_tag(id.clone(), tag.clone()).await
}

#[wasm_bindgen]
pub async fn save_comment(id: &JsString, comment: &JsString) -> Result<(), JsString> {
    Provider{read,write}.save_comment(id.clone(), comment.clone()).await?;
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
        Provider{read, write}.fetch(id.clone()).await
    }
}

