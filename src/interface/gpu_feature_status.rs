use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "GPUFeatureStatus")]
    #[derive(Clone, Debug, PartialEq)]
    pub type GpuFeatureStatus;
}
