use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CreateFromBitmapOptions {
    height: usize,
    scale_factor: Option<f32>,
    width: usize,
}

#[wasm_bindgen]
impl CreateFromBitmapOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(height: usize, scale_factor: Option<f32>, width: usize) -> CreateFromBitmapOptions {
        CreateFromBitmapOptions {
            height,
            scale_factor,
            width,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, value: usize) {
        self.height = value;
    }

    #[wasm_bindgen(getter, js_name = "scaleFactor")]
    pub fn scale_factor(&self) -> Option<f32> {
        self.scale_factor
    }

    #[wasm_bindgen(setter)]
    pub fn set_scale_factor(&mut self, value: Option<f32>) {
        self.scale_factor = value;
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, value: usize) {
        self.width = value;
    }
}
