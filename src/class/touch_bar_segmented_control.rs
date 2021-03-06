use crate::interface::TouchBarSegmentedControlOptions;
use js_sys::JsString;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-segmented-control
    pub type TouchBarSegmentedControl;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarSegmentedControlOptions) -> TouchBarSegmentedControl;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn segments(this: &TouchBarSegmentedControl) -> Box<[JsValue]>;

    #[wasm_bindgen(method, getter)]
    pub fn set_segments(this: &TouchBarSegmentedControl, value: Box<[JsValue]>);

    #[wasm_bindgen(method, getter, js_name = "segment_style")]
    pub fn segment_style(this: &TouchBarSegmentedControl) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "segment_style")]
    pub fn set_segment_style(this: &TouchBarSegmentedControl, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "selected_index")]
    pub fn selected_index(this: &TouchBarSegmentedControl) -> usize;

    #[wasm_bindgen(method, setter, js_name = "selected_index")]
    pub fn set_selected_index(this: &TouchBarSegmentedControl, value: usize);
}
