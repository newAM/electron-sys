use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type WebContents;

    #[wasm_bindgen(js_name = "webContents")]
    pub static web_contents: WebContents;
}
