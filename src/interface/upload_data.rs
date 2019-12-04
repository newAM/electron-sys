use js_sys::JsString;
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct UploadData {
    blob_uuid: Option<JsString>,
    bytes: Buffer,
    file: Option<JsString>,
}

#[wasm_bindgen]
impl UploadData {
    #[wasm_bindgen(constructor)]
    pub fn new(blob_uuid: Option<JsString>, bytes: Buffer, file: Option<JsString>) -> UploadData {
        UploadData { blob_uuid, bytes, file }
    }

    #[wasm_bindgen(getter, js_name = "blobUUID")]
    pub fn blob_uuid(&self) -> Option<JsString> {
        self.blob_uuid.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_blob_uuid(&mut self, value: Option<JsString>) {
        self.blob_uuid = value;
    }

    #[wasm_bindgen(getter)]
    pub fn bytes(&self) -> Buffer {
        self.bytes.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_bytes(&mut self, value: Buffer) {
        self.bytes = value;
    }

    #[wasm_bindgen(getter)]
    pub fn file(&self) -> Option<JsString> {
        self.file.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_file(&mut self, value: Option<JsString>) {
        self.file = value;
    }
}
