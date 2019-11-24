use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type PowerMonitor;

    #[wasm_bindgen(js_name = "powerMonitor")]
    pub static power_monitor: PowerMonitor;
}
