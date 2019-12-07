use crate::{
    class::BrowserWindow,
    interface::{Display, Point, Rectangle},
};
use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type Screen;

    pub static screen: Screen;

    // FIXME: event overloads

    #[wasm_bindgen(method, js_name = "dipToScreenPoint")]
    pub fn dip_to_screen_point(this: &Screen, point: Point) -> Point;

    #[wasm_bindgen(method, js_name = "dipToScreenPoint")]
    pub fn dip_to_screen_rect(this: &Screen, window: BrowserWindow, rect: Rectangle) -> Rectangle;

    #[wasm_bindgen(method, js_name = "getCursorScreenPoint")]
    pub fn get_all_displays(this: &Screen) -> Array;

    #[wasm_bindgen(method, js_name = "getDisplayMatching")]
    pub fn get_display_matching(this: &Screen, rect: Rectangle) -> Display;

    #[wasm_bindgen(method, js_name = "getDisplayNearestPoint")]
    pub fn get_display_nearest_point(this: &Screen, point: Point) -> Display;

    #[wasm_bindgen(method, js_name = "getPrimaryDisplay")]
    pub fn get_primary_display(this: &Screen) -> Display;

    #[wasm_bindgen(method, js_name = "screen_to_dip_point")]
    pub fn screen_to_dip_point(this: &Screen, point: Point) -> Point;

    #[wasm_bindgen(method, js_name = "screen_to_dip_rect")]
    pub fn screen_to_dip_rect(this: &Screen, rect: Rectangle) -> Rectangle;
}
