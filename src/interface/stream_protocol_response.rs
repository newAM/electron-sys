use js_sys::Object;
use node_sys::ReadableStream;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct StreamProtocolResponse {
    data: Option<ReadableStream>,
    headers: Option<Object>,
    status_code: Option<usize>,
}

#[wasm_bindgen]
impl StreamProtocolResponse {
    #[wasm_bindgen(constructor)]
    pub fn new(
        data: Option<ReadableStream>,
        headers: Option<Object>,
        status_code: Option<usize>,
    ) -> StreamProtocolResponse {
        StreamProtocolResponse {
            data,
            headers,
            status_code,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Option<ReadableStream> {
        self.data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, value: Option<ReadableStream>) {
        self.data = value;
    }

    #[wasm_bindgen(getter)]
    pub fn headers(&self) -> Option<Object> {
        self.headers.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_headers(&mut self, value: Option<Object>) {
        self.headers = value;
    }

    #[wasm_bindgen(getter, js_name = "statusCode")]
    pub fn status_code(&self) -> Option<usize> {
        self.status_code
    }

    #[wasm_bindgen(setter)]
    pub fn set_status_code(&mut self, value: Option<usize>) {
        self.status_code = value;
    }
}
