use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct RelaunchOptions {
    args: JsString,
    exec_path: Option<JsString>,
}

#[wasm_bindgen]
impl RelaunchOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(args: JsString, exec_path: Option<JsString>) -> RelaunchOptions {
        RelaunchOptions { args, exec_path }
    }

    #[wasm_bindgen(getter)]
    pub fn args(&self) -> JsString {
        self.args.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_args(&mut self, args: JsString) {
        self.args = args;
    }

    #[wasm_bindgen(getter)]
    pub fn exec_path(&self) -> JsString {
        self.args.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_exec_path(&mut self, exec_path: Option<JsString>) {
        self.exec_path = exec_path;
    }
}
