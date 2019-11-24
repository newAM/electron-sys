use crate::object::input_event::InputEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = InputEvent)]
    pub type MouseInputEvent;
}
