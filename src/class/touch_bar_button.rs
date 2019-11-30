use crate::{class::NativeImage, interface::TouchBarButtonOptions};
use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-button
    pub type TouchBarButton;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarButtonOptions) -> TouchBarButton;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "accessibilityLabel")]
    pub fn accessibility_label(this: &TouchBarButton) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "accessibilityLabel")]
    pub fn set_accessibility_label(this: &TouchBarButton, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "backgroundColor")]
    pub fn background_color(this: &TouchBarButton) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "backgroundColor")]
    pub fn set_background_color(this: &TouchBarButton, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn icon(this: &TouchBarButton) -> NativeImage;

    #[wasm_bindgen(method, setter)]
    pub fn set_icon(this: &TouchBarButton, value: NativeImage);

    #[wasm_bindgen(method, getter)]
    pub fn label(this: &TouchBarButton) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_label(this: &TouchBarButton, value: JsString);
}
