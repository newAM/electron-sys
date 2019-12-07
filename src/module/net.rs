use crate::{class::ClientRequest, interface::ClientRequestOptions};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type Net;

    pub static net: Net;

    #[wasm_bindgen(method)]
    pub fn request(this: &Net, options: ClientRequestOptions) -> ClientRequest;
}
