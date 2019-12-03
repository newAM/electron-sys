use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BluetoothDevice {
    device_id: JsString,
    device_name: JsString,
}

#[wasm_bindgen]
impl BluetoothDevice {
    #[wasm_bindgen(constructor)]
    pub fn new(device_id: JsString, device_name: JsString) -> BluetoothDevice {
        BluetoothDevice { device_id, device_name }
    }

    #[wasm_bindgen(getter, js_name = "deviceId")]
    pub fn device_id(&self) -> JsString {
        self.device_id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_device_id(&mut self, value: JsString) {
        self.device_id = value;
    }

    #[wasm_bindgen(getter, js_name = "deviceName")]
    pub fn device_name(&self) -> JsString {
        self.device_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_device_name(&mut self, value: JsString) {
        self.device_name = value;
    }
}
