use crate::interface::ReadBookmark;
use js_sys::{Array, JsString};
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type Clipboard;

    pub static clipboard: Clipboard;

    #[wasm_bindgen(method, js_name = "availableFormats")]
    pub fn available_formats(this: &Clipboard, r#type: Option<&JsString>) -> Array;

    #[wasm_bindgen(method)]
    pub fn clear(this: &Clipboard, r#type: Option<&JsString>);

    #[wasm_bindgen(method)]
    pub fn has(this: &Clipboard, format: &JsString, r#type: Option<&JsString>) -> bool;

    #[wasm_bindgen(method)]
    pub fn read(this: &Clipboard, format: &JsString) -> JsString;

    #[wasm_bindgen(method, js_name = "readBookmark")]
    pub fn read_bookmark(this: &Clipboard) -> ReadBookmark;

    #[wasm_bindgen(method, js_name = "readBuffer")]
    pub fn read_buffer(this: &Clipboard) -> Buffer;
}
