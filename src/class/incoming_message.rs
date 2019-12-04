use js_sys::{JsString, Map};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/incoming-message
    pub type IncomingMessage;

    // Instance Properties

    #[wasm_bindgen(method, getter)]
    pub fn headers(this: &IncomingMessage) -> Map;

    #[wasm_bindgen(method, getter, js_name = "httpVersion")]
    pub fn http_version(this: &IncomingMessage) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "httpVersion")]
    pub fn set_http_version(this: &IncomingMessage, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "httpVersionMajor")]
    pub fn http_version_major(this: &IncomingMessage) -> u32;

    #[wasm_bindgen(method, setter, js_name = "httpVersionMajor")]
    pub fn set_http_version_major(this: &IncomingMessage, value: u32);

    #[wasm_bindgen(method, getter, js_name = "httpVersionMinor")]
    pub fn http_version_minor(this: &IncomingMessage) -> u32;

    #[wasm_bindgen(method, setter, js_name = "httpVersionMinor")]
    pub fn set_http_version_minor(this: &IncomingMessage, value: u32);

    #[wasm_bindgen(method, getter, js_name = "statusCode")]
    pub fn status_code(this: &IncomingMessage) -> i32;

    #[wasm_bindgen(method, setter, js_name = "statusCode")]
    pub fn set_status_code(this: &IncomingMessage, value: i32);

    #[wasm_bindgen(method, getter, js_name = "statusMessage")]
    pub fn status_message(this: &IncomingMessage) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "statusMessage")]
    pub fn set_status_message(this: &IncomingMessage, value: JsString);
}
