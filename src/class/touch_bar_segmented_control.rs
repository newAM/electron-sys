use crate::interface::TouchBarSegmentedControlOptions;
use js_sys::{Array, JsString};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-segmented-control
    pub type TouchBarSegmentedControl;

    // Constructor

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarSegmentedControlOptions) -> TouchBarSegmentedControl;

    // Instance Properties

    #[wasm_bindgen(getter)]
    pub fn segments(this: &TouchBarSegmentedControl) -> Array;

    #[wasm_bindgen(getter)]
    pub fn set_segments(this: &TouchBarSegmentedControl, value: &Array);

    #[wasm_bindgen(getter, js_name = "segment_style")]
    pub fn segment_style(this: &TouchBarSegmentedControl) -> JsString;

    #[wasm_bindgen(setter, js_name = "segment_style")]
    pub fn set_segment_style(this: &TouchBarSegmentedControl, value: JsString);

    #[wasm_bindgen(getter, js_name = "selected_index")]
    pub fn selected_index(this: &TouchBarSegmentedControl) -> usize;

    #[wasm_bindgen(setter, js_name = "selected_index")]
    pub fn set_selected_index(this: &TouchBarSegmentedControl, value: usize);
}
