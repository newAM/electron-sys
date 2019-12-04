use crate::class::NativeImage;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Item {
    file: Array,
    icon: NativeImage,
}

#[wasm_bindgen]
impl Item {
    #[wasm_bindgen(constructor)]
    pub fn new(file: Array, icon: NativeImage) -> Item {
        Item { file, icon }
    }

    #[wasm_bindgen(getter)]
    pub fn file(&self) -> Array {
        self.file.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_file(&mut self, value: Array) {
        self.file = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> NativeImage {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: NativeImage) {
        self.icon = value;
    }
}
