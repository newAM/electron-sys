use crate::class::NativeImage;
use js_sys::{Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TouchBarButtonOptions {
    accessibility_label: Option<JsString>,
    background_color: Option<JsString>,
    click: Option<Function>,
    icon_position: Option<JsString>,
    icon: Option<NativeImage>,
    label: Option<JsString>,
}

#[wasm_bindgen]
impl TouchBarButtonOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        accessibility_label: Option<JsString>,
        background_color: Option<JsString>,
        click: Option<Function>,
        icon_position: Option<JsString>,
        icon: Option<NativeImage>,
        label: Option<JsString>,
    ) -> TouchBarButtonOptions {
        TouchBarButtonOptions {
            accessibility_label,
            background_color,
            click,
            icon_position,
            icon,
            label,
        }
    }

    #[wasm_bindgen(getter, js_name = "accessibilityLabel")]
    pub fn accessibility_label(&self) -> Option<JsString> {
        self.accessibility_label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_accessibility_label(&mut self, value: Option<JsString>) {
        self.accessibility_label = value;
    }

    #[wasm_bindgen(getter, js_name = "backgroundColor")]
    pub fn background_color(&self) -> Option<JsString> {
        self.background_color.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_background_color(&mut self, value: Option<JsString>) {
        self.background_color = value;
    }

    #[wasm_bindgen(getter)]
    pub fn click(&self) -> Option<Function> {
        self.click.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_click(&mut self, value: Option<Function>) {
        self.click = value;
    }

    #[wasm_bindgen(getter, js_name = "iconPosition")]
    pub fn icon_position(&self) -> Option<JsString> {
        self.icon_position.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon_position(&mut self, value: Option<JsString>) {
        self.icon_position = value;
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
