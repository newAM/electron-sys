use crate::interface::TouchBarSpacerOptions;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-spacer
    pub type TouchBarSpacer;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarSpacerOptions) -> TouchBarSpacer;
}
