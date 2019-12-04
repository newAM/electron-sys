use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[derive(Clone, Debug, PartialEq)]
    pub type IoCounters;

    #[wasm_bindgen(method, getter, js_name = "otherOperationCount")]
    pub fn other_operation_count(this: &IoCounters) -> usize;

    #[wasm_bindgen(method, setter, js_name = "otherOperationCount")]
    pub fn set_other_operation_count(this: &IoCounters, value: usize);

    #[wasm_bindgen(method, getter, js_name = "otherTransferCount")]
    pub fn other_transfer_count(this: &IoCounters) -> usize;

    #[wasm_bindgen(method, setter, js_name = "otherTransferCount")]
    pub fn set_other_transfer_count(this: &IoCounters, value: usize);

    #[wasm_bindgen(method, getter, js_name = "readOperationCount")]
    pub fn read_operation_count(this: &IoCounters) -> usize;

    #[wasm_bindgen(method, setter, js_name = "readOperationCount")]
    pub fn set_read_operation_count(this: &IoCounters, value: usize);

    #[wasm_bindgen(method, getter, js_name = "readTransferCount")]
    pub fn read_transfer_count(this: &IoCounters) -> usize;

    #[wasm_bindgen(method, setter, js_name = "readTransferCount")]
    pub fn set_read_transfer_count(this: &IoCounters, value: usize);

    #[wasm_bindgen(method, getter, js_name = "writeOperationCount")]
    pub fn write_operation_count(this: &IoCounters) -> usize;

    #[wasm_bindgen(method, setter, js_name = "writeOperationCount")]
    pub fn set_write_operation_count(this: &IoCounters, value: usize);

    #[wasm_bindgen(method, getter, js_name = "writeTransferCount")]
    pub fn write_transfer_count(this: &IoCounters) -> usize;

    #[wasm_bindgen(method, setter, js_name = "writeTransferCount")]
    pub fn set_write_transfer_count(this: &IoCounters, value: usize);
}
