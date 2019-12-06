use crate::class::TouchBar;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchBarGroupOptions {
    items: TouchBar,
}

#[wasm_bindgen]
impl TouchBarGroupOptions {
    #[wasm_bindgen]
    pub fn new(items: TouchBar) -> TouchBarGroupOptions {
        TouchBarGroupOptions { items }
    }

    #[wasm_bindgen(getter)]
    pub fn items(&self) -> TouchBar {
        self.items.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_items(&mut self, value: TouchBar) {
        self.items = value;
    }
}
