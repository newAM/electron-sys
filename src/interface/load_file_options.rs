use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadFileOptions {
    hash: Option<JsString>,
    query: Option<Object>,
    search: Option<JsString>,
}

#[wasm_bindgen]
impl LoadFileOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(hash: Option<JsString>, query: Option<Object>, search: Option<JsString>) -> LoadFileOptions {
        LoadFileOptions { hash, query, search }
    }

    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> Option<JsString> {
        self.hash.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_hash(&mut self, value: Option<JsString>) {
        self.hash = value;
    }

    #[wasm_bindgen(getter)]
    pub fn query(&self) -> Option<Object> {
        self.query.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_query(&mut self, value: Option<Object>) {
        self.query = value;
    }

    #[wasm_bindgen(getter)]
    pub fn search(&self) -> Option<JsString> {
        self.search.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_search(&mut self, value: Option<JsString>) {
        self.search = value;
    }
}
