use crate::event_emitter::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern "C" {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type App;

    pub static app: App;
}
