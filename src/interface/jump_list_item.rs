use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct JumpListItem {
    args: Option<JsString>,
    description: Option<JsString>,
    icon_index: Option<usize>,
    icon_path: Option<JsString>,
    kind: Option<JsString>,
    path: Option<JsString>,
    program: Option<JsString>,
    title: Option<JsString>,
    working_directory: Option<JsString>,
}

#[wasm_bindgen]
impl JumpListItem {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new(
        args: Option<JsString>,
        description: Option<JsString>,
        icon_index: Option<usize>,
        icon_path: Option<JsString>,
        kind: Option<JsString>,
        path: Option<JsString>,
        program: Option<JsString>,
        title: Option<JsString>,
        working_directory: Option<JsString>,
    ) -> JumpListItem {
        JumpListItem {
            args,
            description,
            icon_index,
            icon_path,
            kind,
            path,
            program,
            title,
            working_directory,
        }
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
    pub fn description(&self) -> Option<JsString> {
        self.description.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_description(&mut self, value: Option<JsString>) {
        self.description = value;
    }

    #[wasm_bindgen(getter, js_name = "iconIndex")]
    pub fn icon_index(&self) -> Option<usize> {
        self.icon_index
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon_index(&mut self, value: Option<usize>) {
        self.icon_index = value;
    }

    #[wasm_bindgen(getter, js_name = "iconPath")]
    pub fn icon_path(&self) -> Option<JsString> {
        self.icon_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon_path(&mut self, value: Option<JsString>) {
        self.icon_path = value;
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> Option<JsString> {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: Option<JsString>) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Option<JsString> {
        self.path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_path(&mut self, value: Option<JsString>) {
        self.path = value;
    }

    #[wasm_bindgen(getter)]
    pub fn program(&self) -> Option<JsString> {
        self.program.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_program(&mut self, value: Option<JsString>) {
        self.program = value;
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<JsString> {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: Option<JsString>) {
        self.title = value;
    }

    #[wasm_bindgen(getter, js_name = "workingDirectory")]
    pub fn working_directory(&self) -> Option<JsString> {
        self.working_directory.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_working_directory(&mut self, value: Option<JsString>) {
        self.working_directory = value;
    }
}
