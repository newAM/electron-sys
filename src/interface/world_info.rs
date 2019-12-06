use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct WorldInfo {
    csp: Option<JsString>,
    name: Option<JsString>,
    security_origin: Option<JsString>,
}

#[wasm_bindgen]
impl WorldInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(csp: Option<JsString>, name: Option<JsString>, security_origin: Option<JsString>) -> WorldInfo {
        WorldInfo {
            csp,
            name,
            security_origin,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn csp(&self) -> Option<JsString> {
        self.csp.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_csp(&mut self, value: Option<JsString>) {
        self.csp = value;
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<JsString> {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, value: Option<JsString>) {
        self.name = value;
    }

    #[wasm_bindgen(getter, js_name = "securityOrigin")]
    pub fn security_origin(&self) -> Option<JsString> {
        self.security_origin.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_security_origin(&mut self, value: Option<JsString>) {
        self.security_origin = value;
    }
}
