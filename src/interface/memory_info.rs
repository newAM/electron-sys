use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type MemoryInfo;

    #[wasm_bindgen(method, getter, js_name = "peakWorkingSetSize")]
    pub fn peak_working_set_size(this: &MemoryInfo) -> usize;

    #[wasm_bindgen(method, setter, js_name = "peakWorkingSetSize")]
    pub fn set_peak_working_set_size(this: &MemoryInfo) -> usize;

    #[wasm_bindgen(method, getter, js_name = "privateBytes")]
    pub fn private_bytes(this: &MemoryInfo) -> usize;

    #[wasm_bindgen(method, setter, js_name = "privateBytes")]
    pub fn set_private_bytes(this: &MemoryInfo) -> usize;

    #[wasm_bindgen(method, getter, js_name = "workingSetSize")]
    pub fn working_set_size(this: &MemoryInfo) -> usize;

    #[wasm_bindgen(method, setter, js_name = "workingSetSize")]
    pub fn set_working_set_size(this: &MemoryInfo) -> usize;
}
