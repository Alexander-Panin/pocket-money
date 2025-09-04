use web_sys::window;

pub fn store() -> Option<u32> {
    let local_storage = window()?.local_storage().ok()??;
    let x = local_storage.length();
    let s = render()?;
    let _ = local_storage.set_item(&format!("{:?} {}", x, s), "foobar");
    x.ok()
}

pub fn render() -> Option<String> {
    let document = window()?.document()?;
    let x = document.query_selector(&"#one").ok()??;
    Some(x.id())
}