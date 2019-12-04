use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebRequestFilter {
    urls: Array,
}

#[wasm_bindgen]
impl WebRequestFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(urls: Array) -> WebRequestFilter {
        WebRequestFilter { urls }
    }

    #[wasm_bindgen(getter)]
    pub fn urls(&self) -> Array {
        self.urls.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_urls(&mut self, value: Array) {
        self.urls = value;
    }
}
