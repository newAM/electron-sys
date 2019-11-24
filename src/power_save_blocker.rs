use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type PowerSaveBlocker;

    #[wasm_bindgen(js_name = "powerSaveBlocker")]
    pub static power_save_blocker: PowerSaveBlocker;
}
