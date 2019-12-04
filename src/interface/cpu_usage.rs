use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type CpuUsage;

    #[wasm_bindgen(method, getter, js_name = "idleWakeupsPerSecond")]
    pub fn idle_wakeups_per_second(this: &CpuUsage) -> u32;

    #[wasm_bindgen(method, setter, js_name = "idleWakeupsPerSecond")]
    pub fn set_idle_wakeups_per_second(this: &CpuUsage, value: u32);

    #[wasm_bindgen(method, getter, js_name = "percentCPUUsage")]
    pub fn percent_cpu_usage(this: &CpuUsage) -> u32;

    #[wasm_bindgen(method, setter, js_name = "percentCPUUsage")]
    pub fn set_percent_cpu_usage(this: &CpuUsage, value: u32);
}
