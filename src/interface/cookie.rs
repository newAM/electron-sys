use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type Cookie;

    #[wasm_bindgen(method, getter)]
    pub fn domain(this: &Cookie) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_domain(this: &Cookie, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "expirationDate")]
    pub fn expiration_date(this: &Cookie) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "expirationDate")]
    pub fn set_expiration_date(this: &Cookie, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "hostOnly")]
    pub fn host_only(this: &Cookie) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "hostOnly")]
    pub fn set_host_only(this: &Cookie, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "httpOnly")]
    pub fn http_only(this: &Cookie) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "httpOnly")]
    pub fn set_http_only(this: &Cookie, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Cookie) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &Cookie, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn path(this: &Cookie) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_path(this: &Cookie, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn secure(this: &Cookie) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_secure(this: &Cookie, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn session(this: &Cookie) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_session(this: &Cookie, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn value(this: &Cookie) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_value(this: &Cookie, value: JsString);
}
