use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MoveToApplicationsFolderOptions {
    conflict_handler: Function,
}

#[wasm_bindgen]
impl MoveToApplicationsFolderOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(conflict_handler: &Function) -> MoveToApplicationsFolderOptions {
        MoveToApplicationsFolderOptions {
            conflict_handler: conflict_handler.clone(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn conflict_handler(&self) -> Function {
        self.conflict_handler.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_conflict_handler(&mut self, value: Function) {
        self.conflict_handler = value;
    }
}
