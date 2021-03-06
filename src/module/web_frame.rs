use crate::interface::{ResourceUsage, SpellCheckProvider, WorldInfo};
use js_sys::{JsString, Promise};
use node_sys::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type WebFrame;

    #[wasm_bindgen(js_name = "webFrame")]
    pub static web_frame: WebFrame;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "clearCache")]
    pub fn clear_cache(this: &WebFrame);

    #[must_use]
    #[wasm_bindgen(method, js_name = "executeJavaScript")]
    pub fn execute_java_script(this: &WebFrame, code: &str, user_gesture: Option<bool>) -> Promise;

    #[wasm_bindgen(method, js_name = "executeJavaScriptInIsolatedWorld")]
    pub fn execute_java_script_in_isolated_world(
        this: &WebFrame,
        world_id: u32,
        scripts: Box<[JsValue]>,
        user_gesture: Option<bool>,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = "findFrameByName")]
    pub fn find_frame_by_name(this: &WebFrame, name: &str) -> WebFrame;

    #[wasm_bindgen(method, js_name = "findFrameByRoutingId")]
    pub fn find_frame_by_routing_id(this: &WebFrame, routing_id: u32) -> WebFrame;

    #[wasm_bindgen(method, js_name = "getFrameForSelector")]
    pub fn get_frame_for_selector(this: &WebFrame, selector: &str) -> WebFrame;

    #[wasm_bindgen(method, js_name = "getResourceUsage")]
    pub fn get_resource_usage(this: &WebFrame) -> ResourceUsage;

    #[wasm_bindgen(method, js_name = "getZoomFactor")]
    pub fn get_zoom_factor(this: &WebFrame) -> f64;

    #[wasm_bindgen(method, js_name = "getZoomLevel")]
    pub fn get_zoom_level(this: &WebFrame) -> f64;

    #[wasm_bindgen(method, js_name = "insertCSS")]
    pub fn insert_css(this: &WebFrame, css: &str) -> JsString;

    #[wasm_bindgen(method, js_name = "insertText")]
    pub fn insert_text(this: &WebFrame, text: &str);

    #[wasm_bindgen(method, js_name = "removeInsertedCSS")]
    pub fn remove_inserted_css(this: &WebFrame, key: &str);

    #[wasm_bindgen(method, js_name = "setIsolatedWorldInfo")]
    pub fn set_isolated_world_info(this: &WebFrame, world_id: u32, info: WorldInfo);

    #[wasm_bindgen(method, js_name = "setLayoutZoomLevelLimits")]
    pub fn set_layout_zoom_level_limits(this: &WebFrame, min: f64, max: f64);

    #[wasm_bindgen(method, js_name = "setSpellCheckProvider")]
    pub fn set_spell_check_provider(this: &WebFrame, language: &str, provider: SpellCheckProvider);

    #[wasm_bindgen(method, js_name = "setVisualZoomLevelLimits")]
    pub fn set_visual_zoom_level_limits(this: &WebFrame, min: f64, max: f64);

    #[wasm_bindgen(method, js_name = "setZoomFactor")]
    pub fn set_zoom_factor(this: &WebFrame, factor: f64);

    #[wasm_bindgen(method, js_name = "setZoomLevel")]
    pub fn set_zoom_level(this: &WebFrame, level: f64);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "firstChild")] // readonly
    pub fn first_child(this: &WebFrame) -> Option<WebFrame>;

    #[wasm_bindgen(method, getter, js_name = "nextSibling")] // readonly
    pub fn next_sibling(this: &WebFrame) -> Option<WebFrame>;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn opener(this: &WebFrame) -> Option<WebFrame>;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn parent(this: &WebFrame) -> Option<WebFrame>;

    #[wasm_bindgen(method, getter, js_name = "routingId")] // readonly
    pub fn routing_id(this: &WebFrame) -> u32;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn top(this: &WebFrame) -> Option<WebFrame>;
}
