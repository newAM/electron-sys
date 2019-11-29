use crate::interface::InputEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = InputEvent)]
    #[derive(Clone, Debug, PartialEq)]
    pub type MouseInputEvent;
}
