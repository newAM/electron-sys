use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Margins {
    bottom: Option<u32>,
    left: Option<u32>,
    margin_type: Option<JsString>,
    right: Option<u32>,
    top: Option<u32>,
}

#[wasm_bindgen]
impl Margins {
    #[wasm_bindgen(constructor)]
    pub fn new(
        bottom: Option<u32>,
        left: Option<u32>,
        margin_type: Option<JsString>,
        right: Option<u32>,
        top: Option<u32>,
    ) -> Margins {
        Margins {
            bottom,
            left,
            margin_type,
            right,
            top,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn bottom(&self) -> Option<u32> {
        self.bottom
    }

    #[wasm_bindgen(setter)]
    pub fn set_bottom(&mut self, value: Option<u32>) {
        self.bottom = value;
    }

    #[wasm_bindgen(getter)]
    pub fn left(&self) -> Option<u32> {
        self.left
    }

    #[wasm_bindgen(setter)]
    pub fn set_left(&mut self, value: Option<u32>) {
        self.left = value;
    }

    #[wasm_bindgen(getter)]
    pub fn margin_type(&self) -> Option<JsString> {
        self.margin_type.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_margin_type(&mut self, value: Option<JsString>) {
        self.margin_type = value;
    }

    #[wasm_bindgen(getter)]
    pub fn right(&self) -> Option<u32> {
        self.right
    }

    #[wasm_bindgen(setter)]
    pub fn set_right(&mut self, value: Option<u32>) {
        self.right = value;
    }

    #[wasm_bindgen(getter)]
    pub fn top(&self) -> Option<u32> {
        self.top
    }

    #[wasm_bindgen(setter)]
    pub fn set_top(&mut self, value: Option<u32>) {
        self.top = value;
    }
}
