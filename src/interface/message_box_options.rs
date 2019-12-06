use crate::class::NativeImage;
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct MessageBoxOptions {
    buttons: Option<Array>,
    cancel_id: Option<u32>,
    checkbox_checked: Option<bool>,
    checkbox_label: Option<JsString>,
    default_id: Option<u32>,
    detail: Option<JsString>,
    icon: Option<NativeImage>,
    kind: Option<JsString>,
    message: JsString,
    no_link: Option<bool>,
    normalize_access_keys: Option<bool>,
    title: Option<JsString>,
}

#[wasm_bindgen]
impl MessageBoxOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        buttons: Option<Array>,
        cancel_id: Option<u32>,
        checkbox_checked: Option<bool>,
        checkbox_label: Option<JsString>,
        default_id: Option<u32>,
        detail: Option<JsString>,
        icon: Option<NativeImage>,
        kind: Option<JsString>,
        message: JsString,
        no_link: Option<bool>,
        normalize_access_keys: Option<bool>,
        title: Option<JsString>,
    ) -> MessageBoxOptions {
        MessageBoxOptions {
            buttons,
            cancel_id,
            checkbox_checked,
            checkbox_label,
            default_id,
            detail,
            icon,
            kind,
            message,
            no_link,
            normalize_access_keys,
            title,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn buttons(&self) -> Option<Array> {
        self.buttons.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_buttons(&mut self, value: Option<Array>) {
        self.buttons = value;
    }

    #[wasm_bindgen(getter, js_name = "cancelId")]
    pub fn cancel_id(&self) -> Option<u32> {
        self.cancel_id
    }

    #[wasm_bindgen(setter)]
    pub fn set_cancel_id(&mut self, value: Option<u32>) {
        self.cancel_id = value;
    }

    #[wasm_bindgen(getter, js_name = "checkboxChecked")]
    pub fn checkbox_checked(&self) -> Option<bool> {
        self.checkbox_checked
    }

    #[wasm_bindgen(setter)]
    pub fn set_checkbox_checked(&mut self, value: Option<bool>) {
        self.checkbox_checked = value;
    }

    #[wasm_bindgen(getter, js_name = "checkboxLabel")]
    pub fn checkbox_label(&self) -> Option<JsString> {
        self.checkbox_label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_checkbox_label(&mut self, value: Option<JsString>) {
        self.checkbox_label = value;
    }

    #[wasm_bindgen(getter, js_name = "defaultId")]
    pub fn default_id(&self) -> Option<u32> {
        self.default_id
    }

    #[wasm_bindgen(setter)]
    pub fn set_default_id(&mut self, value: Option<u32>) {
        self.default_id = value;
    }

    #[wasm_bindgen(getter)]
    pub fn detail(&self) -> Option<JsString> {
        self.detail.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_detail(&mut self, value: Option<JsString>) {
        self.detail = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> Option<NativeImage> {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: Option<NativeImage>) {
        self.icon = value;
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> Option<JsString> {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: Option<JsString>) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> JsString {
        self.message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, value: JsString) {
        self.message = value;
    }

    #[wasm_bindgen(getter, js_name = "noLink")]
    pub fn no_link(&self) -> Option<bool> {
        self.no_link
    }

    #[wasm_bindgen(setter)]
    pub fn set_no_link(&mut self, value: Option<bool>) {
        self.no_link = value;
    }

    #[wasm_bindgen(getter, js_name = "normalizeAccessKeys")]
    pub fn normalize_access_keys(&self) -> Option<bool> {
        self.normalize_access_keys
    }

    #[wasm_bindgen(setter)]
    pub fn set_normalize_access_keys(&mut self, value: Option<bool>) {
        self.normalize_access_keys = value;
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<JsString> {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: Option<JsString>) {
        self.title = value;
    }
}
