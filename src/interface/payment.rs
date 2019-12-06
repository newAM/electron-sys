use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type Payment;

    #[wasm_bindgen(method, getter, js_name = "productIdentifier")]
    pub fn product_identifier(this: &Payment) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "productIdentifier")]
    pub fn set_product_identifier(this: &Payment, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn quantity(this: &Payment) -> u32;

    #[wasm_bindgen(method, setter)]
    pub fn set_quantity(this: &Payment, value: u32);
}
