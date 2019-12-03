use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayBalloonOptions {
    content: JsString,
    icon_type: Option<JsString>,
    icon: Option<JsString>,
    large_icon: Option<JsString>,
    no_sound: Option<bool>,
    respect_quiet_time: Option<bool>,
    title: JsString,
}

#[wasm_bindgen]
impl DisplayBalloonOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        content: JsString,
        icon_type: Option<JsString>,
        icon: Option<JsString>,
        large_icon: Option<JsString>,
        no_sound: Option<bool>,
        respect_quiet_time: Option<bool>,
        title: JsString,
    ) -> DisplayBalloonOptions {
        DisplayBalloonOptions {
            content,
            icon_type,
            icon,
            large_icon,
            no_sound,
            respect_quiet_time,
            title,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> JsString {
        self.content.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_content(&mut self, value: JsString) {
        self.content = value;
    }

    #[wasm_bindgen(getter, js_name = "iconType")]
    pub fn icon_type(&self) -> Option<JsString> {
        self.icon_type.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon_type(&mut self, value: Option<JsString>) {
        self.icon_type = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> Option<JsString> {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: Option<JsString>) {
        self.icon = value;
    }

    #[wasm_bindgen(getter, js_name = "largeIcon")]
    pub fn large_icon(&self) -> Option<JsString> {
        self.large_icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_large_icon(&mut self, value: Option<JsString>) {
        self.large_icon = value;
    }

    #[wasm_bindgen(getter, js_name = "noSound")]
    pub fn no_sound(&self) -> Option<bool> {
        self.no_sound
    }

    #[wasm_bindgen(setter)]
    pub fn set_no_sound(&mut self, value: Option<bool>) {
        self.no_sound = value;
    }

    #[wasm_bindgen(getter, js_name = "respectQuietTime")]
    pub fn respect_quiet_time(&self) -> Option<bool> {
        self.respect_quiet_time
    }

    #[wasm_bindgen(setter)]
    pub fn set_respect_quiet_time(&mut self, value: Option<bool>) {
        self.respect_quiet_time = value;
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> JsString {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: JsString) {
        self.title = value;
    }
}
