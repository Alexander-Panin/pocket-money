use wasm_bindgen::prelude::*;
use web_sys::{
    window, 
    FileSystemGetFileOptions, 
    FileSystemDirectoryHandle, 
    FileSystemFileHandle, 
    FileSystemWritableFileStream, 
    File, 
};
use web_sys::js_sys::{Promise};
use wasm_bindgen_futures::{JsFuture};

async fn future<T: JsCast>(p: Promise) -> Result<T, JsValue> {
    JsFuture::from(p).await?.dyn_into::<T>()
}

async fn file_handle(key: &str, should_create: bool) -> Result<FileSystemFileHandle, JsValue> {
    let p = window().unwrap().navigator().storage().get_directory();
    let root: FileSystemDirectoryHandle = future(p).await?;
    let p = if should_create {
        let options = FileSystemGetFileOptions::new();
        options.set_create(true);
        root.get_file_handle_with_options(key, &options)
    } else {
        root.get_file_handle(key)
    };
    future(p).await
}

pub async fn read(id: &JsValue, name: &JsValue) -> Result<JsValue, JsValue> {
    let key = &(id + name).as_string().ok_or(JsValue::NULL)?;
    let handle = file_handle(key, false).await?;
    let file: File = future(handle.get_file()).await?;
    Ok(JsFuture::from(file.text()).await?)
}

pub async fn write(id: &JsValue, name: &JsValue, value: &JsValue) -> Result<(), JsValue> {
    let key = &(id + name).as_string().ok_or(JsValue::NULL)?;
    let value = &value.as_string().ok_or(JsValue::NULL)?;
    let p = file_handle(key, true).await?.create_writable();
    let ws: FileSystemWritableFileStream = future(p).await?;
    let p = ws.write_with_str(value)?;
    let _ = JsFuture::from(p).await?;
    JsFuture::from(ws.close()).await?;
    Ok(())
}


