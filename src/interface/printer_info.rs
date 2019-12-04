use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type PrinterInfo;

    #[wasm_bindgen(method, getter)]
    pub fn description(this: &PrinterInfo) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "isDefault")]
    pub fn is_default(this: &PrinterInfo) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &PrinterInfo) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn status(this: &PrinterInfo) -> i32;
}
