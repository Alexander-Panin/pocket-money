use wasm_bindgen::prelude::*;
use crate::linked;
use crate::day::{Day};

pub struct Provider<T,F,T2,F2> 
    where
        T: Fn(JsValue, JsValue) -> F + Clone, 
        F: Future<Output = Result<JsValue, JsValue>>,
        T2: Fn(JsValue, JsValue, JsValue) -> F2 + Clone,
        F2: Future<Output = Result<(), JsValue>>,
{ 
    pub read: T, 
    pub write: T2 
}

impl<T,F,T2,F2> Provider<T,F,T2,F2>
    where
        T: Fn(JsValue, JsValue) -> F + Clone, 
        F: Future<Output = Result<JsValue, JsValue>>,
        T2: Fn(JsValue, JsValue, JsValue) -> F2 + Clone,
        F2: Future<Output = Result<(), JsValue>>,
{

    pub async fn all(&self, ns: JsValue) -> Vec<Day> {
        let mut result = vec![];
        for id in linked::collect_ids(ns, self.read.clone()).await { 
            result.push( self.fetch(id.clone()).await ); 
        }
        result
    }

    pub async fn append(&self, ns: JsValue, id: JsValue) -> Result<(), JsValue> {
        if let Ok(root) = (self.read)(ns.clone(), "root".into()).await {
            (self.write)(id.clone(), "next".into(), root).await?;
        }
        (self.write)(ns, "root".into(), id).await
    }

    pub async fn copy(&self, ns: JsValue, day: Day) -> Day {
        let x = day.r#move(); 
        let _ = self.append(ns, x.id.clone()).await; 
        self.save(&x).await;
        return x;
    }

    pub async fn fetch(&self, id: JsValue) -> Day {
        Day {
            price: (self.read)(id.clone(), "price".into()).await
                .map(|x| x.unchecked_into_f64() as f32)
                .unwrap_or(0.0),
            date: (self.read)(id.clone(), "date".into()).await
                .map(|x| x.unchecked_into_f64() as i32)
                .unwrap_or(1),
            tag: (self.read)(id.clone(), "tag".into()).await
                .unwrap_or("".into()),
            comment: (self.read)(id.clone(), "comment".into()).await
                .unwrap_or("".into()),
            id,
        }
    }

    async fn save(&self, day: &Day) {
        let id = &day.id;
        let _ = (self.write)(id.clone(), "price".into(), day.price.to_string().into()).await;
        let _ = (self.write)(id.clone(), "date".into(), day.date.to_string().into()).await;
        let _ = (self.write)(id.clone(), "tag".into(), day.tag.clone()).await;
        let _ = (self.write)(id.clone(), "comment".into(), day.comment.clone()).await;
    }

    pub async fn save_date(&self, id: JsValue, date: JsValue) -> Result<(), JsValue> {
        (self.write)(id, "date".into(), date).await
    }

    pub async fn save_price(&self, id: JsValue, price: JsValue) -> Result<(), JsValue> {
        (self.write)(id, "price".into(), price).await
    }

    pub async fn save_tag(&self, id: JsValue, tag: JsValue) -> Result<(), JsValue> {
        (self.write)(id, "tag".into(), tag).await
    }    

    pub async fn save_comment(&self, id: JsValue, comment: JsValue) -> Result<(), JsValue> {
        (self.write)(id, "comment".into(), comment).await
    }
}