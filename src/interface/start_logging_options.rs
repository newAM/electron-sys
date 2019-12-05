use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct StartLoggingOptions {
    capture_mode: Option<JsString>,
    max_file_size: Option<usize>,
}

#[wasm_bindgen]
impl StartLoggingOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(capture_mode: Option<JsString>, max_file_size: Option<usize>) -> StartLoggingOptions {
        StartLoggingOptions {
            capture_mode,
            max_file_size,
        }
    }

    #[wasm_bindgen(getter, js_name = "captureMode")]
    pub fn capture_mode(&self) -> Option<JsString> {
        self.capture_mode.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_capture_mode(&mut self, value: Option<JsString>) {
        self.capture_mode = value;
    }

    #[wasm_bindgen(getter, js_name = "maxFileSize")]
    pub fn max_file_size(&self) -> Option<usize> {
        self.max_file_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_file_size(&mut self, value: Option<usize>) {
        self.max_file_size = value;
    }
}
