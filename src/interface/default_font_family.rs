use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DefaultFontFamily {
    cursive: Option<JsString>,
    fantasy: Option<JsString>,
    monospace: Option<JsString>,
    sans_serif: Option<JsString>,
    serif: Option<JsString>,
    standard: Option<JsString>,
}

#[wasm_bindgen]
impl DefaultFontFamily {
    #[wasm_bindgen(constructor)]
    pub fn new(
        cursive: Option<JsString>,
        fantasy: Option<JsString>,
        monospace: Option<JsString>,
        sans_serif: Option<JsString>,
        serif: Option<JsString>,
        standard: Option<JsString>,
    ) -> DefaultFontFamily {
        DefaultFontFamily {
            cursive,
            fantasy,
            monospace,
            sans_serif,
            serif,
            standard,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn cursive(&self) -> Option<JsString> {
        self.cursive.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_cursive(&mut self, value: Option<JsString>) {
        self.cursive = value;
    }

    #[wasm_bindgen(getter)]
    pub fn fantasy(&self) -> Option<JsString> {
        self.fantasy.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_fantasy(&mut self, value: Option<JsString>) {
        self.fantasy = value;
    }

    #[wasm_bindgen(getter)]
    pub fn monospace(&self) -> Option<JsString> {
        self.monospace.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_monospace(&mut self, value: Option<JsString>) {
        self.monospace = value;
    }

    #[wasm_bindgen(getter, js_name = "sansSerif")]
    pub fn sans_serif(&self) -> Option<JsString> {
        self.sans_serif.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_sans_serif(&mut self, value: Option<JsString>) {
        self.sans_serif = value;
    }

    #[wasm_bindgen(getter)]
    pub fn serif(&self) -> Option<JsString> {
        self.serif.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_serif(&mut self, value: Option<JsString>) {
        self.serif = value;
    }

    #[wasm_bindgen(getter)]
    pub fn standard(&self) -> Option<JsString> {
        self.standard.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_standard(&mut self, value: Option<JsString>) {
        self.standard = value;
    }
}
