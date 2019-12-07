use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type ContentTracing;

    #[wasm_bindgen(js_name = "contentTracing")]
    pub static content_tracing: ContentTracing;

    #[must_use]
    #[wasm_bindgen(method, js_name = "getCategories")]
    pub fn get_categories(this: &ContentTracing) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "getTraceBufferUsage")]
    pub fn get_trace_buffer_usage(this: &ContentTracing) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "startRecording")]
    pub fn start_recording(this: &ContentTracing) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "stopRecording")]
    pub fn stop_recording(this: &ContentTracing) -> Promise;
}
