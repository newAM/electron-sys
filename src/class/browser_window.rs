use crate::{
    class::{web_contents::WebContents, BrowserView},
    interface::browser_window_options::BrowserWindowOptions,
};
use js_sys::{Array, JsString, Map};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/browser-window
    pub type BrowserWindow;

    // Static Methods

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "addDevToolsExtension")]
    pub fn add_dev_tools_extension(path: &JsString);

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "addExtension")]
    pub fn add_extension(path: &JsString);

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "fromBrowserView")]
    pub fn from_browser_view(browser_view: &BrowserView) -> Option<BrowserWindow>;

    // FIXME: should return Option<BrowserWindow>?
    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "from_id")]
    pub fn from_id(id: usize) -> BrowserWindow;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "from_web_contents")]
    pub fn from_web_contents(web_contents: &WebContents) -> Option<BrowserWindow>;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "getAllWindows")]
    pub fn get_all_windows() -> Array;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "getDevToolsExtension")]
    pub fn get_dev_tools_extension() -> Map;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "getFocusedWindow")]
    pub fn get_focused_window() -> Option<BrowserWindow>;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "removeDevToolsExtension")]
    pub fn remove_dev_tools_extension(name: &JsString);

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "removeExtension")]
    pub fn remove_extension(name: &JsString);

    // Instance Methods

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<BrowserWindowOptions>) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &BrowserWindow, path: &JsString);

    #[wasm_bindgen(method, js_name = "setTitle")]
    pub fn set_title(this: &BrowserWindow, title: &JsString);

    // Instance Properties

    #[wasm_bindgen(method, getter, js_name = "accessibleTitle")]
    pub fn accessible_title(this: &BrowserWindow) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "accessibleTitle")]
    pub fn set_accessible_title(this: &BrowserWindow, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "autoHideMenuBar")]
    pub fn auto_hide_menu_bar(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter, js_name = "autoHideMenuBar")]
    pub fn set_auto_hide_menu_bar(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn closable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter, js_name = "closable")]
    pub fn set_closable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter, js_name = "excludedFromShownWindowsMenu")]
    pub fn excluded_from_shown_windows_menu(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter, js_name = "excludedFromShownWindowsMenu")]
    pub fn set_excluded_from_shown_windows_menu(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter, js_name = "fullScreenable")]
    pub fn full_screenable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter, js_name = "fullScreenable")]
    pub fn set_full_screenable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &BrowserWindow) -> usize;

    #[wasm_bindgen(method, getter)]
    pub fn maximizable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_maximizable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn minimizable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_minimizable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn movable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_movable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn resizable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_resizable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter, js_name = "webContents")]
    pub fn web_contents(this: &BrowserWindow) -> WebContents;
}
