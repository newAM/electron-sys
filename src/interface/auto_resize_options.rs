use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AutoResizeOptions {
    height: Option<bool>,
    horizontal: Option<bool>,
    vertical: Option<bool>,
    width: Option<bool>,
}

#[wasm_bindgen]
impl AutoResizeOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        height: Option<bool>,
        horizontal: Option<bool>,
        vertical: Option<bool>,
        width: Option<bool>,
    ) -> AutoResizeOptions {
        AutoResizeOptions {
            height,
            horizontal,
            vertical,
            width,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn height(self) -> Option<bool> {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(mut self, value: Option<bool>) {
        self.height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn horizontal(self) -> Option<bool> {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_horizontal(mut self, value: Option<bool>) {
        self.height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn vertical(self) -> Option<bool> {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_vertical(mut self, value: Option<bool>) {
        self.height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn width(self) -> Option<bool> {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(mut self, value: Option<bool>) {
        self.height = value;
    }
}
