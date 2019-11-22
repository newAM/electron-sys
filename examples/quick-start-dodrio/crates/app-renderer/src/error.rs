use failure::Fail;
use serde::Serialize;
use std::convert::From;
use wasm_bindgen::prelude::*;

#[derive(Debug, Fail, Serialize)]
pub enum AppError {
    #[fail(display = "object null or undefined")]
    NoneError, // FIXME: not the best
}

impl From<AppError> for JsValue {
    fn from(err: AppError) -> Self {
        JsValue::from(format!("{}", err))
    }
}
