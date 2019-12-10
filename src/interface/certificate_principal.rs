use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct CertificatePrincipal {
    common_name: JsString,
    country: JsString,
    locality: JsString,
    organization_units: Box<[JsValue]>,
    organizations: Box<[JsValue]>,
    state: JsString,
}

#[wasm_bindgen]
impl CertificatePrincipal {
    #[wasm_bindgen(constructor)]
    pub fn new(
        common_name: JsString,
        country: JsString,
        locality: JsString,
        organization_units: Box<[JsValue]>,
        organizations: Box<[JsValue]>,
        state: JsString,
    ) -> CertificatePrincipal {
        CertificatePrincipal {
            common_name,
            country,
            locality,
            organization_units,
            organizations,
            state,
        }
    }

    #[wasm_bindgen(getter, js_name = "commonName")]
    pub fn common_name(&self) -> JsString {
        self.common_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_common_name(&mut self, value: JsString) {
        self.common_name = value;
    }

    #[wasm_bindgen(getter)]
    pub fn country(&self) -> JsString {
        self.country.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_country(&mut self, value: JsString) {
        self.country = value;
    }

    #[wasm_bindgen(getter)]
    pub fn locality(&self) -> JsString {
        self.locality.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_locality(&mut self, value: JsString) {
        self.locality = value;
    }

    #[wasm_bindgen(getter, js_name = "organizationUnits")]
    pub fn organization_units(&self) -> Box<[JsValue]> {
        self.organization_units.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_organization_units(&mut self, value: Box<[JsValue]>) {
        self.organization_units = value;
    }

    #[wasm_bindgen(getter)]
    pub fn organizations(&self) -> Box<[JsValue]> {
        self.organizations.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_organizations(&mut self, value: Box<[JsValue]>) {
        self.organizations = value;
    }

    #[wasm_bindgen(getter)]
    pub fn state(&self) -> JsString {
        self.state.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_state(&mut self, value: JsString) {
        self.state = value;
    }
}
