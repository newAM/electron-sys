use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct ToPngOptions {
    scale_factor: Option<f32>,
}

#[wasm_bindgen]
impl ToPngOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(scale_factor: Option<f32>) -> ToPngOptions {
        ToPngOptions { scale_factor }
    }

    #[wasm_bindgen(getter, js_name = "scaleFactor")]
    pub fn scale_factor(self) -> Option<f32> {
        self.scale_factor
    }

    #[wasm_bindgen(setter)]
    pub fn set_scale_factor(mut self, value: Option<f32>) {
        self.scale_factor = value;
    }
}
