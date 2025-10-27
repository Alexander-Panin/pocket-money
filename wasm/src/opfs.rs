use wasm_bindgen::prelude::*;
use web_sys::{window, FileSystemGetFileOptions, FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemWritableFileStream, File};
use wasm_bindgen_futures::{JsFuture};

#[wasm_bindgen]
pub async fn write(filename: &str) -> Result<(), JsValue> {
    let p = window().unwrap().navigator().storage().get_directory();
    let root = JsFuture::from(p).await?
        .dyn_into::<FileSystemDirectoryHandle>()
        .unwrap();

    let options = FileSystemGetFileOptions::new();
    options.set_create(true);
    let p = root.get_file_handle_with_options(filename, &options);
    let file = JsFuture::from(p).await?
        .dyn_into::<FileSystemFileHandle>()
        .unwrap();

    let p = file.create_writable();
    let ws = JsFuture::from(p).await?
        .dyn_into::<FileSystemWritableFileStream>()
        .unwrap();

    let p = ws.write_with_str("hello123").unwrap();
    let _ = JsFuture::from(p).await?;
    JsFuture::from(ws.close()).await?;
    Ok(())
}

#[wasm_bindgen]
pub async fn read(filename: &str) -> Result<JsValue, JsValue> {
    let p = window().unwrap().navigator().storage().get_directory();
    let root = JsFuture::from(p).await?
        .dyn_into::<FileSystemDirectoryHandle>()
        .unwrap();

    let p = root.get_file_handle(filename);
    let file = JsFuture::from(p).await?
        .dyn_into::<FileSystemFileHandle>()
        .unwrap();

    let p = file.get_file();
    let file = JsFuture::from(p).await?
        .dyn_into::<File>()
        .unwrap();

    let p = file.text();
    let text = JsFuture::from(p).await?;
    Ok(text)
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: JsValue);
}

#[wasm_bindgen(start)]
async fn run() {
    log("hello world".into());
    log("write".into());
    let _ = write("foobar").await;
    log("read".into());
    let t = read("foobar").await;
    log("value".into());
    log(t.unwrap());
}