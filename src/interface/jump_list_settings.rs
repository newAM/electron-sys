use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JumpListSettings {
    min_items: usize,
    removed_items: Array,
}

#[wasm_bindgen]
impl JumpListSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(min_items: usize, removed_items: Array) -> JumpListSettings {
        JumpListSettings {
            min_items,
            removed_items,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn min_items(&self) -> usize {
        self.min_items
    }

    #[wasm_bindgen(setter)]
    pub fn set_min_items(&mut self, value: usize) {
        self.min_items = value;
    }

    #[wasm_bindgen(getter)]
    pub fn removed_items(&self) -> Array {
        self.removed_items.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_removed_items(&mut self, value: Array) {
        self.removed_items = value;
    }
}
