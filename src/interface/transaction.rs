use crate::interface::Payment;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type Transaction;

    #[wasm_bindgen(method, getter, js_name = "errorCode")]
    pub fn error_code(this: &Transaction) -> u32;

    #[wasm_bindgen(method, setter, js_name = "errorCode")]
    pub fn set_error_code(this: &Transaction, value: u32);

    #[wasm_bindgen(method, getter, js_name = "errorMessage")]
    pub fn error_message(this: &Transaction) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "errorMessage")]
    pub fn set_error_message(this: &Transaction, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "originalTransactionIdentifier")]
    pub fn original_transaction_identifier(this: &Transaction) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "originalTransactionIdentifier")]
    pub fn set_original_transaction_identifier(this: &Transaction, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn payment(this: &Transaction) -> Payment;

    #[wasm_bindgen(method, setter)]
    pub fn set_payment(this: &Transaction, value: Payment);

    #[wasm_bindgen(method, getter, js_name = "transactionDate")]
    pub fn transaction_date(this: &Transaction) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "transactionDate")]
    pub fn set_transaction_date(this: &Transaction, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "transactionIdentifier")]
    pub fn transaction_identifier(this: &Transaction) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "transactionIdentifier")]
    pub fn set_transaction_identifier(this: &Transaction, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "transactionState")]
    pub fn transaction_state(this: &Transaction) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "transactionState")]
    pub fn set_transaction_state(this: &Transaction, value: JsString);
}
