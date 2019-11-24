use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type WebFrame;

    #[wasm_bindgen(js_name = "webFrame")]
    pub static web_frame: WebFrame;
}
