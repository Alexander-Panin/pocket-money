use serde::{Deserialize};
use wasm_bindgen::prelude::*;
use js_sys::{JSON};
use web_sys::{
    window, 
    HtmlTemplateElement, 
    DocumentFragment,
    Element, 
    console,
};

#[wasm_bindgen]
pub fn view(list_id: &str, row_id: &str, popup_id: &str) {
    let r = render(list_id, row_id, popup_id);
    let _ = r.map_err(|x| console::error_1(&x));
}

#[derive(Deserialize, Debug)]
struct Day {
    date: u32,
    price: f32,
    tag: String,
    comment: String,
}

fn storage(key: &str) -> Result<Vec<Day>, JsValue> {
    let value = window()
        .ok_or(JsValue::from_str("not found window"))?
        .local_storage()?
        .ok_or(JsValue::from_str("not found local storage"))?
        .get_item(key)?
        .ok_or(JsValue::from_str(&format!("not found storage[{key}]")))?;
    let x = JSON::parse(&value)?;
    Ok(serde_wasm_bindgen::from_value(x)?)
}

fn container(id: &str) -> Result<Element, JsValue> {
    window()
        .ok_or(JsValue::from_str("not found window"))?
        .document()
        .ok_or(JsValue::from_str("not found document"))?
        .query_selector(&format!("#{id}"))?
        .ok_or(JsValue::from_str(&format!("not found node({id})")))
}

fn template(id: &str) -> Result<DocumentFragment, JsValue> {
    Ok(window()
        .ok_or(JsValue::from_str("not found window"))?
        .document()
        .ok_or(JsValue::from_str("not found document"))?
        .query_selector(&format!("#template-{id}"))?
        .ok_or(JsValue::from_str(&format!("not found template({id})")))?
        .dyn_into::<HtmlTemplateElement>()?
        .content())
}

fn rows(id: &str) -> Result<(), JsValue> {
    let container = container(id)?;
    let template = template(id)?;

    for (day,i) in storage("data")?.into_iter().zip(0..) {
        let content = template
            .clone_node_with_deep(true)?
            .dyn_into::<DocumentFragment>()?;
        row(&content, day, i)?;
        container.append_child(&content)?;
    }
    Ok(())
}

fn row(content: &DocumentFragment, day: Day, x: u32) -> Result<(), JsValue> {
    let node = content
        .query_selector_all("div")?
        .item(0)
        .ok_or(JsValue::from_str("no div[0] in row-template"))?;
    node.set_text_content(Some(&day.tag));
    node.dyn_into::<Element>()?.set_attribute("__id", &format!("{x}"))?;
    Ok(())
}

fn render(list_id: &str, row_id: &str, popup_id: &str) -> Result<(), JsValue> {
    container(list_id)?.append_child(&template(list_id)?.into())?;
    container(popup_id)?.append_child(&template(popup_id)?.into())?;
    rows(row_id)?;
    Ok(())
}























