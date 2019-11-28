use crate::interface::TouchBarGroupOptions;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-group
    pub type TouchBarGroup;

    // Constructor

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarGroupOptions) -> TouchBarGroup;
}
