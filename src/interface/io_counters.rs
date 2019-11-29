use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "IOCounters")]
    #[derive(Clone, Debug, PartialEq)]
    pub type IoCounters;
}
