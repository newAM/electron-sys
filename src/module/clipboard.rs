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
    pub fn available_formats(this: &Clipboard, r#type: Option<&JsString>) -> Box<[JsValue]>;

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

    #[wasm_bindgen(method, js_name = "readFindText")]
    pub fn read_find_text(this: &Clipboard) -> JsString;

    #[wasm_bindgen(method, js_name = "readHTML")]
    pub fn read_html(this: &Clipboard, kind: &JsString) -> JsString;

    #[wasm_bindgen(method, js_name = "readImage")]
    pub fn read_image(this: &Clipboard, kind: &JsString) -> NativeImage;

    #[wasm_bindgen(method, js_name = "readRTF")]
    pub fn read_rtf(this: &Clipboard, kind: &JsString) -> JsString;

    #[wasm_bindgen(method, js_name = "readText")]
    pub fn read_text(this: &Clipboard, kind: &JsString) -> JsString;

    #[wasm_bindgen(method)]
    pub fn write(this: &Clipboard, data: Data, kind: &JsString);

    #[wasm_bindgen(method, js_name = "writeBookmark")]
    pub fn write_bookmark(this: &Clipboard, title: &JsString, url: &JsString, kind: Option<&JsString>);

    #[wasm_bindgen(method, js_name = "writeBuffer")]
    pub fn write_buffer(this: &Clipboard, format: &JsString, buffer: &Buffer, kind: Option<&JsString>);

    #[wasm_bindgen(method, js_name = "writeFindText")]
    pub fn write_find_text(this: &Clipboard, text: &JsString);

    #[wasm_bindgen(method, js_name = "writeHTML")]
    pub fn write_html(this: &Clipboard, markup: &JsString, kind: &JsString);

    #[wasm_bindgen(method, js_name = "writeImage")]
    pub fn write_image(this: &Clipboard, image: &NativeImage, kind: &JsString);

    #[wasm_bindgen(method, js_name = "writeRTF")]
    pub fn write_rtf(this: &Clipboard, text: &JsString, kind: &JsString);

    #[wasm_bindgen(method, js_name = "writeText")]
    pub fn write_text(this: &Clipboard, text: &JsString, kind: &JsString);
}
