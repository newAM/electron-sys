use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type NativeTheme;

    #[wasm_bindgen(js_name = "nativeTheme")]
    pub static native_theme: NativeTheme;
}
