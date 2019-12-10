use crate::interface::{CrashReport, CrashReporterStartOptions};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type CrashReporter;

    #[wasm_bindgen(js_name = "crashReporter")]
    pub static crash_reporter: CrashReporter;

    #[wasm_bindgen(method, js_name = "addExtraParameter")]
    pub fn add_extra_parameter(this: &CrashReporter, key: &JsString, value: &JsString);

    #[wasm_bindgen(method, js_name = "getCrashesDirectory")]
    pub fn get_crashes_directory(this: &CrashReporter) -> JsString;

    #[wasm_bindgen(method, js_name = "getLastCrashReport")]
    pub fn get_last_crash_report(this: &CrashReporter) -> CrashReport;

    #[wasm_bindgen(method, js_name = "getParameters")]
    pub fn get_parameters(this: &CrashReporter);

    #[wasm_bindgen(method, js_name = "getUploadedReports")]
    pub fn get_uploaded_reports(this: &CrashReporter) -> Box<[JsValue]>;

    #[wasm_bindgen(method, js_name = "getUploadToServer")]
    pub fn get_upload_to_server(this: &CrashReporter) -> bool;

    #[wasm_bindgen(method, js_name = "removeExtraParameter")]
    pub fn remove_extra_parameter(this: &CrashReporter, key: &JsString);

    #[wasm_bindgen(method, js_name = "setUploadToServer")]
    pub fn setUploadToServer(this: &CrashReporter, upload_to_server: bool);

    #[wasm_bindgen(method)]
    pub fn start(this: &CrashReporter, options: CrashReporterStartOptions);
}
