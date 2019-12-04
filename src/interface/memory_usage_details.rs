use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type MemoryUsageDetails;

    #[wasm_bindgen(method, getter)]
    pub fn count(this: &MemoryUsageDetails) -> usize;

    #[wasm_bindgen(method, setter)]
    pub fn set_count(this: &MemoryUsageDetails) -> usize;

    #[wasm_bindgen(method, getter, js_name = "liveSize")]
    pub fn live_size(this: &MemoryUsageDetails) -> usize;

    #[wasm_bindgen(method, setter)]
    pub fn set_live_size(this: &MemoryUsageDetails) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &MemoryUsageDetails) -> usize;

    #[wasm_bindgen(method, setter)]
    pub fn set_size(this: &MemoryUsageDetails) -> usize;
}
