use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type PowerSaveBlocker;

    #[wasm_bindgen(js_name = "powerSaveBlocker")]
    pub static power_save_blocker: PowerSaveBlocker;

    #[wasm_bindgen(method, js_name = "isStarted")]
    pub fn is_started(this: &PowerSaveBlocker, id: u32) -> bool;

    #[wasm_bindgen(method)]
    pub fn start(this: &PowerSaveBlocker, kind: &JsString) -> u32;

    #[wasm_bindgen(method)]
    pub fn stop(this: &PowerSaveBlocker, id: u32);
}
