use crate::{class::web_contents::WebContents, interface::browser_window_options::BrowserWindowOptions};
use js_sys::JsString;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/browser-window
    pub type BrowserWindow;

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<BrowserWindowOptions>) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &BrowserWindow, path: &JsString);

    #[wasm_bindgen(method, js_name = "setTitle")]
    pub fn set_title(this: &BrowserWindow, title: &JsString);

    #[wasm_bindgen(method, getter, js_name = "webContents")]
    pub fn web_contents(this: &BrowserWindow) -> WebContents;
}
