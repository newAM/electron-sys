use js_sys::Reflect;
use node_sys::crypto;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Event, HtmlInputElement};

#[wasm_bindgen]
pub fn handler() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    let clo = {
        let document = document.clone();
        Closure::wrap(Box::new(move || {
            let text = &document
                .get_element_by_id("text-input")
                .unwrap_throw()
                .unchecked_into::<HtmlInputElement>()
                .value()
                .into();

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
    clo.forget();
    Ok(())
}
