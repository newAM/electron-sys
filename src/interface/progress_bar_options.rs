use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ProgressBarOptions {
    mode: Option<JsString>,
}

#[wasm_bindgen]
impl ProgressBarOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(mode: Option<JsString>) -> ProgressBarOptions {
        ProgressBarOptions { mode }
    }

    pub fn new() -> ProgressBarOptions {
        Default::default()
    }
}
