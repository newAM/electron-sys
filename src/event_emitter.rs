use js_sys::{Function, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    pub type EventEmitter;

    #[wasm_bindgen(method)]
    pub fn on(this: &EventEmitter, event: JsString, cb: &Function);
}
