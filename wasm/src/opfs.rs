use wasm_bindgen::prelude::*;
use web_sys::{
    window, 
    FileSystemGetFileOptions, 
    FileSystemDirectoryHandle, 
    FileSystemFileHandle, 
    FileSystemSyncAccessHandle,
    File, 
    WorkerGlobalScope,
    TextEncoder
};
use web_sys::js_sys::{
    Promise, 
    JsString, 
    global, 
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

pub async fn read_in_worker(id: JsString, name: JsString) -> Result<JsString, JsString> {
    let key = &(id.concat(&name)).as_string().ok_or::<&str>("".into())?;
    let handle = file_handle_in_worker(key).await?;
    let file: File = future(handle.get_file()).await?;
    Ok(JsFuture::from(file.text()).await?.into())
}

pub async fn write_in_worker(id: JsString, name: JsString, value: JsString) -> Result<(), JsString> {
    let key = &(id.concat(&name)).as_string().ok_or::<&str>("".into())?;
    let value = &value.as_string().ok_or::<&str>("".into())?;
    let handle = file_handle_create_if_needed_in_worker(key).await?;
    let sync_handle: FileSystemSyncAccessHandle = future(handle.create_sync_access_handle()).await?;
    let size = sync_handle.write_with_u8_array(&TextEncoder::new()?.encode_with_input(value))?;
    sync_handle.truncate_with_f64(size)?;
    sync_handle.flush()?; 
    sync_handle.close();
    Ok(())
}

pub async fn noop_write(_id: JsString, _name: JsString, _value: JsString) -> Result<(), JsString> { Ok(()) }

async fn file_handle(key: &str) -> Result<FileSystemFileHandle, JsString> {
    let root = window()
        .ok_or::<&str>("".into())?
        .navigator()
        .storage()
        .get_directory();
    let root: FileSystemDirectoryHandle = future(root).await?;
    future(root.get_file_handle(key)).await
}

async fn file_handle_in_worker(key: &str) -> Result<FileSystemFileHandle, JsString> {
    let root = global()
        .unchecked_into::<WorkerGlobalScope>()
        .navigator()
        .storage()
        .get_directory();
    let root: FileSystemDirectoryHandle = future(root).await?;
    future(root.get_file_handle(key)).await
}

async fn file_handle_create_if_needed_in_worker(key: &str) -> Result<FileSystemFileHandle, JsString> {
    let root = global()
        .unchecked_into::<WorkerGlobalScope>()
        .navigator()
        .storage()
        .get_directory();
    let root: FileSystemDirectoryHandle = future(root).await?;
    let options = FileSystemGetFileOptions::new();
    options.set_create(true);
    future(root.get_file_handle_with_options(&key, &options)).await
}
