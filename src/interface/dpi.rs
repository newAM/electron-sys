use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Dpi {
    horizontal: u32,
    vertical: u32,
}

#[wasm_bindgen]
impl Dpi {
    #[wasm_bindgen(constructor)]
    pub fn new(horizontal: u32, vertical: u32) -> Dpi {
        Dpi { horizontal, vertical }
    }

    #[wasm_bindgen(getter)]
    pub fn horizontal(self) -> u32 {
        self.horizontal
    }

    #[wasm_bindgen(setter)]
    pub fn set_horizontal(mut self, value: u32) {
        self.horizontal = value;
    }

    #[wasm_bindgen(getter)]
    pub fn vertical(self) -> u32 {
        self.horizontal
    }

    #[wasm_bindgen(setter)]
    pub fn set_vertical(mut self, value: u32) {
        self.vertical = value;
    }
}
