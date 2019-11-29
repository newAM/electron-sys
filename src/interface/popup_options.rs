use crate::class::BrowserWindow;
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PopupOptions {
    callback: Option<Function>,
    positioning_item: Option<usize>,
    window: Option<BrowserWindow>,
    x: Option<usize>,
    y: Option<usize>,
}

#[wasm_bindgen]
impl PopupOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        callback: Option<Function>,
        positioning_item: Option<usize>,
        window: Option<BrowserWindow>,
        x: Option<usize>,
        y: Option<usize>,
    ) -> PopupOptions {
        PopupOptions {
            callback,
            positioning_item,
            window,
            x,
            y,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn callback(&self) -> Option<Function> {
        self.callback.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_callback(&mut self, value: Option<Function>) {
        self.callback = value;
    }

    #[wasm_bindgen(getter)]
    pub fn positioning_item(&self) -> Option<usize> {
        self.positioning_item
    }

    #[wasm_bindgen(setter)]
    pub fn set_positioning_item(&mut self, value: Option<usize>) {
        self.positioning_item = value;
    }

    #[wasm_bindgen(getter)]
    pub fn window(&self) -> Option<BrowserWindow> {
        self.window.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_window(&mut self, value: Option<BrowserWindow>) {
        self.window = value;
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> Option<usize> {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, value: Option<usize>) {
        self.x = value;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> Option<usize> {
        self.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, value: Option<usize>) {
        self.y = value;
    }
}
