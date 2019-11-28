use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FeedUrlOptions {
    url: JsString,
    headers: Option<Object>,
    server_type: Option<JsString>,
}

#[wasm_bindgen]
impl FeedUrlOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(url: JsString, headers: Option<Object>, server_type: Option<JsString>) -> FeedUrlOptions {
        FeedUrlOptions {
            url,
            headers,
            server_type,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> JsString {
        self.url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_url(&mut self, value: JsString) {
        self.url = value;
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
    pub fn server_type(&self) -> Option<JsString> {
        self.server_type.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_server_type(&mut self, value: Option<JsString>) {
        self.server_type = value;
    }
}
