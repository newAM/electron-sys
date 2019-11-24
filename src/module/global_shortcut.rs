use js_sys::{Array, Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type GlobalShortcut;

    /// The `global_shortcut` module can register/unregister a global keyboard shortcut with the
    /// operating system so that you can customize the operations for various shortcuts.
    #[wasm_bindgen(js_name = "globalShortcut")]
    pub static global_shortcut: GlobalShortcut;

    /// Registers a global shortcut of `accelerator`. The `callback` is called when the registered
    /// shortcut is pressed by the user.
    #[wasm_bindgen(method)]
    pub fn register(this: &GlobalShortcut, accelerator: &JsString, callback: &Function);

    /// Registers a global shortcut of all `accelerator` items in `accelerators`. The `callback` is
    /// called when any of the registered shortcuts are pressed by the user.
    #[wasm_bindgen(method, js_name = "registerAll")]
    pub fn register_all(this: &GlobalShortcut, accelerators: &Array, callback: &Function);

    /// Returns whether this application has registered accelerator.
    #[wasm_bindgen(method, js_name = "isRegistered")]
    pub fn is_registered(this: &GlobalShortcut, accelerator: &JsString) -> bool;

    /// Unregisters the global shortcut of `accelerator`.
    #[wasm_bindgen(method)]
    pub fn unregister(this: &GlobalShortcut, accelerator: &JsString);

    /// Unregisters all of the global shortcuts.
    #[wasm_bindgen(method, js_name = "unregisterAll")]
    pub fn unregister_all(this: &GlobalShortcut);
}
