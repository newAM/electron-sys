#![feature(try_trait)]

use dodrio::{builder::*, bumpalo, Node, Render, RenderContext, Vdom};
use failure::Fail;
use serde_derive::Serialize;
use std::{convert::From, option::NoneError};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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

struct Hello {
    name: String,
}

impl Render for Hello {
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let msg = bumpalo::format!(in cx.bump, "Hello, {}!", self.name);
        let msg = msg.into_bump_str();
        p(&cx).children([text(msg)]).finish()
    }
}

pub fn boot() -> Result<(), AppError> {
    Vdom::new(
        web_sys::window()?.document()?.body()?.as_ref(),
        Hello {
            name: "World".into(),
        },
    )
    .forget();
    Ok(())
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    boot()?;
    Ok(())
}
