use crate::{
    class::{BrowserWindow, MenuItem},
    interface::PopupOptions,
};
use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/menu
    pub type Menu;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new() -> Menu;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = Menu, js_name = "buildFromTemplate")]
    pub fn build_from_template(template: Box<[JsValue]>) -> Menu;

    #[wasm_bindgen(static_method_of = Menu, js_name = "getApplicationMenu")]
    pub fn get_application_menu(template: Box<[JsValue]>) -> Option<Menu>;

    #[wasm_bindgen(static_method_of = Menu, js_name = "sendApplicationToFirstResponder")]
    pub fn send_application_to_first_responder(action: &JsString);

    #[wasm_bindgen(static_method_of = Menu, js_name = "setApplicationMenu")]
    pub fn set_application_menu(menu: Option<Menu>);

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn append(this: &Menu, menu_item: &MenuItem);

    #[wasm_bindgen(method, js_name = "closePopup")]
    pub fn close_popup(this: &Menu, browser_window: Option<BrowserWindow>);

    #[wasm_bindgen(method, js_name = "getMenuItemById")]
    pub fn get_menu_item_by_id(this: &Menu, id: &JsString) -> MenuItem;

    #[wasm_bindgen(method)]
    pub fn insert(this: &Menu, pos: usize, menu_item: &MenuItem);

    #[wasm_bindgen(method)]
    pub fn popup(this: &Menu, options: Option<PopupOptions>);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn items(this: &Menu) -> Box<[JsValue]>;

    #[wasm_bindgen(method, setter)]
    pub fn set_items(this: &Menu, value: Box<[JsValue]>);
}
