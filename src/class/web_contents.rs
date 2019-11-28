use crate::interface::OpenDevToolsOptions;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    /// Docs: http://electronjs.org/docs/api/web-contents
    pub type WebContents;

    #[wasm_bindgen(method, js_name = "openDevTools")]
    pub fn open_dev_tools(this: &WebContents, options: Option<OpenDevToolsOptions>);
}
