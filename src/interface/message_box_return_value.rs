use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type MessageBoxReturnValue;

    #[wasm_bindgen(method, getter, js_name = checkboxChecked)]
    pub fn checkbox_checked(this: &MessageBoxReturnValue) -> bool;

    #[wasm_bindgen(method, setter, js_name = checkboxChecked)]
    pub fn set_checkbox_checked(this: &MessageBoxReturnValue, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn response(this: &MessageBoxReturnValue) -> u32;

    #[wasm_bindgen(method, setter)]
    pub fn set_esponse(this: &MessageBoxReturnValue, value: u32);
}
