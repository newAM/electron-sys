use crate::interface::WebPreferences;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct BrowserViewOptions {
    web_preferences: Option<WebPreferences>,
}

#[wasm_bindgen]
impl BrowserViewOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(web_preferences: Option<WebPreferences>) -> BrowserViewOptions {
        BrowserViewOptions { web_preferences }
    }

    #[wasm_bindgen(getter, js_name = "webPreferences")]
    pub fn web_preferences(&self) -> Option<WebPreferences> {
        self.web_preferences.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_web_preferences(&mut self, value: Option<WebPreferences>) {
        self.web_preferences = value;
    }
}
