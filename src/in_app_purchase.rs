use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type InAppPurchase;

    #[wasm_bindgen(js_name = "inAppPurchase")]
    pub static in_app_purchase: InAppPurchase;
}
