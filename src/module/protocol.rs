use crate::interface::Protocol;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    pub static protocol: Protocol;
}
