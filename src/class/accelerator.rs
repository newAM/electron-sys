use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = JsString)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type Accelerator;
}
