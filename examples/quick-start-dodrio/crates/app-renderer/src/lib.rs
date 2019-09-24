mod error;

use dodrio::{bumpalo, Node, Render, RenderContext, Vdom};
use wasm_bindgen::prelude::*;

struct Hello {
    name: String,
}

impl Render for Hello {
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        use dodrio::builder::{p, text};
        let msg = bumpalo::format!(in cx.bump, "Hello, {}!", self.name);
        let msg = msg.into_bump_str();
        p(&cx).children([text(msg)]).finish()
    }
}

pub fn boot() -> Result<(), error::AppError> {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.body())
        .and_then(|body| Some(Vdom::new(body.as_ref(), Hello { name: "World".into() }).forget()))
        .ok_or(error::AppError::NoneError)
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    boot()?;
    Ok(())
}
