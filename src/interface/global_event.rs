use wasm_bindgen::prelude::*;
use web_sys::Event;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Event)]
    pub type GlobalEvent;
}
