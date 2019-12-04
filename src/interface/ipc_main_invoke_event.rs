use crate::class::WebContents;
use wasm_bindgen::prelude::*;
use web_sys::Event;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Event)]
    #[derive(Clone, Debug, PartialEq)]
    pub type IpcMainInvokeEvent;

    #[wasm_bindgen(method, getter, js_name = "frameId")]
    pub fn frame_id(this: &IpcMainInvokeEvent) -> u32;

    #[wasm_bindgen(method, setter, js_name = "frameId")]
    pub fn set_frame_id(this: &IpcMainInvokeEvent, value: u32);

    #[wasm_bindgen(method, getter)]
    pub fn sender(this: &IpcMainInvokeEvent) -> WebContents;

    #[wasm_bindgen(method, setter)]
    pub fn set_sender(this: &IpcMainInvokeEvent, value: WebContents);
}
