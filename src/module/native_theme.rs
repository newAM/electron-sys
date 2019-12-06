use js_sys::JsString;
use node_sys::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type NativeTheme;

    #[wasm_bindgen(js_name = "nativeTheme")]
    pub static native_theme: NativeTheme;

    // FIXME: event overloads

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "shouldUseDarkColors")]
    pub fn should_use_dark_colors(this: &NativeTheme) -> bool; // readonly

    #[wasm_bindgen(method, getter, js_name = "shouldUseHighContrastColors")]
    pub fn should_use_high_contrast_colors(this: &NativeTheme) -> bool; // readonly

    #[wasm_bindgen(method, getter, js_name = "shouldUseInvertedColorScheme")]
    pub fn should_use_inverted_color_scheme(this: &NativeTheme) -> bool; // readonly

    #[wasm_bindgen(method, getter, js_name = "themeSource")]
    pub fn theme_source(this: &NativeTheme) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "themeSource")]
    pub fn set_theme_source(this: &NativeTheme, value: JsString);
}
