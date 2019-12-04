use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FromPartitionOptions {
    cache: bool,
}

#[wasm_bindgen]
impl FromPartitionOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(cache: bool) -> FromPartitionOptions {
        FromPartitionOptions { cache }
    }

    #[wasm_bindgen(getter)]
    pub fn cache(self) -> bool {
        self.cache
    }

    #[wasm_bindgen(setter)]
    pub fn set_cache(mut self, value: bool) {
        self.cache = value;
    }
}
