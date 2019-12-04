use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Task {
    arguments: JsString,
    description: JsString,
    icon_index: usize,
    icon_path: JsString,
    program: JsString,
    title: JsString,
    working_directory: Option<JsString>,
}

#[wasm_bindgen]
impl Task {
    pub fn new(
        arguments: JsString,
        description: JsString,
        icon_index: usize,
        icon_path: JsString,
        program: JsString,
        title: JsString,
        working_directory: Option<JsString>,
    ) -> Task {
        Task {
            arguments,
            description,
            icon_index,
            icon_path,
            program,
            title,
            working_directory,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn arguments(&self) -> JsString {
        self.arguments.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_arguments(&mut self, value: JsString) {
        self.arguments = value
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> JsString {
        self.description.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_description(&mut self, value: JsString) {
        self.description = value;
    }

    #[wasm_bindgen(getter, js_name = "iconIndex")]
    pub fn icon_index(&self) -> usize {
        self.icon_index
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon_index(&mut self, value: usize) {
        self.icon_index = value;
    }

    #[wasm_bindgen(getter, js_name = "iconPath")]
    pub fn icon_path(&self) -> JsString {
        self.icon_path.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon_path(&mut self, value: JsString) {
        self.icon_path = value;
    }

    #[wasm_bindgen(getter)]
    pub fn program(&self) -> JsString {
        self.program.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_program(&mut self, value: JsString) {
        self.program = value;
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> JsString {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: JsString) {
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
