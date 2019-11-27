use crate::interface::mouse_input_event::MouseInputEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = MouseInputEvent)]
    pub type MouseWheelInputEvent;
}
