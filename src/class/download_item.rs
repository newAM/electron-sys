use crate::interface::SaveDialogOptions;
use js_sys::{Array, JsString};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/download-item
    pub type DownloadItem;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn cancel(this: &DownloadItem);

    #[wasm_bindgen(method, js_name = "canResume")]
    pub fn can_resume(this: &DownloadItem) -> bool;

    #[wasm_bindgen(method, js_name = "getContentDisposition")]
    pub fn get_content_disposition(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, js_name = "getETag")]
    pub fn get_etag(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, js_name = "getFilename")]
    pub fn get_filename(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, js_name = "getLastModifiedTime")]
    pub fn get_last_modified_time(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, js_name = "getMimeType")]
    pub fn get_mime_type(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, js_name = "getReceivedBytes")]
    pub fn get_received_bytes(this: &DownloadItem) -> usize;

    #[wasm_bindgen(method, js_name = "getSaveDialogOptions")]
    pub fn get_save_dialog_options(this: &DownloadItem) -> SaveDialogOptions;

    #[wasm_bindgen(method, js_name = "getSavePath")]
    pub fn get_save_path(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, js_name = "getSavePath")]
    pub fn get_start_time(this: &DownloadItem) -> u32;

    #[wasm_bindgen(method, js_name = "getState")]
    pub fn get_state(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, js_name = "getTotalBytes")]
    pub fn get_total_bytes(this: &DownloadItem) -> usize;

    #[wasm_bindgen(method, js_name = "getURL")]
    pub fn get_url(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, js_name = "getURLChain")]
    pub fn get_url_chain(this: &DownloadItem) -> Array;

    #[wasm_bindgen(method, js_name = "has_user_gesture")]
    pub fn has_user_gesture(this: &DownloadItem) -> bool;

    #[wasm_bindgen(method, js_name = "is_paused")]
    pub fn is_paused(this: &DownloadItem) -> bool;

    #[wasm_bindgen(method)]
    pub fn pause(this: &DownloadItem);

    #[wasm_bindgen(method)]
    pub fn resume(this: &DownloadItem);

    #[wasm_bindgen(method, js_name = "setSaveDialogOptions")]
    pub fn set_save_dialog_options(this: &DownloadItem, options: SaveDialogOptions);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "savePath")]
    pub fn save_path(this: &DownloadItem) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "savePath")]
    pub fn set_save_path(this: &DownloadItem, value: JsString);
}
