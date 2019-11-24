use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type ContentTracing;

    #[wasm_bindgen(js_name = "contentTracing")]
    pub static content_tracing: ContentTracing;
}
