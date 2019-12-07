use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VisibleOnAllWorkspacesOptions {
    visible_on_full_screen: Option<bool>,
}

#[wasm_bindgen]
impl VisibleOnAllWorkspacesOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(visible_on_full_screen: Option<bool>) -> VisibleOnAllWorkspacesOptions {
        VisibleOnAllWorkspacesOptions { visible_on_full_screen }
    }

    pub fn new() -> VisibleOnAllWorkspacesOptions {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "visibleOnFullScreen")]
    pub fn visible_on_full_screen(self) -> Option<bool> {
        self.visible_on_full_screen
    }

    #[wasm_bindgen(setter)]
    pub fn set_visible_on_full_screen(mut self, value: Option<bool>) {
        self.visible_on_full_screen = value;
    }
}
