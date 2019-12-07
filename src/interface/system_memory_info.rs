use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type SystemMemoryInfo;

    #[wasm_bindgen(method, getter)]
    pub fn free(this: &SystemMemoryInfo) -> u64;

    #[wasm_bindgen(method, setter)]
    pub fn set_free(this: &SystemMemoryInfo, value: u64);

    #[wasm_bindgen(method, getter, js_name = "swapFree")]
    pub fn swap_free(this: &SystemMemoryInfo) -> u64;

    #[wasm_bindgen(method, setter, js_name = "swapFree")]
    pub fn set_swap_free(this: &SystemMemoryInfo, value: u64);

    #[wasm_bindgen(method, getter, js_name = "swapTotal")]
    pub fn swap_total(this: &SystemMemoryInfo) -> u64;

    #[wasm_bindgen(method, setter, js_name = "swapTotal")]
    pub fn set_swap_total(this: &SystemMemoryInfo, value: u64);

    #[wasm_bindgen(method, getter)]
    pub fn total(this: &SystemMemoryInfo) -> u64;

    #[wasm_bindgen(method, setter)]
    pub fn set_total(this: &SystemMemoryInfo, value: u64);
}
