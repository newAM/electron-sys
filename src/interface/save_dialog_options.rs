use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SaveDialogOptions {
    button_label: Option<JsString>,
    default_path: Option<JsString>,
    filters: Option<Array>,
    message: Option<JsString>,
    name_field_label: Option<JsString>,
    properties: Option<Array>,
    security_scoped_bookmarks: Option<bool>,
    show_tag_field: Option<bool>,
    title: Option<JsString>,
}

#[wasm_bindgen]
impl SaveDialogOptions {
    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        button_label: Option<JsString>,
        default_path: Option<JsString>,
        filters: Option<Array>,
        message: Option<JsString>,
        name_field_label: Option<JsString>,
        properties: Option<Array>,
        security_scoped_bookmarks: Option<bool>,
        show_tag_field: Option<bool>,
        title: Option<JsString>,
    ) -> SaveDialogOptions {
        SaveDialogOptions {
            button_label,
            default_path,
            filters,
            message,
            name_field_label,
            properties,
            security_scoped_bookmarks,
            show_tag_field,
            title,
        }
    }

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<JsString> {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: Option<JsString>) {
        self.title = value;
    }

    #[wasm_bindgen(getter, js_name = "defaultPath")]
    pub fn default_path(&self) -> Option<JsString> {
        self.default_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_default_path(&mut self, value: Option<JsString>) {
        self.default_path = value;
    }

    #[wasm_bindgen(getter, js_name = "buttonLabel")]
    pub fn button_label(&self) -> Option<JsString> {
        self.button_label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_button_label(&mut self, value: Option<JsString>) {
        self.button_label = value;
    }

    #[wasm_bindgen(getter)]
    pub fn filters(&self) -> Option<Array> {
        self.filters.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_filters(&mut self, value: Option<Array>) {
        self.filters = value;
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> Option<JsString> {
        self.message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, value: Option<JsString>) {
        self.message = value;
    }

    #[wasm_bindgen(getter, js_name = "nameFieldLabel")]
    pub fn name_field_label(&self) -> Option<JsString> {
        self.name_field_label.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name_field_label(&mut self, value: Option<JsString>) {
        self.name_field_label = value;
    }

    #[wasm_bindgen(getter, js_name = "showTagField")]
    pub fn show_tag_field(&self) -> Option<bool> {
        self.show_tag_field
    }

    #[wasm_bindgen(setter)]
    pub fn set_show_tag_field(&mut self, value: Option<bool>) {
        self.show_tag_field = value;
    }

    #[wasm_bindgen(getter)]
    pub fn properties(&self) -> Option<Array> {
        self.properties.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_properties(&mut self, value: Option<Array>) {
        self.properties = value;
    }

    #[wasm_bindgen(getter, js_name = "securityScopedBookmarks")]
    pub fn security_scoped_bookmarks(&self) -> Option<bool> {
        self.security_scoped_bookmarks
    }

    #[wasm_bindgen(setter)]
    pub fn set_security_scoped_bookmarks(&mut self, value: Option<bool>) {
        self.security_scoped_bookmarks = value;
    }
}
