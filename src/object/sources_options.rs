use crate::object::Size;
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SourcesOptions {
    types: Array,
    thumbnail_size: Option<Size>,
    fetch_window_icons: Option<bool>,
}

#[wasm_bindgen]
impl SourcesOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(types: Array, thumbnail_size: Option<Size>, fetch_window_icons: Option<bool>) -> SourcesOptions {
        SourcesOptions {
            types,
            thumbnail_size,
            fetch_window_icons,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn types(&self) -> Array {
        self.types.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_types(&mut self, types: Array) {
        self.types = types;
    }

    #[wasm_bindgen(getter)]
    pub fn thumbnail_size(&self) -> Option<Size> {
        self.thumbnail_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_thumbnail_size(&mut self, thumbnail_size: Option<Size>) {
        self.thumbnail_size = thumbnail_size;
    }

    #[wasm_bindgen(getter)]
    pub fn fetch_window_icons(&self) -> Option<bool> {
        self.fetch_window_icons.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_fetch_window_icons(&mut self, fetch_window_icons: Option<bool>) {
        self.fetch_window_icons = fetch_window_icons;
    }
}
