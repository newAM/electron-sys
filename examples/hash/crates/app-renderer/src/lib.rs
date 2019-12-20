use node_sys::crypto;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlTextAreaElement;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    let text_area = document
        .get_element_by_id("text-input")
        .unwrap_throw()
        .unchecked_into::<HtmlTextAreaElement>();
    let handler_clo = {
        let document = document.clone();
        let text_input = text_area.clone();
        Closure::wrap(Box::new(move || {
            let text = &text_input.value().into();

            let md5 = crypto::create_hash(&"md5".into(), None)
                .update_with_encoding(text, Some(&"utf8".into()))
                .digest_with_encoding(&"hex".into());
            document
                .get_element_by_id("md5-output")
                .unwrap_throw()
                .set_inner_html(String::from(md5).as_str());

            let sha1 = crypto::create_hash(&"sha1".into(), None)
                .update_with_encoding(text, Some(&"utf8".into()))
                .digest_with_encoding(&"hex".into());
            document
                .get_element_by_id("sha1-output")
                .unwrap_throw()
                .set_inner_html(String::from(sha1).as_str());

            let sha256 = crypto::create_hash(&"sha256".into(), None)
                .update_with_encoding(text, Some(&"utf8".into()))
                .digest_with_encoding(&"hex".into());
            document
                .get_element_by_id("sha256-output")
                .unwrap_throw()
                .set_inner_html(String::from(sha256).as_str());

            let sha512 = crypto::create_hash(&"sha512".into(), None)
                .update_with_encoding(text, Some(&"utf8".into()))
                .digest_with_encoding(&"hex".into());
            document
                .get_element_by_id("sha512-output")
                .unwrap_throw()
                .set_inner_html(String::from(sha512).as_str());
        }) as Box<dyn Fn()>)
    };
    let handler = handler_clo.as_ref().unchecked_ref();
    text_area.set_oninput(Some(handler));
    handler_clo.forget();
    Ok(())
}
