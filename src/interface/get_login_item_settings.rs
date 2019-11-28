use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GetLoginItemSettings {
    open_at_login: bool,
    open_as_hidden: bool,       // FIXME: macos
    was_opened_at_login: bool,  // FIXME: macos
    was_opened_as_hidden: bool, // FIXME: macos
    restore_state: bool,        // FIXME: macos
}

#[wasm_bindgen]
impl GetLoginItemSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(
        open_at_login: bool,
        open_as_hidden: bool,       // FIXME: macos
        was_opened_at_login: bool,  // FIXME: macos
        was_opened_as_hidden: bool, // FIXME: macos
        restore_state: bool,        // FIXME: macos
    ) -> GetLoginItemSettings {
        GetLoginItemSettings {
            open_at_login,
            open_as_hidden,
            was_opened_at_login,
            was_opened_as_hidden,
            restore_state,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn open_at_login(&self) -> bool {
        self.open_at_login
    }

    #[wasm_bindgen(setter)]
    pub fn set_open_at_login(&mut self, open_at_login: bool) {
        self.open_at_login = open_at_login;
    }

    #[wasm_bindgen(getter)]
    pub fn open_as_hidden(&self) -> bool {
        self.open_as_hidden
    }

    #[wasm_bindgen(setter)]
    pub fn set_open_as_hidden(&mut self, open_as_hidden: bool) {
        self.open_as_hidden = open_as_hidden;
    }

    #[wasm_bindgen(getter)]
    pub fn was_opened_at_login(&self) -> bool {
        self.was_opened_at_login
    }

    #[wasm_bindgen(setter)]
    pub fn set_was_opened_at_login(&mut self, was_opened_at_login: bool) {
        self.was_opened_at_login = was_opened_at_login;
    }

    #[wasm_bindgen(getter)]
    pub fn was_opened_as_hidden(&self) -> bool {
        self.was_opened_as_hidden
    }

    #[wasm_bindgen(setter)]
    pub fn set_was_opened_as_hidden(&mut self, was_opened_as_hidden: bool) {
        self.was_opened_as_hidden = was_opened_as_hidden;
    }

    #[wasm_bindgen(getter)]
    pub fn restore_state(&self) -> bool {
        self.restore_state
    }

    #[wasm_bindgen(setter)]
    pub fn set_restore_state(&mut self, restore_state: bool) {
        self.restore_state = restore_state;
    }
}