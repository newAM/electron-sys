use crate::{class::Session, interface::ProtocolResponseUploadData};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct RedirectRequest {
    method: Option<JsString>,
    session: Option<Session>,
    upload_data: Option<ProtocolResponseUploadData>,
    url: JsString,
}

#[wasm_bindgen]
impl RedirectRequest {
    #[wasm_bindgen(constructor)]
    pub fn new(
        method: Option<JsString>,
        session: Option<Session>,
        upload_data: Option<ProtocolResponseUploadData>,
        url: JsString,
    ) -> RedirectRequest {
        RedirectRequest {
            method,
            session,
            upload_data,
            url,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn method(&self) -> Option<JsString> {
        self.method.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_method(&mut self, value: Option<JsString>) {
        self.method = value;
    }

    #[wasm_bindgen(getter)]
    pub fn session(&self) -> Option<Session> {
        self.session.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_session(&mut self, value: Option<Session>) {
        self.session = value;
    }

    #[wasm_bindgen(getter)]
    pub fn upload_data(&self) -> Option<ProtocolResponseUploadData> {
        self.upload_data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_upload_data(&mut self, value: Option<ProtocolResponseUploadData>) {
        self.upload_data = value;
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> JsString {
        self.url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_url(&mut self, value: JsString) {
        self.url = value;
    }
}
