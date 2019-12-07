use js_sys::{Function, JsString};
use node_sys::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type IpcMain;

    #[wasm_bindgen(js_name = "ipcMain")]
    pub static ipc_main: IpcMain;

    #[wasm_bindgen(method)]
    pub fn handle(this: &IpcMain, channel: &JsString, listener: &Function);

    #[wasm_bindgen(method, js_name = "handleOnce")]
    pub fn handle_once(this: &IpcMain, channel: &JsString, listener: &Function);

    #[wasm_bindgen(method, js_name = "removeHandler")]
    pub fn remove_handler(this: &IpcMain, channel: &JsString);
}
