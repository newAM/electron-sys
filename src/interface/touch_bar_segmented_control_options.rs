use js_sys::{Array, Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct TouchBarSegmentedControlOptions {
    change: Option<Function>,
    mode: Option<JsString>,
    segment_style: Option<JsString>,
    segments: Array,
    selected_index: Option<usize>,
}

#[wasm_bindgen]
impl TouchBarSegmentedControlOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        change: Option<Function>,
        mode: Option<JsString>,
        segment_style: Option<JsString>,
        segments: Array,
        selected_index: Option<usize>,
    ) -> TouchBarSegmentedControlOptions {
        TouchBarSegmentedControlOptions {
            change,
            mode,
            segment_style,
            segments,
            selected_index,
        }
    }

    pub fn new(segments: Array) -> TouchBarSegmentedControlOptions {
        let change = Default::default();
        let mode = Default::default();
        let segment_style = Default::default();
        let selected_index = Default::default();
        TouchBarSegmentedControlOptions::new_with_values(change, mode, segment_style, segments, selected_index)
    }

    #[wasm_bindgen(getter)]
    pub fn change(&self) -> Option<Function> {
        self.change.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_change(&mut self, value: Option<Function>) {
        self.change = value;
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> Option<JsString> {
        self.mode.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_mode(&mut self, value: Option<JsString>) {
        self.mode = value;
    }

    #[wasm_bindgen(getter, js_name = "segmentStyle")]
    pub fn segment_style(&self) -> Option<JsString> {
        self.segment_style.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_segment_style(&mut self, value: Option<JsString>) {
        self.segment_style = value;
    }

    #[wasm_bindgen(getter)]
    pub fn segments(&self) -> Array {
        self.segments.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_segments(&mut self, value: Array) {
        self.segments = value;
    }

    #[wasm_bindgen(getter)]
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    #[wasm_bindgen(setter)]
    pub fn set_selected_index(&mut self, value: Option<usize>) {
        self.selected_index = value;
    }
}
