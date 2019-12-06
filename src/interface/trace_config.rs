use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct TraceConfig {
    enable_argument_filter: Option<bool>,
    excluded_categories: Option<Array>,
    histogram_names: Option<Array>,
    included_categories: Option<Array>,
    included_process_ids: Option<Array>,
    memory_dump_config: Option<Object>,
    recording_mode: Option<JsString>,
    trace_buffer_size_in_events: Option<u32>,
    trace_buffer_size_in_kb: Option<u32>,
}

#[wasm_bindgen]
impl TraceConfig {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        enable_argument_filter: Option<bool>,
        excluded_categories: Option<Array>,
        histogram_names: Option<Array>,
        included_categories: Option<Array>,
        included_process_ids: Option<Array>,
        memory_dump_config: Option<Object>,
        recording_mode: Option<JsString>,
        trace_buffer_size_in_events: Option<u32>,
        trace_buffer_size_in_kb: Option<u32>,
    ) -> TraceConfig {
        TraceConfig {
            enable_argument_filter,
            excluded_categories,
            histogram_names,
            included_categories,
            included_process_ids,
            memory_dump_config,
            recording_mode,
            trace_buffer_size_in_events,
            trace_buffer_size_in_kb,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn enable_argument_filter(&self) -> Option<bool> {
        self.enable_argument_filter
    }

    #[wasm_bindgen(setter)]
    pub fn set_enable_argument_filter(&mut self, value: Option<bool>) {
        self.enable_argument_filter = value;
    }

    #[wasm_bindgen(getter)]
    pub fn excluded_categories(&self) -> Option<Array> {
        self.excluded_categories.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_excluded_categories(&mut self, value: Option<Array>) {
        self.excluded_categories = value;
    }

    #[wasm_bindgen(getter)]
    pub fn histogram_names(&self) -> Option<Array> {
        self.histogram_names.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_histogram_names(&mut self, value: Option<Array>) {
        self.histogram_names = value;
    }

    #[wasm_bindgen(getter)]
    pub fn included_categories(&self) -> Option<Array> {
        self.included_categories.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_included_categories(&mut self, value: Option<Array>) {
        self.included_categories = value;
    }

    #[wasm_bindgen(getter)]
    pub fn included_process_ids(&self) -> Option<Array> {
        self.included_process_ids.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_included_process_ids(&mut self, value: Option<Array>) {
        self.included_process_ids = value;
    }

    #[wasm_bindgen(getter)]
    pub fn memory_dump_config(&self) -> Option<Object> {
        self.memory_dump_config.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_memory_dump_config(&mut self, value: Option<Object>) {
        self.memory_dump_config = value;
    }

    #[wasm_bindgen(getter)]
    pub fn recording_mode(&self) -> Option<JsString> {
        self.recording_mode.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_recording_mode(&mut self, value: Option<JsString>) {
        self.recording_mode = value;
    }

    #[wasm_bindgen(getter)]
    pub fn trace_buffer_size_in_events(&self) -> Option<u32> {
        self.trace_buffer_size_in_events
    }

    #[wasm_bindgen(setter)]
    pub fn set_trace_buffer_size_in_events(&mut self, value: Option<u32>) {
        self.trace_buffer_size_in_events = value;
    }

    #[wasm_bindgen(getter)]
    pub fn trace_buffer_size_in_kb(&self) -> Option<u32> {
        self.trace_buffer_size_in_kb
    }

    #[wasm_bindgen(setter)]
    pub fn set_trace_buffer_size_in_kb(&mut self, value: Option<u32>) {
        self.trace_buffer_size_in_kb = value;
    }
}
