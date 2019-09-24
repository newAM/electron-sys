use crate::event_emitter::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    /// The electron app interface type.
    #[wasm_bindgen(extends = EventEmitter)]
    pub type App;

    /// The electron app.
    pub static app: App;
}
