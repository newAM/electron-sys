use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(self) -> usize {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(mut self, value: usize) {
        self.y = value;
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
