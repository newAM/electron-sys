use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type NativeImage;

    #[wasm_bindgen(js_name = "nativeImage")]
    pub static native_image: NativeImage;
}
