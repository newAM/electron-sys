use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "GPUFeatureStatus")]
    pub type GpuFeatureStatus;
}
