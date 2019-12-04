use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct UploadBlob {
    blob_uuid: Option<JsString>,
    kind: JsString,
}

#[wasm_bindgen]
impl UploadBlob {
    #[wasm_bindgen(constructor)]
    pub fn new(blob_uuid: Option<JsString>, kind: JsString) -> UploadBlob {
        UploadBlob { blob_uuid, kind }
    }

    #[wasm_bindgen(getter, js_name = "blobUUID")]
    pub fn blob_uuid(&self) -> Option<JsString> {
        self.blob_uuid.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_blob_uuid(&mut self, value: Option<JsString>) {
        self.blob_uuid = value;
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
