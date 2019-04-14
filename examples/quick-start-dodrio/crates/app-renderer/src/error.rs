use failure::Fail;
use serde_derive::Serialize;
use std::{convert::From, option::NoneError};
use wasm_bindgen::prelude::*;

#[derive(Debug, Fail, Serialize)]
pub enum AppError {
    #[fail(display = "object null or undefined")]
    NoneError, // FIXME: not the best
}

impl From<NoneError> for AppError {
    fn from(_: NoneError) -> Self {
        AppError::NoneError
    }
}

impl From<AppError> for JsValue {
    fn from(err: AppError) -> Self {
        JsValue::from(format!("{}", err))
    }
}
