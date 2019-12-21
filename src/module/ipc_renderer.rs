use js_sys::Promise;
use node_sys::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type IpcRenderer;

    #[wasm_bindgen(js_name = "ipcRenderer")]
    pub static ipc_renderer: IpcRenderer;

    #[must_use]
    #[wasm_bindgen(method, variadic)]
    pub fn invoke(this: &IpcRenderer, channel: &str, args: Box<[JsValue]>) -> Promise;

    #[wasm_bindgen(method, variadic)]
    pub fn send(this: &IpcRenderer, channel: &str, args: Box<[JsValue]>);

    #[wasm_bindgen(method, variadic, js_name = "sendSync")]
    pub fn send_sync(this: &IpcRenderer, channel: &str, args: Box<[JsValue]>) -> JsValue;

    #[wasm_bindgen(method, variadic, js_name = "sendTo")]
    pub fn send_to(this: &IpcRenderer, web_contents_id: u32, channel: &str, args: Box<[JsValue]>);

    #[wasm_bindgen(method, variadic, js_name = "sendToHost")]
    pub fn send_to_host(this: &IpcRenderer, channel: &str, args: Box<[JsValue]>);
}
