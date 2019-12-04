use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type ExtensionInfo;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &ExtensionInfo) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &ExtensionInfo, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn version(this: &ExtensionInfo) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_version(this: &ExtensionInfo, value: JsString);
}
