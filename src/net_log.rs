use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type NetLog;

    #[wasm_bindgen(js_name = "netLog")]
    pub static net_log: NetLog;
}
