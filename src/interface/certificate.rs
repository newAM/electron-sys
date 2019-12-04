use crate::interface::CertificatePrincipal;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type Certificate;

    #[wasm_bindgen(constructor)]
    pub fn new(
        data: JsString,
        fingerprint: JsString,
        issuer: CertificatePrincipal,
        issuer_cert: Certificate,
        issuer_name: JsString,
        serial_number: JsString,
        subject: CertificatePrincipal,
        subject_name: JsString,
        valid_expiry: u32,
        valid_start: u32,
    ) -> Certificate;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &Certificate) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_data(this: &Certificate, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn fingerprint(this: &Certificate) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_fingerprint(this: &Certificate, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn issuer(this: &Certificate) -> CertificatePrincipal;

    #[wasm_bindgen(method, setter)]
    pub fn set_issuer(this: &Certificate, value: CertificatePrincipal);

    #[wasm_bindgen(method, getter, js_name = "issuerCert")]
    pub fn issuer_cert(this: &Certificate) -> Certificate;

    #[wasm_bindgen(method, setter, js_name = "issuerCert")]
    pub fn set_issuer_cert(this: &Certificate, value: Certificate);

    #[wasm_bindgen(method, getter, js_name = "issuerName")]
    pub fn issuer_name(this: &Certificate) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "issuerName")]
    pub fn set_issuer_name(this: &Certificate, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "serialNumber")]
    pub fn serial_number(this: &Certificate) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "serialNumber")]
    pub fn set_serial_number(this: &Certificate, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn subject(this: &Certificate) -> CertificatePrincipal;

    #[wasm_bindgen(method, setter)]
    pub fn set_subject(this: &Certificate, value: CertificatePrincipal);

    #[wasm_bindgen(method, getter, js_name = "subjectName")]
    pub fn subject_name(this: &Certificate) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "subjectName")]
    pub fn set_subject_name(this: &Certificate, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "validExpiry")]
    pub fn valid_expiry(this: &Certificate) -> u32;

    #[wasm_bindgen(method, setter, js_name = "validExpiry")]
    pub fn set_valid_expiry(this: &Certificate, value: u32);

    #[wasm_bindgen(method, getter, js_name = "validStart")]
    pub fn valid_start(this: &Certificate) -> u32;

    #[wasm_bindgen(method, setter, js_name = "validStart")]
    pub fn set_valid_start(this: &Certificate, value: u32);
}
