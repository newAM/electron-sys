use crate::interface::TouchBarColorPickerOptions;
use js_sys::JsString;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-color-picker
    pub type TouchBarColorPicker;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarColorPickerOptions) -> TouchBarColorPicker;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "availableColors")]
    pub fn available_colors(this: &TouchBarColorPicker) -> Box<[JsValue]>;

    #[wasm_bindgen(method, setter, js_name = "availableColors")]
    pub fn set_available_colors(this: &TouchBarColorPicker, value: Box<[JsValue]>);

    #[wasm_bindgen(method, getter, js_name = "selectedColor")]
    pub fn selected_color(this: &TouchBarColorPicker) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "selectedColor")]
    pub fn set_selected_color(this: &TouchBarColorPicker, value: JsString);
}
