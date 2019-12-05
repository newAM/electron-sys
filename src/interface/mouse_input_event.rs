use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

// NOTE: extends InputEvent

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct MouseInputEvent {
    acceleration_ratio_x: Option<i32>,
    acceleration_ratio_y: Option<i32>,
    can_scroll: Option<bool>,
    delta_x: Option<i32>,
    delta_y: Option<i32>,
    has_precise_scrolling_deltas: Option<bool>,
    kind: JsString,
    modifiers: Array,
    wheel_ticks_x: Option<i32>,
    wheel_ticks_y: Option<i32>,
}

#[wasm_bindgen]
impl MouseInputEvent {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        acceleration_ratio_x: Option<i32>,
        acceleration_ratio_y: Option<i32>,
        can_scroll: Option<bool>,
        delta_x: Option<i32>,
        delta_y: Option<i32>,
        has_precise_scrolling_deltas: Option<bool>,
        kind: JsString,
        modifiers: Array,
        wheel_ticks_x: Option<i32>,
        wheel_ticks_y: Option<i32>,
    ) -> MouseInputEvent {
        MouseInputEvent {
            acceleration_ratio_x,
            acceleration_ratio_y,
            can_scroll,
            delta_x,
            delta_y,
            has_precise_scrolling_deltas,
            kind,
            modifiers,
            wheel_ticks_x,
            wheel_ticks_y,
        }
    }

    #[wasm_bindgen(getter, js_name = "accelerationRatioX")]
    pub fn acceleration_ratio_x(&self) -> Option<i32> {
        self.acceleration_ratio_x
    }

    #[wasm_bindgen(setter)]
    pub fn set_acceleration_ratio_x(&mut self, value: Option<i32>) {
        self.acceleration_ratio_x = value;
    }

    #[wasm_bindgen(getter, js_name = "accelerationRatioY")]
    pub fn acceleration_ratio_y(&self) -> Option<i32> {
        self.acceleration_ratio_y
    }

    #[wasm_bindgen(setter)]
    pub fn set_acceleration_ratio_y(&mut self, value: Option<i32>) {
        self.acceleration_ratio_y = value;
    }

    #[wasm_bindgen(getter, js_name = "canScroll")]
    pub fn can_scroll(&self) -> Option<bool> {
        self.can_scroll
    }

    #[wasm_bindgen(setter)]
    pub fn set_can_scroll(&mut self, value: Option<bool>) {
        self.can_scroll = value;
    }

    #[wasm_bindgen(getter, js_name = "deltaX")]
    pub fn delta_x(&self) -> Option<i32> {
        self.delta_x
    }

    #[wasm_bindgen(setter)]
    pub fn set_delta_x(&mut self, value: Option<i32>) {
        self.delta_x = value;
    }

    #[wasm_bindgen(getter, js_name = "deltaY")]
    pub fn delta_y(&self) -> Option<i32> {
        self.delta_y
    }

    #[wasm_bindgen(setter)]
    pub fn set_delta_y(&mut self, value: Option<i32>) {
        self.delta_y = value;
    }

    #[wasm_bindgen(getter, js_name = "hasPreciseScrollingDeltas")]
    pub fn has_precise_scrolling_deltas(&self) -> Option<bool> {
        self.has_precise_scrolling_deltas
    }

    #[wasm_bindgen(setter)]
    pub fn set_has_precise_scrolling_deltas(&mut self, value: Option<bool>) {
        self.has_precise_scrolling_deltas = value;
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> JsString {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: JsString) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn modifiers(&self) -> Array {
        self.modifiers.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_modifiers(&mut self, value: Array) {
        self.modifiers = value;
    }

    #[wasm_bindgen(getter, js_name = "wheelTicksX")]
    pub fn wheel_ticks_x(&self) -> Option<i32> {
        self.wheel_ticks_x
    }

    #[wasm_bindgen(setter)]
    pub fn set_wheel_ticks_x(&mut self, value: Option<i32>) {
        self.wheel_ticks_x = value;
    }

    #[wasm_bindgen(getter, js_name = "wheelTicksY")]
    pub fn wheel_ticks_y(&self) -> Option<i32> {
        self.wheel_ticks_y
    }

    #[wasm_bindgen(setter)]
    pub fn set_wheel_ticks_y(&mut self, value: Option<i32>) {
        self.wheel_ticks_y = value;
    }
}
