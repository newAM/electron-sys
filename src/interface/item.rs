use crate::class::NativeImage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    file: Box<[JsValue]>,
    icon: NativeImage,
}

#[wasm_bindgen]
impl Item {
    #[wasm_bindgen(constructor)]
    pub fn new(file: Box<[JsValue]>, icon: NativeImage) -> Item {
        Item { file, icon }
    }

    #[wasm_bindgen(getter)]
    pub fn file(&self) -> Box<[JsValue]> {
        self.file.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_file(&mut self, value: Box<[JsValue]>) {
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
