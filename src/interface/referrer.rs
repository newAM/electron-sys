use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Referrer {
    policy: JsString,
    url: JsString,
}

#[wasm_bindgen]
impl Referrer {
    #[wasm_bindgen(constructor)]
    pub fn new(policy: JsString, url: JsString) -> Referrer {
        Referrer { policy, url }
    }

    #[wasm_bindgen(getter)]
    pub fn policy(&self) -> JsString {
        self.policy.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_policy(&mut self, value: JsString) {
        self.policy = value;
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
