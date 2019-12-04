use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FilePathWithHeaders {
    headers: Option<Object>,
    path: JsString,
}

#[wasm_bindgen]
impl FilePathWithHeaders {
    #[wasm_bindgen(constructor)]
    pub fn new(headers: Option<Object>, path: JsString) -> FilePathWithHeaders {
        FilePathWithHeaders { headers, path }
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> Option<Object> {
        self.headers.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_headers(&mut self, value: Option<Object>) {
        self.headers = value;
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> JsString {
        self.path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_path(&mut self, value: JsString) {
        self.path = value;
    }
}
