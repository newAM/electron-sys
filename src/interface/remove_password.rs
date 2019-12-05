use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct RemovePassword {
    kind: JsString,
    origin: Option<JsString>,
    password: Option<JsString>,
    realm: Option<JsString>,
    scheme: Option<JsString>,
    username: Option<JsString>,
}

#[wasm_bindgen]
impl RemovePassword {
    #[wasm_bindgen(constructor)]
    pub fn new(
        kind: JsString,
        origin: Option<JsString>,
        password: Option<JsString>,
        realm: Option<JsString>,
        scheme: Option<JsString>,
        username: Option<JsString>,
    ) -> RemovePassword {
        RemovePassword {
            kind,
            origin,
            password,
            realm,
            scheme,
            username,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> JsString {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: JsString) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> Option<JsString> {
        self.origin.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_origin(&mut self, value: Option<JsString>) {
        self.origin = value;
    }

    #[wasm_bindgen(getter)]
    pub fn password(&self) -> Option<JsString> {
        self.password.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_password(&mut self, value: Option<JsString>) {
        self.password = value;
    }

    #[wasm_bindgen(getter)]
    pub fn realm(&self) -> Option<JsString> {
        self.realm.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_realm(&mut self, value: Option<JsString>) {
        self.realm = value;
    }

    #[wasm_bindgen(getter)]
    pub fn scheme(&self) -> Option<JsString> {
        self.scheme.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_scheme(&mut self, value: Option<JsString>) {
        self.scheme = value;
    }

    #[wasm_bindgen(getter)]
    pub fn username(&self) -> Option<JsString> {
        self.username.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_username(&mut self, value: Option<JsString>) {
        self.username = value;
    }
}
