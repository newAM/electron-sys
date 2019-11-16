use crate::event_emitter::EventEmitter;
use js_sys::{Object, JsString};
use wasm_bindgen::prelude::*;

pub type Options = Object;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type BrowserWindow;

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<&Options>) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &BrowserWindow, path: JsString);
}
