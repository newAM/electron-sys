use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResizeOptions {
    height: Option<u32>,
    quality: Option<JsString>,
    width: Option<u32>,
}

#[wasm_bindgen]
impl ResizeOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(height: Option<u32>, quality: Option<JsString>, width: Option<u32>) -> ResizeOptions {
        ResizeOptions { height, quality, width }
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> Option<u32> {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, value: Option<u32>) {
        self.height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn quality(&self) -> Option<JsString> {
        self.quality.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_quality(&mut self, value: Option<JsString>) {
        self.quality = value;
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> Option<u32> {
        self.width
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, value: Option<u32>) {
        self.width = value;
    }
}
