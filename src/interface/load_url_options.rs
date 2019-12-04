use crate::interface::Referrer;
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoadUrlOptions {
    base_url_for_data_url: Option<JsString>,
    extra_headers: Option<JsString>,
    http_referrer: Referrer,
    post_data: Array,
    user_agent: Option<JsString>,
}

#[wasm_bindgen]
impl LoadUrlOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        base_url_for_data_url: Option<JsString>,
        extra_headers: Option<JsString>,
        http_referrer: Referrer,
        post_data: Array,
        user_agent: Option<JsString>,
    ) -> LoadUrlOptions {
        LoadUrlOptions {
            base_url_for_data_url,
            extra_headers,
            http_referrer,
            post_data,
            user_agent,
        }
    }

    #[wasm_bindgen(getter, js_name = "baseURLForDataURL")]
    pub fn base_url_for_data_url(&self) -> Option<JsString> {
        self.base_url_for_data_url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_base_url_for_data_url(&mut self, value: Option<JsString>) {
        self.base_url_for_data_url = value;
    }

    #[wasm_bindgen(getter, js_name = "extraHeader")]
    pub fn extra_headers(&self) -> Option<JsString> {
        self.extra_headers.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_extra_headers(&mut self, value: Option<JsString>) {
        self.extra_headers = value;
    }

    #[wasm_bindgen(getter, js_name = "httpReferrer")]
    pub fn http_referrer(&self) -> Referrer {
        self.http_referrer.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_http_referrer(&mut self, value: Referrer) {
        self.http_referrer = value;
    }

    #[wasm_bindgen(getter, js_name = "postData")]
    pub fn post_data(&self) -> Array {
        self.post_data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_post_data(&mut self, value: Array) {
        self.post_data = value;
    }

    #[wasm_bindgen(getter, js_name = "userAgent")]
    pub fn user_agent(&self) -> Option<JsString> {
        self.user_agent.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_user_agent(&mut self, value: Option<JsString>) {
        self.user_agent = value;
    }
}
