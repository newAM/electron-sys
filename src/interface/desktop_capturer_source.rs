use crate::class::NativeImage;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type DesktopCapturerSource;

    #[wasm_bindgen(method, getter, js_name = "appIcon")]
    pub fn app_icon(this: &DesktopCapturerSource) -> NativeImage;

    #[wasm_bindgen(method, setter, js_name = "appIcon")]
    pub fn set_app_icon(this: &DesktopCapturerSource, value: NativeImage);

    #[wasm_bindgen(method, getter, js_name = "displayId")]
    pub fn display_id(this: &DesktopCapturerSource) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "displayId")]
    pub fn set_display_id(this: &DesktopCapturerSource, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &DesktopCapturerSource) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_id(this: &DesktopCapturerSource, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &DesktopCapturerSource) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &DesktopCapturerSource, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn thumbnail(this: &DesktopCapturerSource) -> NativeImage;

    #[wasm_bindgen(method, setter)]
    pub fn set_thumbnail(this: &DesktopCapturerSource, value: NativeImage);
}
