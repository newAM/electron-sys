use crate::class::NativeImage;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SegmentedControlSegment {
    enabled: Option<bool>,
    icon: Option<NativeImage>,
    label: Option<JsString>,
}

#[wasm_bindgen]
impl SegmentedControlSegment {
    #[wasm_bindgen]
    pub fn new(enabled: Option<bool>, icon: Option<NativeImage>, label: Option<JsString>) -> SegmentedControlSegment {
        SegmentedControlSegment { enabled, icon, label }
    }

    #[wasm_bindgen(getter)]
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }

    #[wasm_bindgen(setter)]
    pub fn set_enabled(&mut self, value: Option<bool>) {
        self.enabled = value;
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
