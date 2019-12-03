use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type WebSource;

    #[wasm_bindgen(method, getter)]
    pub fn code(this: &WebSource) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_code(this: &WebSource, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "startLine")]
    pub fn start_line(this: &WebSource) -> Option<usize>;

    #[wasm_bindgen(method, setter, js_name = "startLine")]
    pub fn set_start_line(this: &WebSource, value: Option<usize>);

    #[wasm_bindgen(method, getter)]
    pub fn url(this: &WebSource) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_url(this: &WebSource, value: Option<JsString>);
}
