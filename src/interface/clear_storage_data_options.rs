use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClearStorageDataOptions {
    origin: Option<JsString>,
    quotas: Option<Box<[JsValue]>>,
    storages: Option<Box<[JsValue]>>,
}

#[wasm_bindgen]
impl ClearStorageDataOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        origin: Option<JsString>,
        quotas: Option<Box<[JsValue]>>,
        storages: Option<Box<[JsValue]>>,
    ) -> ClearStorageDataOptions {
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
    pub fn quotas(&self) -> Option<Box<[JsValue]>> {
        self.quotas.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_quotas(&mut self, value: Option<Box<[JsValue]>>) {
        self.quotas = value;
    }

    #[wasm_bindgen(getter)]
    pub fn storages(&self) -> Option<Box<[JsValue]>> {
        self.storages.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_storages(&mut self, value: Option<Box<[JsValue]>>) {
        self.storages = value;
    }
}
