use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "localStorage"])]
    pub fn getItem(a: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "localStorage"])]
    pub fn setItem(a: JsValue, b: JsValue);

    #[wasm_bindgen(js_namespace = ["window", "localStorage"])]
    pub fn clear();
}

pub async fn read(id: JsValue, name: JsValue) -> Result<JsValue, JsValue> {
    let x = getItem(id+name);
    if x == JsValue::NULL { Err(JsValue::NULL) } else { Ok(x) } 
}

pub async fn write(id: JsValue, name: JsValue, value: JsValue) -> Result<(), JsValue> {
    setItem(id+name, value);
    Ok(())
}

