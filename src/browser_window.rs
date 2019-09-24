use crate::event_emitter::EventEmitter;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// The [BrowserWindow] constructor options.
#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Options {
    /// The [BrowserWindow] height.
    pub height: u32,
    /// The [BrowserWindow] width.
    pub width: u32,
}

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type BrowserWindow;

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<Options>) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &BrowserWindow, path: JsString);
}
