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
            result.push( Day::fetch(&id).await ); 
        }
        result
    }

    pub async fn append(&self, ns: JsValue, id: JsValue) {
        if let Ok(root) = (self.read)(ns.clone(), "root".into()).await {
            let _ = (self.write)(id.clone(), "next".into(), root).await;
        }
        let _ = (self.write)(ns, "root".into(), id).await;
    }

    pub async fn copy(&self, ns: JsValue, day: Day) -> Day {
        let x = day.r#move(); 
        self.append(ns, x.id.clone()).await; 
        x.save().await;
        return x;
    }
}