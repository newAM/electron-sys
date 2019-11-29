use crate::{
    class::{Accelerator, Menu},
    interface::MenuItemOptions,
};
use js_sys::{Function, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/menu-item
    pub type MenuItem;

    // Constructor

    #[wasm_bindgen(constructor)]
    pub fn new(options: MenuItemOptions) -> MenuItem;

    // Instance Properties

    #[wasm_bindgen(method, getter)]
    pub fn accelerator(this: &MenuItem) -> Option<Accelerator>;

    #[wasm_bindgen(method, setter)]
    pub fn set_accelerator(this: &MenuItem, value: Option<Accelerator>);

    #[wasm_bindgen(method, getter)]
    pub fn checked(this: &MenuItem) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_checked(this: &MenuItem, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn click(this: &MenuItem) -> Function;

    #[wasm_bindgen(method, setter)]
    pub fn set_click(this: &MenuItem, value: Function);

    #[wasm_bindgen(method, getter, js_name = "commandId")]
    pub fn command_id(this: &MenuItem) -> usize;

    #[wasm_bindgen(method, setter, js_name = "commandId")]
    pub fn set_command_id(this: &MenuItem, value: usize);

    #[wasm_bindgen(method, getter)]
    pub fn enabled(this: &MenuItem) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_enabled(this: &MenuItem, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn icon(this: &MenuItem) -> JsValue;

    #[wasm_bindgen(method, setter)]
    pub fn set_icon(this: &MenuItem, value: JsValue);

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &MenuItem) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_id(this: &MenuItem, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn kind(this: &MenuItem) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "type")]
    pub fn set_kind(this: &MenuItem, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn label(this: &MenuItem) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_label(this: &MenuItem, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn menu(this: &MenuItem) -> Menu;

    #[wasm_bindgen(method, setter)]
    pub fn set_menu(this: &MenuItem, value: Menu);

    #[wasm_bindgen(method, getter, js_name = "registerAccelerator")]
    pub fn register_accelerator(this: &MenuItem) -> bool;

    #[wasm_bindgen(method, setter, js_name = "registerAccelerator")]
    pub fn set_register_accelerator(this: &MenuItem, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn role(this: &MenuItem) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn set_role(this: &MenuItem, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn sublabel(this: &MenuItem) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn set_sublabel(this: &MenuItem, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn submenu(this: &MenuItem) -> Option<Menu>;

    #[wasm_bindgen(method, setter)]
    pub fn set_submenu(this: &MenuItem, value: Option<Menu>);

    #[wasm_bindgen(method, getter, js_name = "toolTip")]
    pub fn tool_tip(this: &MenuItem) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "toolTip")]
    pub fn set_tool_tip(this: &MenuItem, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn visible(this: &MenuItem) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_visible(this: &MenuItem, value: bool);
}
