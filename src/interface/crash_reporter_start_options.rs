use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CrashReporterStartOptions {
    company_name: JsString,
    crashes_directory: Option<JsString>,
    extra: Option<Object>,
    ignore_system_crash_handler: Option<bool>,
    product_name: Option<JsString>,
    submit_url: JsString,
    upload_to_server: Option<bool>,
}

#[wasm_bindgen]
impl CrashReporterStartOptions {
    #[wasm_bindgen]
    pub fn new(
        company_name: JsString,
        crashes_directory: Option<JsString>,
        extra: Option<Object>,
        ignore_system_crash_handler: Option<bool>,
        product_name: Option<JsString>,
        submit_url: JsString,
        upload_to_server: Option<bool>,
    ) -> CrashReporterStartOptions {
        CrashReporterStartOptions {
            company_name,
            crashes_directory,
            extra,
            ignore_system_crash_handler,
            product_name,
            submit_url,
            upload_to_server,
        }
    }

    #[wasm_bindgen(getter, js_name = "companyName")]
    pub fn company_name(&self) -> JsString {
        self.company_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_company_name(&mut self, value: JsString) {
        self.company_name = value;
    }

    #[wasm_bindgen(getter, js_name = "crashesDirectory")]
    pub fn crashes_directory(&self) -> Option<JsString> {
        self.crashes_directory.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_crashes_directory(&mut self, value: Option<JsString>) {
        self.crashes_directory = value;
    }

    #[wasm_bindgen(getter)]
    pub fn extra(&self) -> Option<Object> {
        self.extra.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_extra(&mut self, value: Option<Object>) {
        self.extra = value;
    }

    #[wasm_bindgen(getter, js_name = "ignoreSystemCrashHandler")]
    pub fn ignore_system_crash_handler(&self) -> Option<bool> {
        self.ignore_system_crash_handler
    }

    #[wasm_bindgen(setter)]
    pub fn set_ignore_system_crash_handler(&mut self, value: Option<bool>) {
        self.ignore_system_crash_handler = value;
    }

    #[wasm_bindgen(getter, js_name = "productName")]
    pub fn product_name(&self) -> Option<JsString> {
        self.product_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_product_name(&mut self, value: Option<JsString>) {
        self.product_name = value;
    }

    #[wasm_bindgen(getter, js_name = "submitUrl")]
    pub fn submit_url(&self) -> JsString {
        self.submit_url.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_submit_url(&mut self, value: JsString) {
        self.submit_url = value;
    }

    #[wasm_bindgen(getter, js_name = "uploadToServer")]
    pub fn upload_to_server(&self) -> Option<bool> {
        self.upload_to_server
    }

    #[wasm_bindgen(setter)]
    pub fn set_upload_to_server(&mut self, value: Option<bool>) {
        self.upload_to_server = value;
    }
}
