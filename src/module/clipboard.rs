use crate::{
    class::NativeImage,
    interface::{Data, ReadBookmark},
};
use js_sys::JsString;
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type Clipboard;

    pub static clipboard: Clipboard;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "availableFormats")]
    pub fn available_formats(this: &Clipboard, r#type: Option<&str>) -> Box<[JsValue]>;

    #[wasm_bindgen(method)]
    pub fn clear(this: &Clipboard, r#type: Option<&str>);

    #[wasm_bindgen(method)]
    pub fn has(this: &Clipboard, format: &str, r#type: Option<&str>) -> bool;

    #[wasm_bindgen(method)]
    pub fn read(this: &Clipboard, format: &str) -> JsString;

    #[wasm_bindgen(method, js_name = "readBookmark")]
    pub fn read_bookmark(this: &Clipboard) -> ReadBookmark;

    #[wasm_bindgen(method, js_name = "readBuffer")]
    pub fn read_buffer(this: &Clipboard) -> Buffer;

    #[wasm_bindgen(method, js_name = "readFindText")]
    pub fn read_find_text(this: &Clipboard) -> JsString;

    #[wasm_bindgen(method, js_name = "readHTML")]
    pub fn read_html(this: &Clipboard, kind: &str) -> JsString;

    #[wasm_bindgen(method, js_name = "readImage")]
    pub fn read_image(this: &Clipboard, kind: &str) -> NativeImage;

    #[wasm_bindgen(method, js_name = "readRTF")]
    pub fn read_rtf(this: &Clipboard, kind: &str) -> JsString;

    #[wasm_bindgen(method, js_name = "readText")]
    pub fn read_text(this: &Clipboard, kind: &str) -> JsString;

    #[wasm_bindgen(method)]
    pub fn write(this: &Clipboard, data: Data, kind: &str);

    #[wasm_bindgen(method, js_name = "writeBookmark")]
    pub fn write_bookmark(this: &Clipboard, title: &str, url: &str, kind: Option<&str>);

    #[wasm_bindgen(method, js_name = "writeBuffer")]
    pub fn write_buffer(this: &Clipboard, format: &str, buffer: &Buffer, kind: Option<&str>);

    #[wasm_bindgen(method, js_name = "writeFindText")]
    pub fn write_find_text(this: &Clipboard, text: &str);

    #[wasm_bindgen(method, js_name = "writeHTML")]
    pub fn write_html(this: &Clipboard, markup: &str, kind: &str);

    #[wasm_bindgen(method, js_name = "writeImage")]
    pub fn write_image(this: &Clipboard, image: &NativeImage, kind: &str);

    #[wasm_bindgen(method, js_name = "writeRTF")]
    pub fn write_rtf(this: &Clipboard, text: &str, kind: &str);

    #[wasm_bindgen(method, js_name = "writeText")]
    pub fn write_text(this: &Clipboard, text: &str, kind: &str);
}
