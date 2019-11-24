use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type IpcMain;

    #[wasm_bindgen(js_name = "ipcMain")]
    pub static ipc_main: IpcMain;
}
