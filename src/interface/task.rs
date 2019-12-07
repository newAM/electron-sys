use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    pub type Task;

    #[wasm_bindgen(constructor)]
    pub fn new(
        arguments: JsString,
        description: JsString,
        icon_index: usize,
        icon_path: JsString,
        program: JsString,
        title: JsString,
        working_directory: Option<JsString>,
    ) -> Task;

    #[wasm_bindgen(method, getter)]
    pub fn arguments(this: &Task) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_arguments(this: &Task, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn description(this: &Task) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_description(this: &Task, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "iconIndex")]
    pub fn icon_index(this: &Task) -> usize;

    #[wasm_bindgen(method, setter)]
    pub fn set_icon_index(this: &Task, value: usize);

    #[wasm_bindgen(method, getter, js_name = "iconPath")]
    pub fn icon_path(this: &Task) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_icon_path(this: &Task, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn program(this: &Task) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_program(this: &Task, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn title(this: &Task) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_title(this: &Task, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "workingDirectory")]
    pub fn working_directory(this: &Task) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_working_directory(this: &Task, value: Option<JsString>);
}
