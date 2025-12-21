use wasm_bindgen::prelude::*;
use web_sys::{
    window, 
    FileSystemGetFileOptions, 
    FileSystemDirectoryHandle, 
    FileSystemFileHandle, 
    FileSystemSyncAccessHandle,
    FileSystemWritableFileStream, 
    File, 
    WorkerGlobalScope,
};
use web_sys::js_sys::{
    Promise, 
    JsString, 
    global, 
    Uint8Array
};
use wasm_bindgen_futures::{JsFuture};

async fn future<T: JsCast>(p: Promise) -> Result<T, JsString> {
    Ok(JsFuture::from(p).await?.dyn_into::<T>()?)
}

pub async fn read(id: JsString, name: JsString) -> Result<JsString, JsString> {
    let key = &(id.concat(&name)).as_string().ok_or::<&str>("".into())?;
    let handle = file_handle(key).await?;
    let file: File = future(handle.get_file()).await?;
    Ok(JsFuture::from(file.text()).await?.into())
}

pub async fn write(id: JsString, name: JsString, value: JsString) -> Result<(), JsString> {
    let key = &(id.concat(&name)).as_string().ok_or::<&str>("".into())?;
    let value = &value.as_string().ok_or::<&str>("".into())?;
    let p = file_handle_create_if_needed(key).await?.create_writable();
    let ws: FileSystemWritableFileStream = future(p).await?;
    let p = ws.write_with_str(value)?;
    let _ = JsFuture::from(p).await?;
    JsFuture::from(ws.close()).await?;
    Ok(())
}

async fn file_handle_create_if_needed(key: &str) -> Result<FileSystemFileHandle, JsString> {
    let p = window().ok_or::<&str>("".into())?.navigator().storage().get_directory();
    let root: FileSystemDirectoryHandle = future(p).await?;
    let options = FileSystemGetFileOptions::new();
    options.set_create(true);
    let p = root.get_file_handle_with_options(key, &options);
    future(p).await
}

async fn file_handle(key: &str) -> Result<FileSystemFileHandle, JsString> {
    let p = window().ok_or::<&str>("".into())?.navigator().storage().get_directory();
    let root: FileSystemDirectoryHandle = future(p).await?;
    let p = root.get_file_handle(key);
    future(p).await
}

#[wasm_bindgen]
pub async fn read_file_in_worker(id: JsString, name: JsString) -> Result<JsString, JsValue> {
    let key = &(id.concat(&name)).as_string().unwrap_or_default();
    let storage = global()
        .unchecked_into::<WorkerGlobalScope>()
        .navigator()
        .storage();
    let root: FileSystemDirectoryHandle = future(storage.get_directory()).await?;
    let handle: FileSystemFileHandle = future(root.get_file_handle(key)).await?;
    let file: File = future(handle.get_file()).await?;
    Ok(JsFuture::from(file.text()).await?.into())
}

#[wasm_bindgen]
pub async fn write_file_in_worker(id: JsString, name: JsString, value: JsString) -> Result<(), JsValue> {
    let key = &(id.concat(&name)).as_string().unwrap_or_default();
    let value = &value.as_string().unwrap_or_default();
    let storage = global()
        .unchecked_into::<WorkerGlobalScope>()
        .navigator()
        .storage();
    let root: FileSystemDirectoryHandle = future(storage.get_directory()).await?;
    let options = FileSystemGetFileOptions::new();
    options.set_create(true);
    let handle: FileSystemFileHandle = future(root.get_file_handle_with_options(&key, &options)).await?;
    let sync_handle: FileSystemSyncAccessHandle = future(handle.create_sync_access_handle()).await?;
    sync_handle.write_with_buffer_source(&Uint8Array::from(value.as_bytes()))?;
    sync_handle.flush()?; 
    sync_handle.close();
    Ok(())
}
