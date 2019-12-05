use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type ProtocolResponseUploadData;

    #[wasm_bindgen(method, getter, js_name = "contentType")]
    pub fn content_type(this: &ProtocolResponseUploadData) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "contentType")]
    pub fn set_content_type(this: &ProtocolResponseUploadData, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &ProtocolResponseUploadData) -> JsValue;

    #[wasm_bindgen(method, setter)]
    pub fn set_data(this: &ProtocolResponseUploadData, value: JsValue);
}
