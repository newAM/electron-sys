use crate::interface::WebRequestFilter;
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/web-request
    pub type WebRequest;

    // Instance Methods

    #[wasm_bindgen(method, js_name = "onBeforeRedirect")]
    pub fn on_before_redirect(this: &WebRequest, filter: WebRequestFilter, listener: Option<&Function>);

    #[wasm_bindgen(method, js_name = "onBeforeRequest")]
    pub fn on_before_request(this: &WebRequest, filter: WebRequestFilter, listener: Option<&Function>);

    #[wasm_bindgen(method, js_name = "onBeforeSendHeaders")]
    pub fn on_before_send_headers(this: &WebRequest, filter: WebRequestFilter, listener: Option<&Function>);

    #[wasm_bindgen(method, js_name = "onCompleted")]
    pub fn on_completed(this: &WebRequest, filter: WebRequestFilter, listener: Option<&Function>);

    #[wasm_bindgen(method, js_name = "onErrorOccurred")]
    pub fn on_error_occurred(this: &WebRequest, filter: WebRequestFilter, listener: Option<&Function>);

    #[wasm_bindgen(method, js_name = "onHeadersReceived")]
    pub fn on_headers_received(this: &WebRequest, filter: WebRequestFilter, listener: Option<&Function>);

    #[wasm_bindgen(method, js_name = "onResponseStarted")]
    pub fn on_response_started(this: &WebRequest, filter: WebRequestFilter, listener: Option<&Function>);

    #[wasm_bindgen(method, js_name = "onSendHeaders")]
    pub fn on_send_headers(this: &WebRequest, filter: WebRequestFilter, listener: Option<&Function>);
}
