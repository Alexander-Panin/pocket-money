use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use std::future::Future;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

pub async fn collect_ids<T, F>(ns: &JsValue, read: T) -> Vec<JsValue>
    where
        T: Fn(JsValue, JsValue) -> F, 
        F: Future<Output = Result<JsValue, JsValue>> 
{
    let Ok(mut id) = read(ns.clone(), "root".into()).await else { return vec![]; };
    let (mut result, mut acc) = (vec![id.clone()], HashSet::from([id.as_string().unwrap()]));
    while let Ok(new_id) = read(id, "next".into()).await {
        if !acc.insert(new_id.as_string().unwrap()) {
            #[cfg(not(test))]
            alert("Corrupted data - cycle detected");
            break;
        }
        result.push(new_id.clone());
        id = new_id; 
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    async fn singleton(_id: JsValue, _name: JsValue) -> Result<JsValue, JsValue> {
        Ok("ok".into())
    }

    async fn err(_id: JsValue, _name: JsValue) -> Result<JsValue, JsValue> {
        Err("err".into())
    }

    #[wasm_bindgen_test]
    async fn singleton_test() {
        let x = collect_ids(&"adf".into(), singleton).await;
        let result: Vec<JsValue> = vec!["ok".into()]; 
        assert_eq!(result, x);
    }

    #[wasm_bindgen_test]
    async fn err_test() {
        let x = collect_ids(&"adf".into(), err).await;
        let result: Vec<JsValue> = vec![]; 
        assert_eq!(result, x);
    }
}
