use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type ProcessMemoryInfo;

    #[wasm_bindgen(method, getter)]
    pub fn private(this: &ProcessMemoryInfo) -> usize;

    #[wasm_bindgen(method, setter)]
    pub fn set_private(this: &ProcessMemoryInfo) -> usize;

    #[wasm_bindgen(method, getter, js_name = "residentSet")]
    pub fn resident_set(this: &ProcessMemoryInfo) -> usize;

    #[wasm_bindgen(method, setter)]
    pub fn set_resident_set(this: &ProcessMemoryInfo) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn shared(this: &ProcessMemoryInfo) -> usize;

    #[wasm_bindgen(method, setter)]
    pub fn set_shared(this: &ProcessMemoryInfo) -> usize;
}
