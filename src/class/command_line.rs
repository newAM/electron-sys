use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/command-line
    pub type CommandLine;

    // Instance Methods

    #[wasm_bindgen(method, js_name = "appendArgument")]
    pub fn append_argument(this: &CommandLine, value: &JsString);

    #[wasm_bindgen(method, js_name = "appendSwitch")]
    pub fn append_switch(this: &CommandLine, switch: &JsString, value: Option<&JsString>);

    // FIXME: null is empty string from electron; we could return Option though
    #[wasm_bindgen(method, js_name = "getSwitchValue")]
    pub fn get_switch_value(this: &CommandLine, switch: &JsString) -> JsString;

    #[wasm_bindgen(method, js_name = "hasSwitch")]
    pub fn has_switch(this: &CommandLine, switch: &JsString) -> bool;
}
