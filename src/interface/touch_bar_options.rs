use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TouchBarOptions {
    escape_item: Option<Object>,
    items: Option<Box<[JsValue]>>,
}

#[wasm_bindgen]
impl TouchBarOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(escape_item: Option<Object>, items: Option<Box<[JsValue]>>) -> TouchBarOptions {
        TouchBarOptions { escape_item, items }
    }

    pub fn new() -> TouchBarOptions {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "escapeItem")]
    pub fn escape_item(&self) -> Option<Object> {
        self.escape_item.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_escape_item(&mut self, value: Option<Object>) {
        self.escape_item = value;
    }

    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Option<Box<[JsValue]>> {
        self.items.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_items(&mut self, value: Option<Box<[JsValue]>>) {
        self.items = value;
    }
}
