use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GetFileIconOptions {
    size: JsString,
}

#[wasm_bindgen]
impl GetFileIconOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(size: JsString) -> GetFileIconOptions {
        GetFileIconOptions { size }
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> JsString {
        self.size.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, value: JsString) {
        self.size = value;
    }
}
