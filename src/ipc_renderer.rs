use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type IpcRenderer;

    #[wasm_bindgen(js_name = "ipcRenderer")]
    pub static ipc_renderer: IpcRenderer;
}
