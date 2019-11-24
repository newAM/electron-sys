use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type SystemPreferences;

    #[wasm_bindgen(js_name = "systemPreferences")]
    pub static system_preferences: SystemPreferences;
}
