use wasm_bindgen::prelude::*;
use web_sys::js_sys::{JsString};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "localStorage"])]
    pub fn getItem(a: JsString) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "localStorage"])]
    pub fn setItem(a: JsString, b: JsString);
}

pub async fn read(id: JsString, name: JsString) -> Result<JsString, JsString> {
    let x = getItem(id.concat(&name));
    if x == JsValue::NULL { Err("".into()) } else { Ok(x.into()) } 
}

pub async fn write(id: JsString, name: JsString, value: JsString) -> Result<(), JsString> {
    setItem(id.concat(&name), value);
    Ok(())
}

