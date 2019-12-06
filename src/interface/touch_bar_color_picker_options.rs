use js_sys::{Array, Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TouchBarColorPickerOptions {
    available_colors: Option<Array>,
    change: Option<Function>,
    selected_color: Option<JsString>,
}

#[wasm_bindgen]
impl TouchBarColorPickerOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        available_colors: Option<Array>,
        change: Option<Function>,
        selected_color: Option<JsString>,
    ) -> TouchBarColorPickerOptions {
        TouchBarColorPickerOptions {
            available_colors,
            change,
            selected_color,
        }
    }

    pub fn new() -> TouchBarColorPickerOptions {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "availableColors")]
    pub fn available_colors(&self) -> Option<Array> {
        self.available_colors.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_available_colors(&mut self, value: Option<Array>) {
        self.available_colors = value;
    }

    #[wasm_bindgen(getter)]
    pub fn change(&self) -> Option<Function> {
        self.change.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_change(&mut self, value: Option<Function>) {
        self.change = value;
    }

    #[wasm_bindgen(getter, js_name = "selectedColor")]
    pub fn selected_color(&self) -> Option<JsString> {
        self.selected_color.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_selected_color(&mut self, value: Option<JsString>) {
        self.selected_color = value;
    }
}
