use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct Privileges {
    allow_service_workers: Option<bool>,
    bypass_csp: Option<bool>,
    cors_enabled: Option<bool>,
    secure: Option<bool>,
    standard: Option<bool>,
    support_fetch_api: Option<bool>,
}

#[wasm_bindgen]
impl Privileges {
    #[wasm_bindgen(constructor)]
    pub fn new(
        allow_service_workers: Option<bool>,
        bypass_csp: Option<bool>,
        cors_enabled: Option<bool>,
        secure: Option<bool>,
        standard: Option<bool>,
        support_fetch_api: Option<bool>,
    ) -> Privileges {
        Privileges {
            allow_service_workers,
            bypass_csp,
            cors_enabled,
            secure,
            standard,
            support_fetch_api,
        }
    }

    #[wasm_bindgen(getter, js_name = "allowServiceWorkers")]
    pub fn allow_service_workers(self) -> Option<bool> {
        self.allow_service_workers
    }

    #[wasm_bindgen(setter)]
    pub fn set_allow_service_workers(mut self, value: Option<bool>) {
        self.allow_service_workers = value;
    }

    #[wasm_bindgen(getter, js_name = "bypassCSP")]
    pub fn bypass_csp(self) -> Option<bool> {
        self.bypass_csp
    }

    #[wasm_bindgen(setter)]
    pub fn set_bypass_csp(mut self, value: Option<bool>) {
        self.bypass_csp = value;
    }

    #[wasm_bindgen(getter, js_name = "corsEnabled")]
    pub fn cors_enabled(self) -> Option<bool> {
        self.cors_enabled
    }

    #[wasm_bindgen(setter)]
    pub fn set_cors_enabled(mut self, value: Option<bool>) {
        self.cors_enabled = value;
    }

    #[wasm_bindgen(getter)]
    pub fn secure(self) -> Option<bool> {
        self.secure
    }

    #[wasm_bindgen(setter)]
    pub fn set_secure(mut self, value: Option<bool>) {
        self.secure = value;
    }

    #[wasm_bindgen(getter)]
    pub fn standard(self) -> Option<bool> {
        self.standard
    }

    #[wasm_bindgen(setter)]
    pub fn set_standard(mut self, value: Option<bool>) {
        self.standard = value;
    }

    #[wasm_bindgen(getter, js_name = "supportFetchAPI")]
    pub fn support_fetch_api(self) -> Option<bool> {
        self.support_fetch_api
    }

    #[wasm_bindgen(setter)]
    pub fn set_support_fetch_api(mut self, value: Option<bool>) {
        self.support_fetch_api = value;
    }
}
