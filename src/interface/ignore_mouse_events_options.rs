use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IgnoreMouseEventsOptions {
    forward: Option<bool>,
}

#[wasm_bindgen]
impl IgnoreMouseEventsOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(forward: Option<bool>) -> IgnoreMouseEventsOptions {
        IgnoreMouseEventsOptions { forward }
    }

    pub fn new() -> IgnoreMouseEventsOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn forward(self) -> Option<bool> {
        self.forward
    }

    #[wasm_bindgen(setter)]
    pub fn set_forward(mut self, value: Option<bool>) {
        self.forward = value;
    }
}
