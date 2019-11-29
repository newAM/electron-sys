use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportCertificateOptions {
    certificate: JsString,
    password: JsString,
}

#[wasm_bindgen]
impl ImportCertificateOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(certificate: JsString, password: JsString) -> ImportCertificateOptions {
        ImportCertificateOptions { certificate, password }
    }

    pub fn certificate(&self) -> JsString {
        self.certificate.clone()
    }

    pub fn set_certificate(&mut self, certificate: JsString) {
        self.certificate = certificate;
    }

    pub fn password(&self) -> JsString {
        self.password.clone()
    }

    pub fn set_password(&mut self, password: JsString) {
        self.password = password;
    }
}
