use crate::{
    class::{Menu, NativeImage},
    interface::{DisplayBalloonOptions, Point, Rectangle},
};
use js_sys::JsString;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/tray
    pub type Tray;

    // Constructor

    #[wasm_bindgen(constructor)]
    pub fn new(image: &NativeImage) -> Tray;

    // Instance Methods

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Tray);

    #[wasm_bindgen(method, js_name = "displayBalloon")]
    pub fn display_balloon(this: &Tray, options: DisplayBalloonOptions);

    #[wasm_bindgen(method)]
    pub fn focus(this: &Tray);

    #[wasm_bindgen(method, js_name = "getBounds")]
    pub fn get_bounds(this: &Tray) -> Rectangle;

    #[wasm_bindgen(method, js_name = "getIgnoreDoubleClickEvents")]
    pub fn get_ignore_double_click_events(this: &Tray) -> bool;

    #[wasm_bindgen(method, js_name = "getTitle")]
    pub fn get_title(this: &Tray) -> JsString;

    #[wasm_bindgen(method, js_name = "isDestroyed")]
    pub fn is_destroyed(this: &Tray) -> bool;

    #[wasm_bindgen(method, js_name = "popUpContextMenu")]
    pub fn pop_up_context_menu(this: &Tray, menu: Option<Menu>, position: Option<Point>);

    #[wasm_bindgen(method, js_name = "removeBalloon")]
    pub fn remove_balloon(this: &Tray);

    #[wasm_bindgen(method, js_name = "setContextMenu")]
    pub fn set_context_menu(this: &Tray, menu: Option<Menu>);

    #[wasm_bindgen(method, js_name = "setIgnoreDoubleClickEvents")]
    pub fn set_ignore_double_click_events(this: &Tray, ignore: bool);

    #[wasm_bindgen(method, js_name = "setImage")]
    pub fn set_image(this: &Tray, image: NativeImage);

    #[wasm_bindgen(method, js_name = "setPressedImage")]
    pub fn set_pressed_image(this: &Tray, image: NativeImage);

    #[wasm_bindgen(method, js_name = "setTitle")]
    pub fn set_title(this: &Tray, title: &JsString);

    #[wasm_bindgen(method, js_name = "setToolTip")]
    pub fn set_tool_tip(this: &Tray, tool_tip: &JsString);
}
