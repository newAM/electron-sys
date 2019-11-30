use crate::class::{Menu, NativeImage};
use js_sys::{JsString, Object, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/dock
    pub type Dock;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn bounce(this: &Dock, kind: &JsString) -> usize;

    #[wasm_bindgen(method, js_name = "cancelBounce")]
    pub fn cancel_bounce(this: &Dock, id: usize);

    #[wasm_bindgen(method, js_name = "downloadFinished")]
    pub fn download_finished(this: &Dock, file_path: &JsString);

    #[wasm_bindgen(method, js_name = "getBadge")]
    pub fn get_badge(this: &Dock) -> JsString;

    #[wasm_bindgen(method, js_name = "getMenu")]
    pub fn get_menu(this: &Dock) -> Option<Menu>;

    #[wasm_bindgen(method)]
    pub fn hide(this: &Dock);

    #[wasm_bindgen(method, js_name = "isVisible")]
    pub fn is_visible(this: &Dock) -> bool;

    #[wasm_bindgen(method, js_name = "setBadge")]
    pub fn set_badge(this: &Dock, text: &JsString) -> bool;

    #[wasm_bindgen(method, js_name = "setIcon")]
    pub fn set_icon(this: &Dock, image: &NativeImage) -> bool;

    #[wasm_bindgen(method, js_name = "setMenu")]
    pub fn set_menu(this: &Dock, menu: &Menu);

    #[must_use]
    #[wasm_bindgen(method)]
    pub fn show(this: &Dock) -> Promise;
}
