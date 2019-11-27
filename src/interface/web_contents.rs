use crate::interface::OpenDevToolsOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type WebContents;

    #[wasm_bindgen(method, js_name = "openDevTools")]
    pub fn open_dev_tools(this: &WebContents, options: Option<OpenDevToolsOptions>);
}
