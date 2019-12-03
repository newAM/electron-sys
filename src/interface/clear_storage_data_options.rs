use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClearStorageDataOptions {
    origin: Option<JsString>,
    quotas: Option<Array>,
    storages: Option<Array>,
}

#[wasm_bindgen]
impl ClearStorageDataOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(origin: Option<JsString>, quotas: Option<Array>, storages: Option<Array>) -> ClearStorageDataOptions {
        ClearStorageDataOptions {
            origin,
            quotas,
            storages,
        }
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
    pub fn quotas(&self) -> Option<Array> {
        self.quotas.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_quotas(&mut self, value: Option<Array>) {
        self.quotas = value;
    }

    #[wasm_bindgen(getter)]
    pub fn storages(&self) -> Option<Array> {
        self.storages.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_storages(&mut self, value: Option<Array>) {
        self.storages = value;
    }
}
