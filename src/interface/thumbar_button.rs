use crate::class::NativeImage;
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ThumbarButton {
    click: Function,
    flags: Option<Box<[JsValue]>>,
    icon: NativeImage,
    tooltip: Option<Box<[JsValue]>>,
}

#[wasm_bindgen]
impl ThumbarButton {
    #[wasm_bindgen]
    pub fn new(
        click: Function,
        flags: Option<Box<[JsValue]>>,
        icon: NativeImage,
        tooltip: Option<Box<[JsValue]>>,
    ) -> ThumbarButton {
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
    pub fn flags(&self) -> Option<Box<[JsValue]>> {
        self.flags.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_flags(&mut self, value: Option<Box<[JsValue]>>) {
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
    pub fn tooltip(&self) -> Option<Box<[JsValue]>> {
        self.tooltip.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_tooltip(&mut self, value: Option<Box<[JsValue]>>) {
        self.tooltip = value;
    }
}
