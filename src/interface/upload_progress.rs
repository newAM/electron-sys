use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct UploadProgress {
    active: bool,
    current: usize,
    started: bool,
    total: usize,
}

#[wasm_bindgen]
impl UploadProgress {
    #[wasm_bindgen(constructor)]
    pub fn new(active: bool, current: usize, started: bool, total: usize) -> UploadProgress {
        UploadProgress {
            active,
            current,
            started,
            total,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn active(&self) -> bool {
        self.active
    }

    #[wasm_bindgen(setter)]
    pub fn set_active(&mut self, value: bool) {
        self.active = value;
    }

    #[wasm_bindgen(getter)]
    pub fn current(&self) -> usize {
        self.current
    }

    #[wasm_bindgen(setter)]
    pub fn set_current(&mut self, value: usize) {
        self.current = value;
    }

    #[wasm_bindgen(getter)]
    pub fn started(&self) -> bool {
        self.started
    }

    #[wasm_bindgen(setter)]
    pub fn set_started(&mut self, value: bool) {
        self.started = value;
    }

    #[wasm_bindgen(getter)]
    pub fn total(&self) -> usize {
        self.total
    }

    #[wasm_bindgen(setter)]
    pub fn set_total(&mut self, value: usize) {
        self.total = value;
    }
}
