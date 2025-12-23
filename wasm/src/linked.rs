use alloc::vec;
use alloc::vec::Vec;
use alloc::collections::BTreeSet;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{JsString};
use core::future::Future;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

pub async fn collect_ids<T, F>(ns: JsString, read: T) -> Vec<JsString>
    where
        T: Fn(JsString, JsString) -> F, 
        F: Future<Output = Result<JsString, JsString>> 
{
    let Ok(mut id) = read(ns, "root".into()).await else { return vec![]; };
    let mut result = vec![id.clone()];
    let mut acc: BTreeSet<_> = BTreeSet::from([id.as_string()]);
    while let Ok(new_id) = read(id, "next".into()).await {
        if !acc.insert(new_id.as_string()) {
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

    async fn singleton(_id: JsString, _name: JsString) -> Result<JsString, JsString> {
        Ok("ok".into())
    }

    async fn err(_id: JsString, _name: JsString) -> Result<JsString, JsString> {
        Err("err".into())
    }

    async fn many(id: JsString, _name: JsString) -> Result<JsString, JsString> {
        if id.as_string().unwrap().len() > 7 {
            Err("err".into())
        } else {
            let x: JsString = "a".into(); 
            Ok(x.concat(&id))
        }
    }

    async fn many_with_loop(id: JsString, _name: JsString) -> Result<JsString, JsString> {
        if id.as_string().unwrap().len() > 7 {
            Ok("aaa".into())
        } else {
            let x: JsString = "a".into(); 
            Ok(x.concat(&id))
        }
    }

    #[wasm_bindgen_test]
    async fn many_with_loop_test() {
        let x = collect_ids("a".into(), many_with_loop).await;
        let result: Vec<JsString> = vec!["aa".into(), "aaa".into(), "aaaa".into(), "aaaaa".into(), "aaaaaa".into(), "aaaaaaa".into(), "aaaaaaaa".into()];
        assert_eq!(result, x);
    }    

    #[wasm_bindgen_test]
    async fn many_test() {
        let x = collect_ids("a".into(), many).await;
        let result: Vec<JsString> = vec!["aa".into(), "aaa".into(), "aaaa".into(), "aaaaa".into(), "aaaaaa".into(), "aaaaaaa".into(), "aaaaaaaa".into()];
        assert_eq!(result, x);
    }

    #[wasm_bindgen_test]
    async fn singleton_test() {
        let x = collect_ids("adf".into(), singleton).await;
        let result: Vec<JsString> = vec!["ok".into()]; 
        assert_eq!(result, x);
    }

    #[wasm_bindgen_test]
    async fn err_test() {
        let x = collect_ids("adf".into(), err).await;
        let result: Vec<JsString> = vec![]; 
        assert_eq!(result, x);
    }
}
