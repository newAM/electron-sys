use crate::interface::Certificate;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct CertificateTrustDialogOptions {
    certificate: Certificate,
    message: JsString,
}

#[wasm_bindgen]
impl CertificateTrustDialogOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(certificate: Certificate, message: JsString) -> CertificateTrustDialogOptions {
        CertificateTrustDialogOptions { certificate, message }
    }

    #[wasm_bindgen(getter)]
    pub fn certificate(&self) -> Certificate {
        self.certificate.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_certificate(&mut self, value: Certificate) {
        self.certificate = value;
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> JsString {
        self.message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_message(&mut self, value: JsString) {
        self.message = value;
    }
}
