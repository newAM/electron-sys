use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CreateFromBufferOptions {
    height: Option<usize>,
    scale_factor: Option<f32>,
    width: Option<usize>,
}

#[wasm_bindgen]
impl CreateFromBufferOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(height: Option<usize>, scale_factor: Option<f32>, width: Option<usize>) -> CreateFromBufferOptions {
        CreateFromBufferOptions {
            height,
            scale_factor,
            width,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> Option<usize> {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, value: Option<usize>) {
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
    pub fn width(&self) -> Option<usize> {
        self.width
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, value: Option<usize>) {
        self.width = value;
    }
}
