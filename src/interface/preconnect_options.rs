use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreconnectOptions {
    num_sockets: Option<usize>,
    url: JsString,
}

#[wasm_bindgen]
impl PreconnectOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(url: JsString, num_sockets: Option<usize>) -> PreconnectOptions {
        PreconnectOptions { url, num_sockets }
    }

    #[wasm_bindgen(getter, js_name = "numSockets")]
    pub fn num_sockets(&self) -> Option<usize> {
        self.num_sockets
    }

    #[wasm_bindgen(setter)]
    pub fn set_num_sockets(&mut self, value: Option<usize>) {
        self.num_sockets = value;
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
