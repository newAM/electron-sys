use crate::interface::{Point, Size};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

// NOTE: called `Parameters` upstream.

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct EnableDeviceEmulationOptions {
    device_scale_factor: f32,
    scale: f32,
    screen_position: JsString,
    screen_size: Size,
    view_position: Point,
    view_size: Size,
}

#[wasm_bindgen]
impl EnableDeviceEmulationOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        device_scale_factor: f32,
        scale: f32,
        screen_position: JsString,
        screen_size: Size,
        view_position: Point,
        view_size: Size,
    ) -> EnableDeviceEmulationOptions {
        EnableDeviceEmulationOptions {
            device_scale_factor,
            scale,
            screen_position,
            screen_size,
            view_position,
            view_size,
        }
    }

    #[wasm_bindgen(getter, js_name = "deviceScaleFactor")]
    pub fn device_scale_factor(&self) -> f32 {
        self.device_scale_factor
    }

    #[wasm_bindgen(setter)]
    pub fn set_device_scale_factor(&mut self, value: f32) {
        self.device_scale_factor = value;
    }

    #[wasm_bindgen(getter)]
    pub fn scale(&self) -> f32 {
        self.scale
    }

    #[wasm_bindgen(setter)]
    pub fn set_scale(&mut self, value: f32) {
        self.scale = value;
    }

    #[wasm_bindgen(getter, js_name = "screenPosition")]
    pub fn screen_position(&self) -> JsString {
        self.screen_position.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_screen_position(&mut self, value: JsString) {
        self.screen_position = value;
    }

    #[wasm_bindgen(getter, js_name = "screenSize")]
    pub fn screen_size(&self) -> Size {
        self.screen_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_screen_size(&mut self, value: Size) {
        self.screen_size = value;
    }

    #[wasm_bindgen(getter, js_name = "viewPosition")]
    pub fn view_position(&self) -> Point {
        self.view_position
    }

    #[wasm_bindgen(setter)]
    pub fn set_view_position(&mut self, value: Point) {
        self.view_position = value;
    }

    #[wasm_bindgen(getter, js_name = "viewSize")]
    pub fn view_size(&self) -> Size {
        self.view_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_view_size(&mut self, value: Size) {
        self.view_size = value;
    }
}
