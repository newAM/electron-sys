use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Rectangle {
    height: usize,
    width: usize,
    x: usize,
    y: usize,
}

#[wasm_bindgen]
impl Rectangle {
    #[wasm_bindgen(constructor)]
    pub fn new(height: usize, width: usize, x: usize, y: usize) -> Rectangle {
        Rectangle { height, width, x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn height(self) -> usize {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(mut self, value: usize) {
        self.height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn width(self) -> usize {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(mut self, value: usize) {
        self.width = value;
    }

    #[wasm_bindgen(getter)]
    pub fn x(self) -> usize {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(mut self, value: usize) {
        self.x = value;
    }

    #[wasm_bindgen(getter)]
    pub fn y(self) -> usize {
        self.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(mut self, value: usize) {
        self.y = value;
    }
}
