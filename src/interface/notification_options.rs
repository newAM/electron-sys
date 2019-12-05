use crate::class::NativeImage;
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotificationOptions {
    actions: Option<Array>,
    body: JsString,
    close_button_text: Option<JsString>,
    has_reply: Option<bool>,
    icon: Option<NativeImage>,
    reply_placeholder: Option<JsString>,
    silent: Option<bool>,
    sound: Option<JsString>,
    subtitle: Option<JsString>,
    timeout_type: Option<JsString>,
    title: JsString,
    urgency: Option<JsString>,
}

#[wasm_bindgen]
impl NotificationOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        actions: Option<Array>,
        body: JsString,
        close_button_text: Option<JsString>,
        has_reply: Option<bool>,
        icon: Option<NativeImage>,
        reply_placeholder: Option<JsString>,
        silent: Option<bool>,
        sound: Option<JsString>,
        subtitle: Option<JsString>,
        timeout_type: Option<JsString>,
        title: JsString,
        urgency: Option<JsString>,
    ) -> NotificationOptions {
        NotificationOptions {
            actions,
            body,
            close_button_text,
            has_reply,
            icon,
            reply_placeholder,
            silent,
            sound,
            subtitle,
            timeout_type,
            title,
            urgency,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn actions(&self) -> Option<Array> {
        self.actions.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_actions(&mut self, value: Option<Array>) {
        self.actions = value;
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> JsString {
        self.body.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, value: JsString) {
        self.body = value;
    }

    #[wasm_bindgen(getter)]
    pub fn close_button_text(&self) -> Option<JsString> {
        self.close_button_text.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_close_button_text(&mut self, value: Option<JsString>) {
        self.close_button_text = value;
    }

    #[wasm_bindgen(getter)]
    pub fn has_reply(&self) -> Option<bool> {
        self.has_reply
    }

    #[wasm_bindgen(setter)]
    pub fn set_has_reply(&mut self, value: Option<bool>) {
        self.has_reply = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> Option<NativeImage> {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: Option<NativeImage>) {
        self.icon = value;
    }

    #[wasm_bindgen(getter)]
    pub fn reply_placeholder(&self) -> Option<JsString> {
        self.reply_placeholder.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_reply_placeholder(&mut self, value: Option<JsString>) {
        self.reply_placeholder = value;
    }

    #[wasm_bindgen(getter)]
    pub fn silent(&self) -> Option<bool> {
        self.silent
    }

    #[wasm_bindgen(setter)]
    pub fn set_silent(&mut self, value: Option<bool>) {
        self.silent = value;
    }

    #[wasm_bindgen(getter)]
    pub fn sound(&self) -> Option<JsString> {
        self.sound.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_sound(&mut self, value: Option<JsString>) {
        self.sound = value;
    }

    #[wasm_bindgen(getter)]
    pub fn subtitle(&self) -> Option<JsString> {
        self.subtitle.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_subtitle(&mut self, value: Option<JsString>) {
        self.subtitle = value;
    }

    #[wasm_bindgen(getter)]
    pub fn timeout_type(&self) -> Option<JsString> {
        self.timeout_type.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_timeout_type(&mut self, value: Option<JsString>) {
        self.timeout_type = value;
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> JsString {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: JsString) {
        self.title = value;
    }

    #[wasm_bindgen(getter)]
    pub fn urgency(&self) -> Option<JsString> {
        self.urgency.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_urgency(&mut self, value: Option<JsString>) {
        self.urgency = value;
    }
}
