use js_sys::{JsString, Number};
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct AddRepresentationOptions {
    buffer: Option<Buffer>,
    data_url: Option<JsString>,
    height: Option<Number>,
    scale_factor: Option<Number>,
    width: Option<Number>,
}

#[wasm_bindgen]
impl AddRepresentationOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        buffer: Option<Buffer>,
        data_url: Option<JsString>,
        height: Option<Number>,
        scale_factor: Option<Number>,
        width: Option<Number>,
    ) -> AddRepresentationOptions {
        AddRepresentationOptions {
            buffer,
            data_url,
            height,
            scale_factor,
            width,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn buffer(&self) -> Option<Buffer> {
        self.buffer.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_buffer(&mut self, value: Option<Buffer>) {
        self.buffer = value;
    }

    #[wasm_bindgen(getter, js_name = "dataURL")]
    pub fn data_url(&self) -> Option<JsString> {
        self.data_url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data_url(&mut self, value: Option<JsString>) {
        self.data_url = value;
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> Option<Number> {
        self.height.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, value: Option<Number>) {
        self.height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn scale_factor(&self) -> Option<Number> {
        self.scale_factor.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_scale_factor(&mut self, value: Option<Number>) {
        self.scale_factor = value;
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> Option<Number> {
        self.width.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, value: Option<Number>) {
        self.width = value;
    }
}
