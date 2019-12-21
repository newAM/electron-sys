use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/browser-window-proxy
    pub type BrowserWindowProxy;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn blur(this: &BrowserWindowProxy);

    #[wasm_bindgen(method)]
    pub fn close(this: &BrowserWindowProxy);

    #[wasm_bindgen(method)]
    pub fn eval(this: &BrowserWindowProxy, code: &str);

    #[wasm_bindgen(method)]
    pub fn focus(this: &BrowserWindowProxy);

    #[wasm_bindgen(method, js_name = "postMessage")]
    pub fn post_message(this: &BrowserWindowProxy, message: &JsValue, target_origin: &str);

    #[wasm_bindgen(method)]
    pub fn print(this: &BrowserWindowProxy);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn closed(this: &BrowserWindowProxy) -> bool;
}
