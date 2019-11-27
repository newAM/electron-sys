use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OpenExternalOptions {
    activate: Option<bool>,
    working_directory: JsString,
}

#[wasm_bindgen]
impl OpenExternalOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(activate: Option<bool>, working_directory: JsString) -> OpenExternalOptions {
        OpenExternalOptions {
            activate,
            working_directory,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn activate(&self) -> Option<bool> {
        self.activate
    }

    #[wasm_bindgen(setter)]
    pub fn set_activate(&mut self, activate: Option<bool>) {
        self.activate = activate;
    }

    #[wasm_bindgen(getter)]
    pub fn working_directory(&self) -> JsString {
        self.working_directory.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_working_directory(&mut self, working_directory: JsString) {
        self.working_directory = working_directory;
    }
}
