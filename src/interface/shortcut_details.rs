use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ShortcutDetails {
    app_user_model_id: Option<JsString>,
    args: Option<JsString>,
    cwd: Option<JsString>,
    description: Option<JsString>,
    icon: Option<JsString>,
    icon_index: Option<usize>,
    target: JsString,
}

#[wasm_bindgen]
impl ShortcutDetails {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        app_user_model_id: Option<JsString>,
        args: Option<JsString>,
        cwd: Option<JsString>,
        description: Option<JsString>,
        icon: Option<JsString>,
        icon_index: Option<usize>,
        target: JsString,
    ) -> ShortcutDetails {
        ShortcutDetails {
            app_user_model_id,
            args,
            cwd,
            description,
            icon,
            icon_index,
            target,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn app_user_model_id(&self) -> Option<JsString> {
        self.app_user_model_id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_app_user_model_id(&mut self, value: Option<JsString>) {
        self.app_user_model_id = value;
    }

    #[wasm_bindgen(getter)]
    pub fn args(&self) -> Option<JsString> {
        self.args.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_args(&mut self, value: Option<JsString>) {
        self.args = value;
    }

    #[wasm_bindgen(getter)]
    pub fn cwd(&self) -> Option<JsString> {
        self.cwd.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_cwd(&mut self, value: Option<JsString>) {
        self.cwd = value;
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> Option<JsString> {
        self.description.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_description(&mut self, value: Option<JsString>) {
        self.description = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> Option<JsString> {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: Option<JsString>) {
        self.icon = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon_index(&self) -> Option<usize> {
        self.icon_index
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon_index(&mut self, value: Option<usize>) {
        self.icon_index = value;
    }

    #[wasm_bindgen(getter)]
    pub fn target(&self) -> JsString {
        self.target.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_target(&mut self, value: JsString) {
        self.target = value;
    }
}
