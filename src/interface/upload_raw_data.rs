use js_sys::JsString;
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct UploadRawData {
    bytes: Buffer,
    kind: JsString,
}

#[wasm_bindgen]
impl UploadRawData {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Buffer, kind: JsString) -> UploadRawData {
        UploadRawData { bytes, kind }
    }

    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Buffer {
        self.bytes.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_bytes(&mut self, value: Buffer) {
        self.bytes = value;
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> JsString {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: JsString) {
        self.kind = value;
    }
}
