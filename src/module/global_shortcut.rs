use js_sys::{Array, Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type GlobalShortcut;

    #[wasm_bindgen(js_name = "globalShortcut")]
    pub static global_shortcut: GlobalShortcut;

    #[wasm_bindgen(method)]
    pub fn register(this: &GlobalShortcut, accelerator: &JsString, callback: &Function);

    #[wasm_bindgen(method, js_name = "registerAll")]
    pub fn register_all(this: &GlobalShortcut, accelerators: &Array, callback: &Function);

    #[wasm_bindgen(method, js_name = "isRegistered")]
    pub fn is_registered(this: &GlobalShortcut, accelerator: &JsString) -> bool;

    #[wasm_bindgen(method)]
    pub fn unregister(this: &GlobalShortcut, accelerator: &JsString);

    #[wasm_bindgen(method, js_name = "unregisterAll")]
    pub fn unregister_all(this: &GlobalShortcut);
}
