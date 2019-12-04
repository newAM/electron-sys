use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JumpListCategory {
    items: Option<Array>,
    kind: Option<JsString>,
    name: Option<JsString>,
}

#[wasm_bindgen]
impl JumpListCategory {
    #[wasm_bindgen(constructor)]
    pub fn new(items: Option<Array>, kind: Option<JsString>, name: Option<JsString>) -> JumpListCategory {
        JumpListCategory { items, kind, name }
    }

    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Option<Array> {
        self.items.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_items(&mut self, value: Option<Array>) {
        self.items = value;
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> Option<JsString> {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: Option<JsString>) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<JsString> {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, value: Option<JsString>) {
        self.name = value;
    }
}
