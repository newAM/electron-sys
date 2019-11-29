use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenDevToolsOptions {
    activate: Option<bool>,
    mode: JsString,
}

#[wasm_bindgen]
impl OpenDevToolsOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(activate: Option<bool>, mode: JsString) -> OpenDevToolsOptions {
        OpenDevToolsOptions { activate, mode }
    }

    #[wasm_bindgen(getter)]
    pub fn activate(&self) -> Option<bool> {
        self.activate
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> JsString {
        self.mode.clone()
    }
}
