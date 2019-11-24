use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type CrashReporter;

    #[wasm_bindgen(js_name = "crashReporter")]
    pub static crash_reporter: CrashReporter;
}
