use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadBookmark {
    title: JsString,
    url: JsString,
}

#[wasm_bindgen]
impl ReadBookmark {
    #[wasm_bindgen(constructor)]
    pub fn new(title: JsString, url: JsString) -> ReadBookmark {
        ReadBookmark { title, url }
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> JsString {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: JsString) {
        self.title = value;
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> JsString {
        self.url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_url(&mut self, value: JsString) {
        self.url = value;
    }
}
