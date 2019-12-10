use js_sys::JsString;
use wasm_bindgen::prelude::*;

// NOTE: extends InputEvent
#[wasm_bindgen]
pub struct KeyboardInputEvent {
    key_code: JsString,
    kind: JsString,
    modifiers: Option<Box<[JsValue]>>,
}

#[wasm_bindgen]
impl KeyboardInputEvent {
    #[wasm_bindgen(constructor)]
    pub fn new(key_code: JsString, kind: JsString, modifiers: Option<Box<[JsValue]>>) -> KeyboardInputEvent {
        KeyboardInputEvent {
            key_code,
            kind,
            modifiers,
        }
    }

    #[wasm_bindgen(getter, js_name = "keyCode")]
    pub fn key_code(&self) -> JsString {
        self.key_code.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_key_code(&mut self, value: JsString) {
        self.key_code = value;
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> JsString {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: JsString) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn modifiers(&self) -> Option<Box<[JsValue]>> {
        self.modifiers.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_modifiers(&mut self, value: Option<Box<[JsValue]>>) {
        self.modifiers = value;
    }
}
