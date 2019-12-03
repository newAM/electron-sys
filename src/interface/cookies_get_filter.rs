use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CookiesGetFilter {
    domain: Option<JsString>,
    name: Option<JsString>,
    path: Option<JsString>,
    secure: Option<JsString>,
    session: Option<JsString>,
    url: Option<JsString>,
}

#[wasm_bindgen]
impl CookiesGetFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        domain: Option<JsString>,
        name: Option<JsString>,
        path: Option<JsString>,
        secure: Option<JsString>,
        session: Option<JsString>,
        url: Option<JsString>,
    ) -> CookiesGetFilter {
        CookiesGetFilter {
            domain,
            name,
            path,
            secure,
            session,
            url,
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
    pub fn secure(&self) -> Option<JsString> {
        self.secure.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_secure(&mut self, value: Option<JsString>) {
        self.secure = value;
    }

    #[wasm_bindgen(getter)]
    pub fn session(&self) -> Option<JsString> {
        self.session.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_session(&mut self, value: Option<JsString>) {
        self.session = value;
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> Option<JsString> {
        self.url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_url(&mut self, value: Option<JsString>) {
        self.url = value;
    }
}
