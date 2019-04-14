use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    let body = document.body().unwrap_throw();
    let elem = document.create_element("p")?;
    elem.set_inner_html("Hello from Rust!");
    body.append_child(&elem)?;
    Ok(())
}
