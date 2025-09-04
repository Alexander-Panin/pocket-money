use web_sys::window;

pub fn store() -> Option<u32> {
    let local_storage = window()?.local_storage().ok()??;
    let x = local_storage.length();
    let _ = local_storage.set_item(&format!("{:?}", x), "foobar");
    x.ok()
}