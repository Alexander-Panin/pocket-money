use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlTemplateElement, DocumentFragment};

pub fn store() -> Option<u32> {
    let local_storage = window()?.local_storage().ok()??;
    let x = local_storage.length();
    let _ = local_storage.set_item(&format!("{:?}", x), "foobar");
    x.ok()
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}


#[wasm_bindgen]
pub fn render_template(main: &str, t: &str) {
    let _ = render(main, t);
}

fn render(main:& str, t: &str) -> Option<()> {
    let document = window()?.document()?;
    let main = document.query_selector(main).ok()??;
    let template = document.query_selector(t).ok()??.dyn_into::<HtmlTemplateElement>().ok()?;

    for x in 1..5 {
        let root = template
            .content()
            .clone_node_with_deep(true).ok()?
            .dyn_into::<DocumentFragment>().ok()?;
        let nodes = root.query_selector_all("div").ok()?;
        nodes.item(0)?.set_text_content(Some(&format!("{x}")));
        nodes.item(1)?.set_text_content(Some(&format!("{x}+{x}+{x}")));
        let _ = main.append_child(&root);
    }
    Some(())
}























