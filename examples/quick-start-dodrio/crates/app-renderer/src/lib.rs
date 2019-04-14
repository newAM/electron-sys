#![feature(try_trait)]

mod error;

use dodrio::{builder::*, bumpalo, Node, Render, RenderContext, Vdom};
use wasm_bindgen::prelude::*;

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

pub fn boot() -> Result<(), error::AppError> {
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
