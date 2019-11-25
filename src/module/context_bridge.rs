use js_sys::{JsString, Object};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type ContextBridge;

    #[wasm_bindgen(js_name = "contextBridge")]
    pub static context_bridge: ContextBridge;

    #[wasm_bindgen(method, js_name = "exposeInMainWorld")]
    pub fn expose_in_main_world(this: &ContextBridge, api_key: &JsString, api: &Object);
}
