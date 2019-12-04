use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CookiesSetDetails {
    domain: Option<JsString>,
    expiration_date: Option<bool>,
    http_only: Option<bool>,
    name: Option<JsString>,
    path: Option<JsString>,
    secure: Option<bool>,
    url: JsString,
    value: Option<JsString>,
}

#[wasm_bindgen]
impl CookiesSetDetails {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        domain: Option<JsString>,
        expiration_date: Option<bool>,
        http_only: Option<bool>,
        name: Option<JsString>,
        path: Option<JsString>,
        secure: Option<bool>,
        url: JsString,
        value: Option<JsString>,
    ) -> CookiesSetDetails {
        CookiesSetDetails {
            domain,
            expiration_date,
            http_only,
            name,
            path,
            secure,
            url,
            value,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn domain(&self) -> Option<JsString> {
        self.domain.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_domain(&mut self, value: Option<JsString>) {
        self.domain = value;
    }

    #[wasm_bindgen(getter, js_name = "expirationDate")]
    pub fn expiration_date(&self) -> Option<bool> {
        self.expiration_date
    }

    #[wasm_bindgen(setter)]
    pub fn set_expiration_date(&mut self, value: Option<bool>) {
        self.expiration_date = value;
    }

    #[wasm_bindgen(getter, js_name = "httpOnly")]
    pub fn http_only(&self) -> Option<bool> {
        self.http_only
    }

    #[wasm_bindgen(setter)]
    pub fn set_http_only(&mut self, value: Option<bool>) {
        self.http_only = value;
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<JsString> {
        self.name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, value: Option<JsString>) {
        self.name = value;
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Option<JsString> {
        self.path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_path(&mut self, value: Option<JsString>) {
        self.path = value;
    }

    #[wasm_bindgen(getter)]
    pub fn secure(&self) -> Option<bool> {
        self.secure
    }

    #[wasm_bindgen(setter)]
    pub fn set_secure(&mut self, value: Option<bool>) {
        self.secure = value;
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> JsString {
        self.url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_url(&mut self, value: JsString) {
        self.url = value;
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> Option<JsString> {
        self.value.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_value(&mut self, value: Option<JsString>) {
        self.value = value;
    }
}
