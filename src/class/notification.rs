use crate::interface::NotificationOptions;
use js_sys::{Array, JsString};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/notification
    pub type Notification;

    // Constructor

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<NotificationOptions>) -> Notification;

    // Static Methods

    #[wasm_bindgen(static_method_of = Notification, js_name = "isSupported")]
    pub fn is_supported() -> bool;

    // Instance Methods

    #[wasm_bindgen(method)]
    pub fn close(this: &Notification);

    #[wasm_bindgen(method)]
    pub fn show(this: &Notification);

    // Instance Properties

    #[wasm_bindgen(method, getter)]
    pub fn actions(this: &Notification) -> Array;

    #[wasm_bindgen(method, setter)]
    pub fn set_actions(this: &Notification, value: Array);

    #[wasm_bindgen(method, getter)]
    pub fn body(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_body(this: &Notification, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "closeButtonText")]
    pub fn close_button_text(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "closeButtonText")]
    pub fn set_close_button_text(this: &Notification, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "hasReply")]
    pub fn has_reply(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "hasReply")]
    pub fn set_has_reply(this: &Notification, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "replyPlaceholder")]
    pub fn reply_placeholder(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "replyPlaceholder")]
    pub fn set_reply_placeholder(this: &Notification, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn silent(this: &Notification) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_silent(this: &Notification, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn sound(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_sound(this: &Notification, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn subtitle(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_subtitle(this: &Notification, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn timeout_type(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_timeout_type(this: &Notification, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn title(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_title(this: &Notification, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn urgency(this: &Notification) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_urgency(this: &Notification, value: JsString);
}
