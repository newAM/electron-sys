use crate::interface::SourcesOptions;
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type DesktopCapturer;

    #[wasm_bindgen(js_name = "desktopCapturer")]
    pub static desktop_capturer: DesktopCapturer;

    #[must_use]
    #[wasm_bindgen(method, js_name = "getSources")]
    pub fn get_sources(this: &DesktopCapturer, options: SourcesOptions) -> Promise;
}
