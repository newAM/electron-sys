use js_sys::{Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TouchBarSliderOptions {
    change: Option<Function>,
    label: Option<JsString>,
    max_value: Option<f32>,
    min_value: Option<f32>,
    value: Option<f32>,
}

#[wasm_bindgen]
impl TouchBarSliderOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        change: Option<Function>,
        label: Option<JsString>,
        max_value: Option<f32>,
        min_value: Option<f32>,
        value: Option<f32>,
    ) -> TouchBarSliderOptions {
        TouchBarSliderOptions {
            change,
            label,
            max_value,
            min_value,
            value,
        }
    }

    pub fn new() -> TouchBarSliderOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn change(&self) -> Option<Function> {
        self.change.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_change(&mut self, value: Option<Function>) {
        self.change = value;
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<JsString> {
        self.label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_label(&mut self, value: Option<JsString>) {
        self.label = value;
    }

    #[wasm_bindgen(getter, js_name = "maxValue")]
    pub fn max_value(&self) -> Option<f32> {
        self.max_value
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_value(&mut self, value: Option<f32>) {
        self.max_value = value;
    }

    #[wasm_bindgen(getter, js_name = "minValue")]
    pub fn min_value(&self) -> Option<f32> {
        self.min_value
    }

    #[wasm_bindgen(setter)]
    pub fn set_min_value(&mut self, value: Option<f32>) {
        self.min_value = value;
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Option<f32> {
        self.value
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: Option<f32>) {
        self.value = value;
    }
}
