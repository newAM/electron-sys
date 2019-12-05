use crate::interface::{CpuUsage, MemoryInfo};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type ProcessMetric;

    #[wasm_bindgen(method, getter)]
    pub fn cpu(this: &ProcessMetric) -> CpuUsage;

    #[wasm_bindgen(method, setter)]
    pub fn set_cpu(this: &ProcessMetric, value: CpuUsage);

    #[wasm_bindgen(method, getter, js_name = "creationTime")]
    pub fn creation_time(this: &ProcessMetric) -> u32;

    #[wasm_bindgen(method, setter, js_name = "creationTime")]
    pub fn set_creation_time(this: &ProcessMetric, value: u32);

    #[wasm_bindgen(method, getter, js_name = "integrityLevel")]
    pub fn integrity_level(this: &ProcessMetric) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "integrityLevel")]
    pub fn set_integrity_level(this: &ProcessMetric, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &ProcessMetric) -> MemoryInfo;

    #[wasm_bindgen(method, setter)]
    pub fn set_memory(this: &ProcessMetric, value: MemoryInfo);

    #[wasm_bindgen(method, getter)]
    pub fn pid(this: &ProcessMetric) -> u32;

    #[wasm_bindgen(method, setter)]
    pub fn set_pid(this: &ProcessMetric, value: u32);

    #[wasm_bindgen(method, getter)]
    pub fn sandboxed(this: &ProcessMetric) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_sandboxed(this: &ProcessMetric, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn kind(this: &ProcessMetric) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "type")]
    pub fn set_kind(this: &ProcessMetric, value: JsString);
}
