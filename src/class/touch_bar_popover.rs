use crate::{class::NativeImage, interface::TouchBarPopoverOptions};
use js_sys::JsString;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-popover
    pub type TouchBarPopover;

    // Constructor

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarPopoverOptions) -> TouchBarPopover;

    // Instance Properties

    #[wasm_bindgen(method, getter)]
    pub fn icon(this: &TouchBarPopover) -> NativeImage;

    #[wasm_bindgen(method, setter)]
    pub fn set_icon(this: &TouchBarPopover, value: NativeImage);

    #[wasm_bindgen(method, getter)]
    pub fn label(this: &TouchBarPopover) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_label(this: &TouchBarPopover, value: JsString);
}
