use crate::interface::{OpenExternalOptions, ShortcutDetails};
use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type Shell;

    pub static shell: Shell;

    #[wasm_bindgen(method)]
    pub fn beep(this: &Shell);

    #[wasm_bindgen(method, js_name = "moveItemToTrash")]
    pub fn move_item_to_trash(this: &Shell, full_path: &str, delete_on_fail: Option<bool>) -> bool;

    #[must_use]
    #[wasm_bindgen(method, js_name = "openExternal")]
    pub fn open_external(this: &Shell, url: &str, options: OpenExternalOptions) -> Promise;

    #[wasm_bindgen(method, js_name = "openItem")]
    pub fn open_item(this: &Shell, full_path: &str) -> bool;

    #[wasm_bindgen(method, js_name = "readShortcutLink")]
    pub fn read_shortcut_link(this: &Shell, shortcut_path: &str) -> ShortcutDetails;

    #[wasm_bindgen(method, js_name = "showItemInFolder")]
    pub fn show_item_in_folder(this: &Shell, full_path: &str);

    #[wasm_bindgen(method, js_name = "writeShortcutLink")]
    pub fn write_shortcut_link(this: &Shell, operation: Option<&str>, options: ShortcutDetails) -> bool;
}
