use crate::interface::Size;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct SourcesOptions {
    types: Box<[JsValue]>,
    thumbnail_size: Option<Size>,
    fetch_window_icons: Option<bool>,
}

#[wasm_bindgen]
impl SourcesOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        types: Box<[JsValue]>,
        thumbnail_size: Option<Size>,
        fetch_window_icons: Option<bool>,
    ) -> SourcesOptions {
        SourcesOptions {
            types,
            thumbnail_size,
            fetch_window_icons,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn types(&self) -> Box<[JsValue]> {
        self.types.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_types(&mut self, value: Box<[JsValue]>) {
        self.types = value;
    }

    #[wasm_bindgen(getter)]
    pub fn thumbnail_size(&self) -> Option<Size> {
        self.thumbnail_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_thumbnail_size(&mut self, value: Option<Size>) {
        self.thumbnail_size = value;
    }

    #[wasm_bindgen(getter)]
    pub fn fetch_window_icons(&self) -> Option<bool> {
        self.fetch_window_icons
    }

    #[wasm_bindgen(setter)]
    pub fn set_fetch_window_icons(&mut self, value: Option<bool>) {
        self.fetch_window_icons = value;
    }
}
