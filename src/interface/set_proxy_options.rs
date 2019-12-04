use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetProxyOptions {
    pac_script: Option<JsString>,
    proxy_rules: Option<JsString>,
    proxy_bypass_rules: Option<JsString>,
}

#[wasm_bindgen]
impl SetProxyOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        pac_script: Option<JsString>,
        proxy_rules: Option<JsString>,
        proxy_bypass_rules: Option<JsString>,
    ) -> SetProxyOptions {
        SetProxyOptions {
            pac_script,
            proxy_rules,
            proxy_bypass_rules,
        }
    }

    #[wasm_bindgen(getter, js_name = "pacScript")]
    pub fn pac_script(&self) -> Option<JsString> {
        self.pac_script.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_pac_script(&mut self, value: Option<JsString>) {
        self.pac_script = value;
    }

    #[wasm_bindgen(getter, js_name = "proxyRules")]
    pub fn proxy_rules(&self) -> Option<JsString> {
        self.proxy_rules.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_proxy_rules(&mut self, value: Option<JsString>) {
        self.proxy_rules = value;
    }

    #[wasm_bindgen(getter, js_name = "proxyBypassRules")]
    pub fn proxy_bypass_rules(&self) -> Option<JsString> {
        self.proxy_bypass_rules.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_proxy_bypass_rules(&mut self, value: Option<JsString>) {
        self.proxy_bypass_rules = value;
    }
}
