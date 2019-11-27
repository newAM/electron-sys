use crate::object::WebContents;
use js_sys::{JsString, Object};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

pub type Options = Object;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type BrowserWindow;

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<&Options>) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &BrowserWindow, path: &JsString);

    #[wasm_bindgen(method, js_name = "setTitle")]
    pub fn set_title(this: &BrowserWindow, title: &JsString);

    #[wasm_bindgen(method, getter, js_name = "webContents")]
    pub fn web_contents(this: &BrowserWindow) -> WebContents;
}
