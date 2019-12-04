use crate::class::Session;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClientRequestOptions {
    host: Option<JsString>,
    hostname: Option<JsString>,
    method: Option<JsString>,
    partition: Option<JsString>,
    path: Option<JsString>,
    port: Option<usize>,
    protocol: Option<JsString>,
    redirect: Option<JsString>,
    session: Option<Session>,
    url: Option<JsString>,
}

#[wasm_bindgen]
impl ClientRequestOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        host: Option<JsString>,
        hostname: Option<JsString>,
        method: Option<JsString>,
        partition: Option<JsString>,
        path: Option<JsString>,
        port: Option<usize>,
        protocol: Option<JsString>,
        redirect: Option<JsString>,
        session: Option<Session>,
        url: Option<JsString>,
    ) -> ClientRequestOptions {
        ClientRequestOptions {
            host,
            hostname,
            method,
            partition,
            path,
            port,
            protocol,
            redirect,
            session,
            url,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn host(&self) -> Option<JsString> {
        self.host.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_host(&mut self, value: Option<JsString>) {
        self.host = value;
    }

    #[wasm_bindgen(getter)]
    pub fn hostname(&self) -> Option<JsString> {
        self.hostname.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_hostname(&mut self, value: Option<JsString>) {
        self.hostname = value;
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
    pub fn partition(&self) -> Option<JsString> {
        self.partition.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_partition(&mut self, value: Option<JsString>) {
        self.partition = value;
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
    pub fn port(&self) -> Option<usize> {
        self.port
    }

    #[wasm_bindgen(setter)]
    pub fn set_port(&mut self, value: Option<usize>) {
        self.port = value;
    }

    #[wasm_bindgen(getter)]
    pub fn protocol(&self) -> Option<JsString> {
        self.protocol.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_protocol(&mut self, value: Option<JsString>) {
        self.protocol = value;
    }

    #[wasm_bindgen(getter)]
    pub fn redirect(&self) -> Option<JsString> {
        self.redirect.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_redirect(&mut self, value: Option<JsString>) {
        self.redirect = value;
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
    pub fn url(&self) -> Option<JsString> {
        self.url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_url(&mut self, value: Option<JsString>) {
        self.url = value;
    }
}
