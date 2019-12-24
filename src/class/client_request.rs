use crate::interface::{ClientRequestOptions, UploadProgress};
use js_sys::{Function, JsString};
use node_sys::{events::EventEmitter, Buffer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    /// Docs: http://electronjs.org/docs/api/client-request
    pub type ClientRequest;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(options: ClientRequestOptions) -> ClientRequest;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn abort(this: &ClientRequest);

    #[wasm_bindgen(method)]
    pub fn end(this: &ClientRequest, chunk: Option<&Buffer>, encoding: Option<&str>, callback: Option<&Function>);

    #[wasm_bindgen(method)]
    pub fn follow_redirect(this: &ClientRequest);

    #[wasm_bindgen(method, js_name = "getHeader")]
    pub fn get_header(this: &ClientRequest, name: &str) -> JsString;

    #[wasm_bindgen(method, js_name = "getUploadProgress")]
    pub fn get_upload_progress(this: &ClientRequest) -> UploadProgress;

    #[wasm_bindgen(method, js_name = "removeHeader")]
    pub fn remove_header(this: &ClientRequest, name: &str);

    #[wasm_bindgen(method, js_name = "setHeader")]
    pub fn set_header(this: &ClientRequest, name: &str, value: &str);

    #[wasm_bindgen(method)]
    pub fn write(this: &ClientRequest, chunk: &Buffer, encoding: Option<&str>, callback: Option<&Function>);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "chunkedEncoding")]
    pub fn chunked_encoding(this: &ClientRequest) -> bool;

    #[wasm_bindgen(method, setter, js_name = "chunkedEncoding")]
    pub fn set_chunked_encoding(this: &ClientRequest, value: bool);
}
