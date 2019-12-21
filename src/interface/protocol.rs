use js_sys::{Function, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug)]
    pub type Protocol;

    #[wasm_bindgen(method, js_name = "interceptBufferProtocol")]
    pub fn intercept_buffer_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "interceptFileProtocol")]
    pub fn intercept_file_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "interceptHttpProtocol")]
    pub fn intercept_http_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "interceptStreamProtocol")]
    pub fn intercept_stream_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "interceptStringProtocol")]
    pub fn intercept_string_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[must_use]
    #[wasm_bindgen(method, js_name = "isProtocolHandled")]
    pub fn is_protocol_handled(this: &Protocol, scheme: &str) -> Promise;

    #[wasm_bindgen(method, js_name = "registerBufferProtocol")]
    pub fn register_buffer_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "registerFileProtocol")]
    pub fn register_file_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "registerHttpProtocol")]
    pub fn register_http_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "registerSchemesAsPriviledged")]
    pub fn register_schemes_as_privileged(this: &Protocol, custom_schemes: Box<[JsValue]>);

    #[wasm_bindgen(method, js_name = "registerStreamProtocol")]
    pub fn register_stream_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "registerStringProtocol")]
    pub fn register_string_protocol(this: &Protocol, scheme: &str, handler: &Function, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "uninterceptProtocol")]
    pub fn unintercept_protocol(this: &Protocol, scheme: &str, completion: Option<&Function>);

    #[wasm_bindgen(method, js_name = "unregisterProtocol")]
    pub fn unregister_protocl(this: &Protocol, scheme: &str, completion: Option<&Function>);
}
