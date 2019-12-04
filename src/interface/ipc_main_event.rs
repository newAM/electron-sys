use crate::class::WebContents;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::Event;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Event)]
    #[derive(Clone, Debug, PartialEq)]
    pub type IpcMainEvent;

    #[wasm_bindgen(method, getter, js_name = "frameId")]
    pub fn frame_id(this: &IpcMainEvent) -> u32;

    #[wasm_bindgen(method, setter, js_name = "frameId")]
    pub fn set_frame_id(this: &IpcMainEvent, value: u32);

    #[wasm_bindgen(method, getter)]
    pub fn reply(this: &IpcMainEvent) -> Function;

    #[wasm_bindgen(method, setter)]
    pub fn set_reply(this: &IpcMainEvent, value: Function);

    #[wasm_bindgen(method, getter, js_name = "returnValue")]
    pub fn return_value(this: &IpcMainEvent) -> JsValue;

    #[wasm_bindgen(method, setter, js_name = "returnValue")]
    pub fn set_return_value(this: &IpcMainEvent, value: JsValue);

    #[wasm_bindgen(method, getter)]
    pub fn sender(this: &IpcMainEvent) -> WebContents;

    #[wasm_bindgen(method, setter)]
    pub fn set_sender(this: &IpcMainEvent, value: WebContents);
}
