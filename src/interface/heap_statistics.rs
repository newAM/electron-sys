use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type HeapStatistics;

    #[wasm_bindgen(method, getter, js_name = "totalHeapSize")]
    pub fn total_heap_size(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "totalHeapSize")]
    pub fn set_total_heap_size(this: &HeapStatistics, value: u64);

    #[wasm_bindgen(method, getter, js_name = "totalHeapSizeExecutable")]
    pub fn total_heap_size_executable(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "totalHeapSizeExecutable")]
    pub fn set_total_heap_size_executable(this: &HeapStatistics, value: u64);

    #[wasm_bindgen(method, getter, js_name = "totalPhysicalSize")]
    pub fn total_physical_size(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "totalPhysicalSize")]
    pub fn set_total_physical_size(this: &HeapStatistics, value: u64);

    #[wasm_bindgen(method, getter, js_name = "totalAvailableSize")]
    pub fn total_available_size(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "totalAvailableSize")]
    pub fn set_total_available_size(this: &HeapStatistics, valeu: u64);

    #[wasm_bindgen(method, getter, js_name = "usedHeapSize")]
    pub fn used_heap_size(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "usedHeapSize")]
    pub fn set_used_heap_size(this: &HeapStatistics, value: u64);

    #[wasm_bindgen(method, getter, js_name = "heapSizeLimit")]
    pub fn heap_size_limit(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "heapSizeLimit")]
    pub fn set_heap_size_limit(this: &HeapStatistics, value: u64);

    #[wasm_bindgen(method, getter, js_name = "mallocedMemory")]
    pub fn malloced_memory(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "mallocedMemory")]
    pub fn set_malloced_memory(this: &HeapStatistics, value: u64);

    #[wasm_bindgen(method, getter, js_name = "peakMallocedMemory")]
    pub fn peak_malloced_memory(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "peakMallocedMemory")]
    pub fn set_peak_malloced_memory(this: &HeapStatistics, value: u64);

    #[wasm_bindgen(method, getter, js_name = "doesZapGarbage")]
    pub fn does_zap_garbage(this: &HeapStatistics) -> u64;

    #[wasm_bindgen(method, setter, js_name = "doesZapGarbage")]
    pub fn set_does_zap_garbage(this: &HeapStatistics, value: u64);
}
