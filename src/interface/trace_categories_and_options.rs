use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceCategoriesAndOptions {
    category_filter: JsString,
    trace_options: JsString,
}

#[wasm_bindgen]
impl TraceCategoriesAndOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(category_filter: JsString, trace_options: JsString) -> TraceCategoriesAndOptions {
        TraceCategoriesAndOptions {
            category_filter,
            trace_options,
        }
    }

    #[wasm_bindgen(getter, js_name = "categoryFilter")]
    pub fn category_filter(&self) -> JsString {
        self.category_filter.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_category_filter(&mut self, value: JsString) {
        self.category_filter = value;
    }

    #[wasm_bindgen(getter, js_name = "traceOptions")]
    pub fn trace_options(&self) -> JsString {
        self.trace_options.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_trace_options(&mut self, value: JsString) {
        self.trace_options = value;
    }
}
