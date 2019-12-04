use crate::class::NativeImage;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct ScrubberItem {
    icon: Option<NativeImage>,
    label: Option<JsString>,
}

#[wasm_bindgen]
impl ScrubberItem {
    #[wasm_bindgen(constructor)]
    pub fn new(icon: Option<NativeImage>, label: Option<JsString>) -> ScrubberItem {
        ScrubberItem { icon, label }
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> Option<NativeImage> {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: Option<NativeImage>) {
        self.icon = value;
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<JsString> {
        self.label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_label(&mut self, value: Option<JsString>) {
        self.label = value;
    }
}
