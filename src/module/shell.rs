use crate::interface::{OpenExternalOptions, ShortcutDetails};
use js_sys::{JsString, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type Shell;

    pub static shell: Shell;

    /// Play the beep sound.
    #[wasm_bindgen(method)]
    pub fn beep(this: &Shell);

    /// Move the given file to trash and returns a boolean status for the operation.
    #[wasm_bindgen(method, js_name = "moveItemToTrash")]
    pub fn move_item_to_trash(this: &Shell, full_path: &JsString, delete_on_fail: Option<bool>) -> bool;

    /// Open the given external protocol URL in the desktop's default manner. (For example, mailto:
    /// URLs in the user's default mail agent).
    #[wasm_bindgen(method, js_name = "openExternal")]
    pub fn open_external(this: &Shell, url: &JsString, options: OpenExternalOptions) -> Promise;

    /// Open the given file in the desktop's default manner.
    #[wasm_bindgen(method, js_name = "openItem")]
    pub fn open_item(this: &Shell, full_path: &JsString) -> bool;

    /// Resolve the shortcut link at `shortcut_path`.
    #[wasm_bindgen(method, js_name = "readShortcutLink")]
    pub fn read_shortcut_link(this: &Shell, shortcut_path: &JsString) -> ShortcutDetails;

    /// Show the given file in a file manager. If possible, select the file.
    #[wasm_bindgen(method, js_name = "showItemInFolder")]
    pub fn show_item_in_folder(this: &Shell, full_path: &JsString);

    /// Create or update a shortcut link at `shortcut_path`.
    #[wasm_bindgen(method, js_name = "writeShortcutLink")]
    pub fn write_shortcut_link(this: &Shell, operation: Option<JsString>, options: ShortcutDetails) -> bool;
}
