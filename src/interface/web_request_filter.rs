use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct WebRequestFilter {
    urls: Box<[JsValue]>,
}

#[wasm_bindgen]
impl WebRequestFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(urls: Box<[JsValue]>) -> WebRequestFilter {
        WebRequestFilter { urls }
    }

    #[wasm_bindgen(getter)]
    pub fn urls(&self) -> Box<[JsValue]> {
        self.urls.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_urls(&mut self, value: Box<[JsValue]>) {
        self.urls = value;
    }
}
