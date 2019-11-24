use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type DesktopCapturer;

    #[wasm_bindgen(js_name = "desktopCapturer")]
    pub static desktop_capturer: DesktopCapturer;
}
