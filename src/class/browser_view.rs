use crate::{
    class::WebContents,
    interface::{AutoResizeOptions, Rectangle},
};
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/browser-view
    pub type BrowserView;

    #[wasm_bindgen(static_method_of = BrowserView)]
    pub fn from_id(id: usize) -> BrowserView;

    #[wasm_bindgen(static_method_of = BrowserView)]
    pub fn from_web_contents(web_contents: &WebContents) -> Option<BrowserView>;

    #[wasm_bindgen(static_method_of = BrowserView)]
    pub fn get_all_views() -> Array;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &BrowserView);

    #[wasm_bindgen(method, js_name = "getBounds")]
    pub fn get_bounds(this: &BrowserView) -> Rectangle;

    #[wasm_bindgen(method, js_name = "isDestroyed")]
    pub fn is_destroyed(this: &BrowserView) -> bool;

    #[wasm_bindgen(method, js_name = "set_auto_resize")]
    pub fn set_auto_resize(this: &BrowserView, options: AutoResizeOptions);

    #[wasm_bindgen(method, js_name = "set_background_color")]
    pub fn set_background_color(this: &BrowserView, color: &JsString);

    #[wasm_bindgen(method, js_name = "set_bounds")]
    pub fn set_bounds(this: &BrowserView, bounds: &Rectangle);

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &BrowserView) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn web_contents(this: &BrowserView) -> WebContents;
}
