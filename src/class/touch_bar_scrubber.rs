use crate::interface::TouchBarScrubberOptions;
use js_sys::JsString;
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug)]
    /// Docs: http://electronjs.org/docs/api/touch-bar-scrubber
    pub type TouchBarScrubber;

    //*************//
    // Constructor //
    //*************//

    #[wasm_bindgen(constructor)]
    pub fn new(options: TouchBarScrubberOptions) -> TouchBarScrubber;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter)]
    pub fn continuous(this: &TouchBarScrubber) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_continuous(this: &TouchBarScrubber, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn items(this: &TouchBarScrubber) -> Box<[JsValue]>;

    #[wasm_bindgen(method, setter)]
    pub fn set_items(this: &TouchBarScrubber, value: Box<[JsValue]>);

    #[wasm_bindgen(method, getter)]
    pub fn mode(this: &TouchBarScrubber) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_mode(this: &TouchBarScrubber, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "overlayStyle")]
    pub fn overlay_style(this: &TouchBarScrubber) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "overlayStyle")]
    pub fn set_overlay_style(this: &TouchBarScrubber, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "selectedStyle")]
    pub fn selected_style(this: &TouchBarScrubber) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "selectedStyle")]
    pub fn set_selected_style(this: &TouchBarScrubber, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "showArrowButtons")]
    pub fn show_arrow_buttons(this: &TouchBarScrubber) -> bool;

    #[wasm_bindgen(method, setter, js_name = "showArrowButtons")]
    pub fn set_show_arrow_buttons(this: &TouchBarScrubber, value: bool);
}
