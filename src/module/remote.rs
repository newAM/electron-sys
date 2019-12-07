use crate::class::{BrowserWindow, WebContents};
use js_sys::JsString;
use node_sys::Process;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type Remote;

    pub static remote: Remote;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "getCurrentWebContents")]
    pub fn get_current_web_contents(this: &Remote) -> WebContents;

    #[wasm_bindgen(method, js_name = "getCurrentWindow")]
    pub fn get_current_window(this: &Remote) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "getGlobal")]
    pub fn get_global(this: &Remote, name: &JsString) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn require(this: &Remote, module: &JsString) -> JsValue;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn process(this: &Remote) -> Process;
}
