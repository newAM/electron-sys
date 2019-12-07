use crate::{class::Session, interface::FromPartitionOptions};
use js_sys::JsString;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/session
    pub type SessionModule;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "fromPartition")]
    pub fn from_partition(this: &SessionModule, partition: &JsString, options: Option<FromPartitionOptions>)
    -> Session;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "defaultSession")]
    pub fn default_session(this: &SessionModule) -> Session;
}
