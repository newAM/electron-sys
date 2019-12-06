use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TouchBarSpacerOptions {
    size: Option<JsString>,
}

#[wasm_bindgen]
impl TouchBarSpacerOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(size: Option<JsString>) -> TouchBarSpacerOptions {
        TouchBarSpacerOptions { size }
    }

    pub fn new() -> TouchBarSpacerOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> Option<JsString> {
        self.size.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, value: Option<JsString>) {
        self.size = value;
    }
}
