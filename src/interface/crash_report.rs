use js_sys::{Date, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type CrashReport;

    #[wasm_bindgen(method, getter)]
    pub fn date(this: &CrashReport) -> Date;

    #[wasm_bindgen(method, setter)]
    pub fn set_date(this: &CrashReport, value: Date);

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &CrashReport) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_id(this: &CrashReport, value: JsString);
}
