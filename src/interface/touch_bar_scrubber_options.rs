use js_sys::{Array, Function, JsString};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchBarScrubberOptions {
    continuous: Option<bool>,
    highlight: Option<Function>,
    items: Array,
    mode: Option<JsString>,
    overlay_style: Option<JsString>,
    select: Option<Function>,
    selected_style: Option<JsString>,
    show_arrow_buttons: Option<bool>,
}

#[wasm_bindgen]
impl TouchBarScrubberOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        continuous: Option<bool>,
        highlight: Option<Function>,
        items: Array,
        mode: Option<JsString>,
        overlay_style: Option<JsString>,
        select: Option<Function>,
        selected_style: Option<JsString>,
        show_arrow_buttons: Option<bool>,
    ) -> TouchBarScrubberOptions {
        TouchBarScrubberOptions {
            continuous,
            highlight,
            items,
            mode,
            overlay_style,
            select,
            selected_style,
            show_arrow_buttons,
        }
    }

    pub fn new(items: Array) -> TouchBarScrubberOptions {
        let continuous = Default::default();
        let highlight = Default::default();
        let mode = Default::default();
        let overlay_style = Default::default();
        let select = Default::default();
        let selected_style = Default::default();
        let show_arrow_buttons = Default::default();
        TouchBarScrubberOptions::new_with_values(
            continuous,
            highlight,
            items,
            mode,
            overlay_style,
            select,
            selected_style,
            show_arrow_buttons,
        )
    }

    #[wasm_bindgen(getter)]
    pub fn continuous(&self) -> Option<bool> {
        self.continuous
    }

    #[wasm_bindgen(setter)]
    pub fn set_continuous(&mut self, value: Option<bool>) {
        self.continuous = value;
    }

    #[wasm_bindgen(getter)]
    pub fn highlight(&self) -> Option<Function> {
        self.highlight.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_highlight(&mut self, value: Option<Function>) {
        self.highlight = value;
    }

    #[wasm_bindgen(getter)]
    pub fn items(&self) -> Array {
        self.items.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_items(&mut self, value: Array) {
        self.items = value;
    }

    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> Option<JsString> {
        self.mode.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_mode(&mut self, value: Option<JsString>) {
        self.mode = value;
    }

    #[wasm_bindgen(getter, js_name = "overlayStyle")]
    pub fn overlay_style(&self) -> Option<JsString> {
        self.overlay_style.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_overlay_style(&mut self, value: Option<JsString>) {
        self.overlay_style = value;
    }

    #[wasm_bindgen(getter)]
    pub fn select(&self) -> Option<Function> {
        self.select.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_select(&mut self, value: Option<Function>) {
        self.select = value;
    }

    #[wasm_bindgen(getter, js_name = "selectedStyle")]
    pub fn selected_style(&self) -> Option<JsString> {
        self.selected_style.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_selected_style(&mut self, value: Option<JsString>) {
        self.selected_style = value;
    }

    #[wasm_bindgen(getter, js_name = "showArrowButtons")]
    pub fn show_arrow_buttons(&self) -> Option<bool> {
        self.show_arrow_buttons
    }

    #[wasm_bindgen(setter)]
    pub fn set_show_arrow_buttons(&mut self, value: Option<bool>) {
        self.show_arrow_buttons = value;
    }
}
