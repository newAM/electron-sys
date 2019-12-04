use crate::class::NativeImage;
use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ThumbarButton {
    click: Function,
    flags: Option<Array>,
    icon: NativeImage,
    tooltip: Option<Array>,
}

#[wasm_bindgen]
impl ThumbarButton {
    #[wasm_bindgen]
    pub fn new(click: Function, flags: Option<Array>, icon: NativeImage, tooltip: Option<Array>) -> ThumbarButton {
        ThumbarButton {
            click,
            flags,
            icon,
            tooltip,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn click(&self) -> Function {
        self.click.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_click(&mut self, value: Function) {
        self.click = value;
    }

    #[wasm_bindgen(getter)]
    pub fn flags(&self) -> Option<Array> {
        self.flags.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_flags(&mut self, value: Option<Array>) {
        self.flags = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> NativeImage {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: NativeImage) {
        self.icon = value;
    }

    #[wasm_bindgen(getter)]
    pub fn tooltip(&self) -> Option<Array> {
        self.tooltip.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_tooltip(&mut self, value: Option<Array>) {
        self.tooltip = value;
    }
}
