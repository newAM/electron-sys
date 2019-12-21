use crate::interface::{CookiesGetFilter, CookiesSetDetails};
use js_sys::Promise;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/cookies
    pub type Cookies;

    //******************//
    // Instance Methods //
    //******************//

    #[must_use]
    #[wasm_bindgen(method, js_name = "flushStore")]
    pub fn flush_store(this: &Cookies) -> Promise;

    #[must_use]
    #[wasm_bindgen(method)]
    pub fn get(this: &Cookies, filter: CookiesGetFilter) -> Promise;

    #[must_use]
    #[wasm_bindgen(method)]
    pub fn remove(this: &Cookies, url: &str, name: &str) -> Promise;

    #[must_use]
    #[wasm_bindgen(method)]
    pub fn set(this: &Cookies, details: CookiesSetDetails) -> Promise;
}
