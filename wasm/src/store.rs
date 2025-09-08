use wasm_bindgen::prelude::*;
use web_sys::{
    window, 
    HtmlTemplateElement, 
    DocumentFragment,
    Document, 
    Element, 
    console,
};

pub fn store() -> Option<u32> {
    let local_storage = window()?.local_storage().ok()??;
    let x = local_storage.length();
    let _ = local_storage.set_item(&format!("{:?}", x), "foobar");
    x.ok()
}

#[wasm_bindgen]
pub fn view(list_id: &str, row_id: &str, popup_id: &str) {
    let r = render(list_id, row_id, popup_id);
    let _ = r.map_err(|x| console::error_1(&x));
}

fn render_list(document: &Document, list_id: &str) -> Result<(), JsValue> {
    let list = document
        .query_selector(&format!("#{list_id}"))?
        .ok_or(JsValue::from_str("can not find list"))?;
    let list_template = document
        .query_selector(&format!("#template-{list_id}"))?
        .ok_or(JsValue::from_str("can not find template-list"))?
        .dyn_into::<HtmlTemplateElement>()?;
    let content = list_template
        .content()
        .clone_node_with_deep(true)?
        .dyn_into::<DocumentFragment>()?;
    let _ = list.append_child(&content);
    Ok(())
}

fn render_rows(document: &Document, row_id: &str) -> Result<(), JsValue> {
    let row = document
        .query_selector(&format!("#{row_id}"))?
        .ok_or(JsValue::from_str("can not find row"))?;
    let row_template = document
        .query_selector(&format!("#template-{row_id}"))?
        .ok_or(JsValue::from_str("can not find template-row"))?
        .dyn_into::<HtmlTemplateElement>()?;

    for x in 1..5 {
        let content = row_template
            .content()
            .clone_node_with_deep(true)?
            .dyn_into::<DocumentFragment>()?;
        let nodes = content.query_selector_all("div")?;
        let node = nodes
            .item(0)
            .ok_or(JsValue::from_str("no nodes[0] in row-template"))?;
        node.set_text_content(Some("hello world!"));
        let _ = node
            .dyn_into::<Element>()?
            .set_attribute("__id", &format!("{x}"));
        let _ = row.append_child(&content);
    }
    Ok(())
}

fn render_popup(document: &Document, popup_id: &str) -> Result<(), JsValue> {
    let popup = document
        .query_selector(&format!("#{popup_id}"))?
        .ok_or(JsValue::from_str("can not find popup"))?;
    let popup_template = document
        .query_selector(&format!("#template-{popup_id}"))?
        .ok_or(JsValue::from_str("can not find template-popup"))?
        .dyn_into::<HtmlTemplateElement>()?;
    let content = popup_template
        .content()
        .clone_node_with_deep(true)?
        .dyn_into::<DocumentFragment>()?;
    let _ = popup.append_child(&content);
    Ok(())
}

fn render(list_id: &str, row_id: &str, popup_id: &str) -> Result<(), JsValue> {
    let document = window()
        .ok_or(JsValue::from_str("can not find window"))?
        .document()
        .ok_or(JsValue::from_str("can not find document"))?;
    let _ = render_list(&document, list_id);
    let _ = render_rows(&document, row_id);
    let _ = render_popup(&document, popup_id);
    Ok(())
}























