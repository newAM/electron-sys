use crate::interface::{Rectangle, Size};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Display {
    accelerometer_support: JsString,
    bounds: Rectangle,
    color_depth: u32,
    color_space: JsString,
    depth_per_component: u32,
    id: u32,
    internal: bool,
    monochrome: bool,
    rotation: u32,
    scale_factor: f32,
    size: Size,
    touch_support: JsString,
    work_area_size: Size,
    work_area: Rectangle,
}

#[wasm_bindgen]
impl Display {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        accelerometer_support: JsString,
        bounds: Rectangle,
        color_depth: u32,
        color_space: JsString,
        depth_per_component: u32,
        id: u32,
        internal: bool,
        monochrome: bool,
        rotation: u32,
        scale_factor: f32,
        size: Size,
        touch_support: JsString,
        work_area_size: Size,
        work_area: Rectangle,
    ) -> Display {
        Display {
            accelerometer_support,
            bounds,
            color_depth,
            color_space,
            depth_per_component,
            id,
            internal,
            monochrome,
            rotation,
            scale_factor,
            size,
            touch_support,
            work_area_size,
            work_area,
        }
    }

    #[wasm_bindgen(getter, js_name = "accelerometerSupport")]
    pub fn accelerometer_support(&self) -> JsString {
        self.accelerometer_support.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_accelerometer_support(&mut self, value: JsString) {
        self.accelerometer_support = value;
    }

    #[wasm_bindgen(getter)]
    pub fn bounds(&self) -> Rectangle {
        self.bounds.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_bounds(&mut self, value: Rectangle) {
        self.bounds = value;
    }

    #[wasm_bindgen(getter, js_name = "colorDepth")]
    pub fn color_depth(&self) -> u32 {
        self.color_depth
    }

    #[wasm_bindgen(setter)]
    pub fn set_color_depth(&mut self, value: u32) {
        self.color_depth = value;
    }

    #[wasm_bindgen(getter, js_name = "colorSpace")]
    pub fn color_space(&self) -> JsString {
        self.color_space.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_color_space(&mut self, value: JsString) {
        self.color_space = value;
    }

    #[wasm_bindgen(getter, js_name = "depthPerComponent")]
    pub fn depth_per_component(&self) -> u32 {
        self.depth_per_component
    }

    #[wasm_bindgen(setter)]
    pub fn set_depth_per_component(&mut self, value: u32) {
        self.depth_per_component = value;
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u32 {
        self.id
    }

    #[wasm_bindgen(setter)]
    pub fn set_id(&mut self, value: u32) {
        self.id = value;
    }

    #[wasm_bindgen(getter)]
    pub fn internal(&self) -> bool {
        self.internal
    }

    #[wasm_bindgen(setter)]
    pub fn set_internal(&mut self, value: bool) {
        self.internal = value;
    }

    #[wasm_bindgen(getter)]
    pub fn monochrome(&self) -> bool {
        self.monochrome
    }

    #[wasm_bindgen(setter)]
    pub fn set_monochrome(&mut self, value: bool) {
        self.monochrome = value;
    }

    #[wasm_bindgen(getter)]
    pub fn rotation(&self) -> u32 {
        self.rotation
    }

    #[wasm_bindgen(setter)]
    pub fn set_rotation(&mut self, value: u32) {
        self.rotation = value;
    }

    #[wasm_bindgen(getter, js_name = "scaleFactor")]
    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    #[wasm_bindgen(setter)]
    pub fn set_scale_factor(&mut self, value: f32) {
        self.scale_factor = value;
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> Size {
        self.size
    }

    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, value: Size) {
        self.size = value;
    }

    #[wasm_bindgen(getter, js_name = "touchSupport")]
    pub fn touch_support(&self) -> JsString {
        self.touch_support.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_touch_support(&mut self, value: JsString) {
        self.touch_support = value;
    }

    #[wasm_bindgen(getter, js_name = "workAreaSize")]
    pub fn work_area_size(&self) -> Size {
        self.work_area_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_work_area_size(&mut self, value: Size) {
        self.work_area_size = value;
    }

    #[wasm_bindgen(getter, js_name = "workArea")]
    pub fn work_area(&self) -> Rectangle {
        self.work_area.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_work_area(&mut self, value: Rectangle) {
        self.work_area = value;
    }
}
