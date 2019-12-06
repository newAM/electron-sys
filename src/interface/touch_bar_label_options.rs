use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TouchBarLabelOptions {
    accessibility: Option<JsString>,
    label: Option<JsString>,
    text_color: Option<JsString>,
}

#[wasm_bindgen]
impl TouchBarLabelOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        accessibility: Option<JsString>,
        label: Option<JsString>,
        text_color: Option<JsString>,
    ) -> TouchBarLabelOptions {
        TouchBarLabelOptions {
            accessibility,
            label,
            text_color,
        }
    }

    pub fn new() -> TouchBarLabelOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn accessibility(&self) -> Option<JsString> {
        self.accessibility.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_accessibility(&mut self, value: Option<JsString>) {
        self.accessibility = value;
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<JsString> {
        self.label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_label(&mut self, value: Option<JsString>) {
        self.label = value;
    }

    #[wasm_bindgen(getter, js_name = "textColor")]
    pub fn text_color(&self) -> Option<JsString> {
        self.text_color.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_text_color(&mut self, value: Option<JsString>) {
        self.text_color = value;
    }
}
