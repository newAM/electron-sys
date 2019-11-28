use js_sys::{JsString, Promise};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/debugger
    pub type Debugger;

    // Instance Methods

    #[wasm_bindgen(method)]
    pub fn attach(this: &Debugger, protocol_version: Option<JsString>);

    #[wasm_bindgen(method)]
    pub fn detach(this: &Debugger);

    #[wasm_bindgen(method, js_name = "isAttached")]
    pub fn is_attached(this: &Debugger);

    #[wasm_bindgen(method, js_name = "sendCommand")]
    pub fn send_command(this: &Debugger, method: &JsString, command_params: &JsValue) -> Promise;
}
