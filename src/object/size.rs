use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Size {
    height: usize,
    width: usize
}

#[wasm_bindgen]
impl Size {
    #[wasm_bindgen(constructor)]
    pub fn new(height: usize, width: usize) -> Size {
        Size { height, width }
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }
}
