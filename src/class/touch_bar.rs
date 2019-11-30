use crate::interface::TouchBarOptions;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/touch-bar
    pub type TouchBar;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarOptions) -> TouchBar;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "escapeItem")]
    pub fn escape_item(this: &TouchBar) -> Option<Object>;

    #[wasm_bindgen(method, setter, js_name = "escapeItem")]
    pub fn set_escape_item(this: &TouchBar, value: Option<Object>);
}
