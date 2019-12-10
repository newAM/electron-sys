use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type Product;

    #[wasm_bindgen(method, getter, js_name = "contentLengths")]
    pub fn content_lengths(this: &Product) -> Box<[JsValue]>;

    #[wasm_bindgen(method, getter, js_name = "contentVersion")]
    pub fn content_version(this: &Product) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "formattedPrice")]
    pub fn formatted_price(this: &Product) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "isDownloadable")]
    pub fn is_downloadable(this: &Product) -> bool;

    #[wasm_bindgen(method, getter, js_name = "localizedDescription")]
    pub fn localized_description(this: &Product) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "localizedTitle")]
    pub fn localized_title(this: &Product) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn price(this: &Product) -> f32;

    #[wasm_bindgen(method, getter, js_name = "productIdentifier")]
    pub fn product_identifier(this: &Product, value: JsString);

    #[wasm_bindgen(method, setter, js_name = "contentLengths")]
    pub fn set_content_lengths(this: &Product, value: Box<[JsValue]>);

    #[wasm_bindgen(method, setter, js_name = "contentVersion")]
    pub fn set_content_version(this: &Product, value: JsString);

    #[wasm_bindgen(method, setter, js_name = "formattedPrice")]
    pub fn set_formatted_price(this: &Product, value: JsString);

    #[wasm_bindgen(method, setter, js_name = "isDownloadable")]
    pub fn set_is_downloadable(this: &Product, value: bool);

    #[wasm_bindgen(method, setter, js_name = "localizedDescription")]
    pub fn set_localized_description(this: &Product, value: JsString);

    #[wasm_bindgen(method, setter, js_name = "localizedTitle")]
    pub fn set_localized_title(this: &Product, value: JsString);

    #[wasm_bindgen(method, setter)]
    pub fn set_price(this: &Product, value: f32);

    #[wasm_bindgen(method, setter, js_name = "productIdentifier")]
    pub fn set_product_identifier(this: &Product, value: JsString);
}
