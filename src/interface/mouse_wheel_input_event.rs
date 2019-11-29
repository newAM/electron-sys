use crate::interface::MouseInputEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = MouseInputEvent)]
    #[derive(Clone, Debug, PartialEq)]
    pub type MouseWheelInputEvent;
}
