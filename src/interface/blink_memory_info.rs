use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type BlinkMemoryInfo;

    #[wasm_bindgen(method)]
    pub fn allocated(this: &BlinkMemoryInfo) -> u32;

    #[wasm_bindgen(method)]
    pub fn marked(this: &BlinkMemoryInfo) -> u32;

    #[wasm_bindgen(method)]
    pub fn total(this: &BlinkMemoryInfo) -> u32;
}
