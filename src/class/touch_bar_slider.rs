use crate::interface::TouchBarSliderOptions;
use js_sys::{JsString, Number};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-slider
    pub type TouchBarSlider;

    // Constructor

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarSliderOptions) -> TouchBarSlider;

    // Instance Properties

    #[wasm_bindgen(method, getter)]
    pub fn label(this: &TouchBarSlider) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_label(this: &TouchBarSlider, value: &JsString);

    #[wasm_bindgen(method, getter, js_name = "maxValue")]
    pub fn max_value(this: &TouchBarSlider) -> Number;

    #[wasm_bindgen(method, setter, js_name = "maxValue")]
    pub fn set_max_value(this: &TouchBarSlider, value: &Number);

    #[wasm_bindgen(method, getter, js_name = "minValue")]
    pub fn min_value(this: &TouchBarSlider) -> Number;

    #[wasm_bindgen(method, setter, js_name = "minValue")]
    pub fn set_min_value(this: &TouchBarSlider, value: &Number);

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &TouchBarSlider) -> Number;

    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &TouchBarSlider, value: &Number);

}
