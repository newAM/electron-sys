use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveClientCertificate {
    kind: JsString,
    origin: JsString,
}

#[wasm_bindgen]
impl RemoveClientCertificate {
    #[wasm_bindgen(constructor)]
    pub fn new(kind: JsString, origin: JsString) -> RemoveClientCertificate {
        RemoveClientCertificate { kind, origin }
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> JsString {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: JsString) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> JsString {
        self.origin.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_origin(&mut self, value: JsString) {
        self.origin = value;
    }
}
