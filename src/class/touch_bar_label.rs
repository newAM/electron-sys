use crate::interface::TouchBarLabelOptions;
use js_sys::JsString;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-label
    pub type TouchBarLabel;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(this: TouchBarLabelOptions) -> TouchBarLabel;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "accessibilityLabel")]
    pub fn accessibility_label(this: &TouchBarLabel) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "accessibilityLabel")]
    pub fn set_accessibility_label(this: &TouchBarLabel, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn label(this: &TouchBarLabel) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_label(this: &TouchBarLabel, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "textColor")]
    pub fn text_color(this: &TouchBarLabel) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "textColor")]
    pub fn set_text_color(this: &TouchBarLabel, value: JsString);
}
