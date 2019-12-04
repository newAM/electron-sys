use js_sys::JsString;
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct MimeTypedBuffer {
    data: Buffer,
    mime_typed: JsString,
}

#[wasm_bindgen]
impl MimeTypedBuffer {
    pub fn new(data: Buffer, mime_typed: JsString) -> MimeTypedBuffer {
        MimeTypedBuffer { data, mime_typed }
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Buffer {
        self.data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, value: Buffer) {
        self.data = value;
    }

    #[wasm_bindgen(getter)]
    pub fn mime_typed(&self) -> JsString {
        self.mime_typed.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_mime_typed(&mut self, value: JsString) {
        self.mime_typed = value;
    }
}
