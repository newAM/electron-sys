use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type ContextBridge;

    #[wasm_bindgen(js_name = "contextBridge")]
    pub static context_bridge: ContextBridge;
}
