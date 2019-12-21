use crate::interface::StartLoggingOptions;
use js_sys::{JsString, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type NetLog;

    //******************//
    // Instance Methods //
    //******************//

    #[must_use]
    #[wasm_bindgen(method, js_name = "startLogging")]
    pub fn start_logging(this: &NetLog, path: &str, options: StartLoggingOptions) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "stopLogging")]
    pub fn stop_logging(this: &NetLog) -> Promise;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "currentlyLogging")] // readonly
    pub fn currently_logging(this: &NetLog) -> bool;

    #[wasm_bindgen(method, getter, js_name = "currentlyLoggingPath")] // readonly
    pub fn currently_logging_path(this: &NetLog) -> JsString;
}
