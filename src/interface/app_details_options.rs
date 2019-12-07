use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppDetailsOptions {
    app_icon_index: Option<JsString>,
    app_icon_path: Option<JsString>,
    app_id: Option<JsString>,
    relaunch_command: Option<JsString>,
    relaunch_display_name: Option<JsString>,
}

#[wasm_bindgen]
impl AppDetailsOptions {
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        app_icon_index: Option<JsString>,
        app_icon_path: Option<JsString>,
        app_id: Option<JsString>,
        relaunch_command: Option<JsString>,
        relaunch_display_name: Option<JsString>,
    ) -> AppDetailsOptions {
        AppDetailsOptions {
            app_icon_index,
            app_icon_path,
            app_id,
            relaunch_command,
            relaunch_display_name,
        }
    }

    pub fn new() -> AppDetailsOptions {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "appIconIndex")]
    pub fn app_icon_index(&self) -> Option<JsString> {
        self.app_icon_index.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_app_icon_index(&mut self, value: Option<JsString>) {
        self.app_icon_index = value;
    }

    #[wasm_bindgen(getter, js_name = "appIconPath")]
    pub fn app_icon_path(&self) -> Option<JsString> {
        self.app_icon_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_app_icon_path(&mut self, value: Option<JsString>) {
        self.app_icon_path = value;
    }

    #[wasm_bindgen(getter, js_name = "appId")]
    pub fn app_id(&self) -> Option<JsString> {
        self.app_id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_app_id(&mut self, value: Option<JsString>) {
        self.app_id = value;
    }

    #[wasm_bindgen(getter, js_name = "relaunchCommand")]
    pub fn relaunch_command(&self) -> Option<JsString> {
        self.relaunch_command.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_relaunch_command(&mut self, value: Option<JsString>) {
        self.relaunch_command = value;
    }

    #[wasm_bindgen(getter, js_name = "relaunchDisplayName")]
    pub fn relaunch_display_name(&self) -> Option<JsString> {
        self.relaunch_display_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_relaunch_display_name(&mut self, value: Option<JsString>) {
        self.relaunch_display_name = value;
    }
}
