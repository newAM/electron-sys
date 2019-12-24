use crate::interface::{BlinkMemoryInfo, CpuUsage, HeapStatistics, IoCounters, ProcessMemoryInfo, SystemMemoryInfo};
use js_sys::JsString;
use node_sys::Process as NodeProcess;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = NodeProcess)]
    #[derive(Clone, Debug)]
    pub type Process;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn crash(this: &Process);

    #[wasm_bindgen(method, js_name = "getBlinkMemoryInfo")]
    pub fn get_blink_memory_info(this: &Process) -> BlinkMemoryInfo;

    #[wasm_bindgen(method, js_name = "getCPUUsage")]
    pub fn get_cpu_usage(this: &Process) -> CpuUsage;

    #[wasm_bindgen(method, js_name = "getCreationTime")]
    pub fn get_creation_time(this: &Process) -> Option<u64>;

    #[wasm_bindgen(method, js_name = "getHeapStatistics")]
    pub fn get_heap_statistics(this: &Process) -> HeapStatistics;

    #[wasm_bindgen(method, js_name = "getIOCounters")]
    pub fn get_io_counters(this: &Process) -> IoCounters;

    #[wasm_bindgen(method, js_name = "getProcessMemoryInfo")]
    pub fn get_process_memory_info(this: &Process) -> ProcessMemoryInfo;

    #[wasm_bindgen(method, js_name = "getSystemMemoryInfo")]
    pub fn get_system_memory_info(this: &Process) -> SystemMemoryInfo;

    #[wasm_bindgen(method, js_name = "getSystemVersion")]
    pub fn get_system_version(this: &Process) -> JsString;

    #[wasm_bindgen(method)]
    pub fn hang(this: &Process);

    #[wasm_bindgen(method, js_name = "setFdLimit")]
    pub fn set_fd_limit(this: &Process, max_descriptors: u64);

    #[wasm_bindgen(method, js_name = "takeHeapSnapshot")]
    pub fn take_heap_snapshot(this: &Process, file_path: &str) -> bool;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)] // readonly
    pub fn chrome(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "defaultApp")] // readonly
    pub fn default_app(this: &Process) -> bool;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn electron(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "enablePromiseApi")]
    pub fn enable_promise_api(this: &Process) -> bool;

    #[wasm_bindgen(method, setter, js_name = "enablePromiseApi")]
    pub fn set_enable_promise_api(this: &Process, value: bool);

    #[wasm_bindgen(method, getter, js_name = "isMainFrame")] // readonly
    pub fn is_main_frame(this: &Process) -> bool;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn mas(this: &Process) -> bool;

    #[wasm_bindgen(method, getter, js_name = "noAsar")]
    pub fn no_asar(this: &Process) -> bool;

    #[wasm_bindgen(method, setter, js_name = "noAsar")]
    pub fn set_no_asar(this: &Process, value: bool);

    #[wasm_bindgen(method, getter, js_name = "noDeprecation")]
    pub fn no_deprecation(this: &Process) -> bool;

    #[wasm_bindgen(method, setter, js_name = "noDeprecation")]
    pub fn set_no_deprecation(this: &Process, value: bool);

    #[wasm_bindgen(method, getter, js_name = "resourcesPath")] // readonly
    pub fn resources_path(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn sandboxed(this: &Process) -> bool;

    #[wasm_bindgen(method, getter, js_name = "throwDeprecation")]
    pub fn throw_deprecation(this: &Process) -> bool;

    #[wasm_bindgen(method, setter, js_name = "throwDeprecation")]
    pub fn set_throw_deprecation(this: &Process, value: bool);

    #[wasm_bindgen(method, getter, js_name = "traceDeprecation")]
    pub fn trace_deprecation(this: &Process) -> bool;

    #[wasm_bindgen(method, setter, js_name = "traceDeprecation")]
    pub fn set_trace_deprecation(this: &Process, value: bool);

    #[wasm_bindgen(method, getter, js_name = "traceProcessWarnings")]
    pub fn trace_process_warnings(this: &Process) -> bool;

    #[wasm_bindgen(method, setter, js_name = "traceProcessWarnings")]
    pub fn set_trace_process_warnings(this: &Process, value: bool);

    #[wasm_bindgen(method, getter, js_name = "type")] // readonly
    pub fn kind(this: &Process) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "windowsStore")] // readonly
    pub fn windows_store(this: &Process) -> bool;
}
