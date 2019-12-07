use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type PowerMonitor;

    #[wasm_bindgen(js_name = "powerMonitor")]
    pub static power_monitor: PowerMonitor;

    #[wasm_bindgen(js_name = "getSystemIdleState")]
    pub fn get_system_idle_state(this: &PowerMonitor, idle_threshold: u32) -> JsString;

    #[wasm_bindgen(js_name = "getSystemIdleTime")]
    pub fn get_system_idle_time(this: &PowerMonitor) -> u32;
}
