use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnableNetworkEmulationOptions {
    download_throughput: Option<u32>,
    latency: Option<u32>,
    offline: Option<bool>,
    upload_throughput: Option<u32>,
}

#[wasm_bindgen]
impl EnableNetworkEmulationOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        download_throughput: Option<u32>,
        latency: Option<u32>,
        offline: Option<bool>,
        upload_throughput: Option<u32>,
    ) -> EnableNetworkEmulationOptions {
        EnableNetworkEmulationOptions {
            download_throughput,
            latency,
            offline,
            upload_throughput,
        }
    }

    #[wasm_bindgen(getter, js_name = "downloadThroughput")]
    pub fn download_throughput(&self) -> Option<u32> {
        self.download_throughput
    }

    #[wasm_bindgen(setter)]
    pub fn set_download_throughput(&mut self, value: Option<u32>) {
        self.download_throughput = value;
    }

    #[wasm_bindgen(getter)]
    pub fn latency(&self) -> Option<u32> {
        self.latency
    }

    #[wasm_bindgen(setter)]
    pub fn set_latency(&mut self, value: Option<u32>) {
        self.latency = value;
    }

    #[wasm_bindgen(getter)]
    pub fn offline(&self) -> Option<bool> {
        self.offline
    }

    #[wasm_bindgen(setter)]
    pub fn set_offline(&mut self, value: Option<bool>) {
        self.offline = value;
    }

    #[wasm_bindgen(getter, js_name = "uploadThroughput")]
    pub fn upload_throughput(&self) -> Option<u32> {
        self.upload_throughput
    }

    #[wasm_bindgen(setter)]
    pub fn set_upload_throughput(&mut self, value: Option<u32>) {
        self.upload_throughput = value;
    }
}
