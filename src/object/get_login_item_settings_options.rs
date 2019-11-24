use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GetLoginItemSettingsOptions {
    path: Option<JsString>, // FIXME: windows
    args: JsString,         // FIXME: windows
}

#[wasm_bindgen]
impl GetLoginItemSettingsOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(path: Option<JsString>, args: JsString) -> GetLoginItemSettingsOptions {
        GetLoginItemSettingsOptions { path, args }
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Option<JsString> {
        self.path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_path(&mut self, path: Option<JsString>) {
        self.path = path;
    }

    #[wasm_bindgen(getter)]
    pub fn args(&self) -> JsString {
        self.args.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_args(&mut self, args: JsString) {
        self.args = args;
    }
}
