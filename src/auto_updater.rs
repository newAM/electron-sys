use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type AutoUpdater;

    #[wasm_bindgen(js_name = "autoUpdater")]
    pub static auto_updater: AutoUpdater;
}
