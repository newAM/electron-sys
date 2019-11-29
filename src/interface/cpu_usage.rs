use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "CPUUsage")]
    #[derive(Clone, Debug, PartialEq)]
    pub type CpuUsage;
}
