use crate::class::NativeImage;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Data {
    bookmark: Option<JsString>,
    html: Option<JsString>,
    image: Option<NativeImage>,
    rtf: Option<JsString>,
    text: Option<JsString>,
}

#[wasm_bindgen]
impl Data {
    #[wasm_bindgen(constructor)]
    pub fn new(
        bookmark: Option<JsString>,
        html: Option<JsString>,
        image: Option<NativeImage>,
        rtf: Option<JsString>,
        text: Option<JsString>,
    ) -> Data {
        Data {
            bookmark,
            html,
            image,
            rtf,
            text,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn bookmark(&self) -> Option<JsString> {
        self.bookmark.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_bookmark(&mut self, value: Option<JsString>) {
        self.bookmark = value;
    }

    #[wasm_bindgen(getter)]
    pub fn html(&self) -> Option<JsString> {
        self.html.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_html(&mut self, value: Option<JsString>) {
        self.html = value;
    }

    #[wasm_bindgen(getter)]
    pub fn image(&self) -> Option<NativeImage> {
        self.image.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_image(&mut self, value: Option<NativeImage>) {
        self.image = value;
    }

    #[wasm_bindgen(getter)]
    pub fn rtf(&self) -> Option<JsString> {
        self.rtf.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_rtf(&mut self, value: Option<JsString>) {
        self.rtf = value;
    }

    #[wasm_bindgen(getter)]
    pub fn text(&self) -> Option<JsString> {
        self.text.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_text(&mut self, value: Option<JsString>) {
        self.text = value;
    }
}
