use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct SetAboutPanelOptions {
    application_name: Option<JsString>,
    application_version: Option<JsString>,
    copyright: Option<JsString>,
    version: Option<JsString>,   // FIXME: macos
    credits: Option<JsString>,   // FIXME: macos
    authors: JsString,           // FIXME: linux
    website: Option<JsString>,   // FIXME: linux
    icon_path: Option<JsString>, // FIXME: linux
}

#[wasm_bindgen]
impl SetAboutPanelOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        application_name: Option<JsString>,
        application_version: Option<JsString>,
        copyright: Option<JsString>,
        version: Option<JsString>,   // FIXME: macos
        credits: Option<JsString>,   // FIXME: macos
        authors: JsString,           // FIXME: linux
        website: Option<JsString>,   // FIXME: linux
        icon_path: Option<JsString>, // FIXME: linux
    ) -> SetAboutPanelOptions {
        SetAboutPanelOptions {
            application_name,
            application_version,
            copyright,
            version,
            credits,
            authors,
            website,
            icon_path,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn application_name(&self) -> Option<JsString> {
        self.application_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_application_name(&mut self, value: Option<JsString>) {
        self.application_name = value;
    }

    #[wasm_bindgen(getter)]
    pub fn application_version(&self) -> Option<JsString> {
        self.application_version.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_application_version(&mut self, value: Option<JsString>) {
        self.application_version = value;
    }

    #[wasm_bindgen(getter)]
    pub fn copyright(&self) -> Option<JsString> {
        self.copyright.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_copyright(&mut self, value: Option<JsString>) {
        self.copyright = value;
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> Option<JsString> {
        self.version.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_version(&mut self, value: Option<JsString>) {
        self.version = value;
    }

    #[wasm_bindgen(getter)]
    pub fn credits(&self) -> Option<JsString> {
        self.credits.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_credits(&mut self, value: Option<JsString>) {
        self.credits = value;
    }

    #[wasm_bindgen(getter)]
    pub fn authors(&self) -> JsString {
        self.authors.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_authors(&mut self, value: JsString) {
        self.authors = value;
    }

    #[wasm_bindgen(getter)]
    pub fn website(&self) -> Option<JsString> {
        self.website.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_website(&mut self, value: Option<JsString>) {
        self.website = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon_path(&self) -> Option<JsString> {
        self.icon_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon_path(&mut self, value: Option<JsString>) {
        self.icon_path = value;
    }
}
