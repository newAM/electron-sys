use crate::interface::Privileges;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomScheme {
    privileges: Option<Privileges>,
    scheme: JsString,
}

#[wasm_bindgen]
impl CustomScheme {
    #[wasm_bindgen(constructor)]
    pub fn new(privileges: Option<Privileges>, scheme: JsString) -> CustomScheme {
        CustomScheme { privileges, scheme }
    }

    #[wasm_bindgen(getter)]
    pub fn privileges(&self) -> Option<Privileges> {
        self.privileges
    }

    #[wasm_bindgen(setter)]
    pub fn set_privileges(&mut self, value: Option<Privileges>) {
        self.privileges = value;
    }

    #[wasm_bindgen(getter)]
    pub fn scheme(&self) -> JsString {
        self.scheme.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_scheme(&mut self, value: JsString) {
        self.scheme = value;
    }
}
