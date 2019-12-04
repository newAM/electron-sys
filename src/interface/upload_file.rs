use js_sys::{JsString, Number};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct UploadFile {
    file_path: JsString,
    kind: JsString,
    length: usize,
    modification_time: Number,
    offset: usize,
}

#[wasm_bindgen]
impl UploadFile {
    #[wasm_bindgen(constructor)]
    pub fn new(
        file_path: JsString,
        kind: JsString,
        length: usize,
        modification_time: Number,
        offset: usize,
    ) -> UploadFile {
        UploadFile {
            file_path,
            kind,
            length,
            modification_time,
            offset,
        }
    }

    #[wasm_bindgen(getter, js_name = "filePath")]
    pub fn file_path(&self) -> JsString {
        self.file_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_file_path(&mut self, value: JsString) {
        self.file_path = value;
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> JsString {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: JsString) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.length
    }

    #[wasm_bindgen(setter)]
    pub fn set_length(&mut self, value: usize) {
        self.length = value;
    }

    #[wasm_bindgen(getter, js_name = "modificationTime")]
    pub fn modification_time(&self) -> Number {
        self.modification_time.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_modification_time(&mut self, value: Number) {
        self.modification_time = value;
    }

    #[wasm_bindgen(getter)]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[wasm_bindgen(setter)]
    pub fn set_offset(&mut self, value: usize) {
        self.offset = value;
    }
}
