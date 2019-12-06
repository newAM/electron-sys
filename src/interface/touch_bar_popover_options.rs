use crate::class::{NativeImage, TouchBar};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchBarPopoverOptions {
    icon: Option<NativeImage>,
    items: TouchBar,
    label: Option<JsString>,
    show_close_button: Option<bool>,
}

#[wasm_bindgen]
impl TouchBarPopoverOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        icon: Option<NativeImage>,
        items: TouchBar,
        label: Option<JsString>,
        show_close_button: Option<bool>,
    ) -> TouchBarPopoverOptions {
        TouchBarPopoverOptions {
            icon,
            items,
            label,
            show_close_button,
        }
    }

    pub fn new(items: TouchBar) -> TouchBarPopoverOptions {
        let icon = Default::default();
        let label = Default::default();
        let show_close_buton = Default::default();
        TouchBarPopoverOptions::new_with_values(icon, items, label, show_close_buton)
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
    pub fn items(&self) -> TouchBar {
        self.items.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_items(&mut self, value: TouchBar) {
        self.items = value;
    }

    #[wasm_bindgen(getter)]
    pub fn label(&self) -> Option<JsString> {
        self.label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_label(&mut self, value: Option<JsString>) {
        self.label = value;
    }

    #[wasm_bindgen(getter, js_name = "showCloseButton")]
    pub fn show_close_button(&self) -> Option<bool> {
        self.show_close_button
    }

    #[wasm_bindgen(setter)]
    pub fn set_show_close_button(&mut self, value: Option<bool>) {
        self.show_close_button = value;
    }
}
