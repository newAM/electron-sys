use js_sys::Promise;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    /// Docs: http://electronjs.org/docs/api/debugger
    pub type Debugger;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn attach(this: &Debugger, protocol_version: Option<&str>);

    #[wasm_bindgen(method)]
    pub fn detach(this: &Debugger);

    #[wasm_bindgen(method, js_name = "isAttached")]
    pub fn is_attached(this: &Debugger);

    #[must_use]
    #[wasm_bindgen(method, js_name = "sendCommand")]
    pub fn send_command(this: &Debugger, method: &str, command_params: &JsValue) -> Promise;
}
