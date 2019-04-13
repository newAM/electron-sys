use crate::event_emitter::EventEmitter;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Options {
    pub height: u32,
    pub width: u32,
}

#[wasm_bindgen(module = "electron")]
extern "C" {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type BrowserWindow;

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<Options>) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &BrowserWindow, path: JsString);
}
