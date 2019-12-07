use crate::interface::AnimationSettings;
use js_sys::{Function, JsString, Object, Promise};
use node_sys::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type SystemPreferences;

    #[wasm_bindgen(js_name = "systemPreferences")]
    pub static system_preferences: SystemPreferences;

    // FIXME: event overrides

    //******************//
    // Instance Methods //
    //******************//

    #[must_use]
    #[wasm_bindgen(method, js_name = "askForMediaAccess")]
    pub fn ask_for_media_access(this: &SystemPreferences, media_type: &JsString) -> Promise;

    #[wasm_bindgen(method, js_name = "canPromptTouchID")]
    pub fn can_prompt_touch_id(this: &SystemPreferences) -> bool;

    #[wasm_bindgen(method, js_name = "getAccentColor")]
    pub fn get_accent_color(this: &SystemPreferences) -> JsString;

    #[wasm_bindgen(method, js_name = "getAnimationSettings")]
    pub fn get_animation_settings(this: &SystemPreferences) -> AnimationSettings;

    #[wasm_bindgen(method, js_name = "getAppLevelAppearance")]
    pub fn get_app_level_appearance(this: &SystemPreferences) -> JsString;

    #[wasm_bindgen(method, js_name = "getColor")]
    pub fn get_color(this: &SystemPreferences, color: &JsString) -> JsString;

    #[wasm_bindgen(method, js_name = "getEffectiveAppearance")]
    pub fn get_effective_appearance(this: &SystemPreferences) -> JsString;

    #[wasm_bindgen(method, js_name = "getMediaAccessStatus")]
    pub fn get_media_access_status(this: &SystemPreferences, media_type: &JsString) -> JsString;

    #[wasm_bindgen(method, js_name = "getSystemColor")]
    pub fn get_system_color(this: &SystemPreferences, color: &JsString) -> JsString;

    #[wasm_bindgen(method, js_name = "getUserDefault")]
    pub fn get_user_default(this: &SystemPreferences, key: &JsString, kind: &JsString) -> JsValue;

    #[wasm_bindgen(method, js_name = "isAeroGlassEnabled")]
    pub fn is_aero_glass_enabled(this: &SystemPreferences) -> bool;

    #[wasm_bindgen(method, js_name = "isSwipeTrackingFromScrollEventsEnabled")]
    pub fn is_swipe_tracking_from_scroll_events_enabled(this: &SystemPreferences) -> bool;

    #[wasm_bindgen(method, js_name = "isTrustedAccessibilityClient")]
    pub fn is_trusted_accessibility_client(this: &SystemPreferences, prompt: bool) -> bool;

    #[wasm_bindgen(method, js_name = "postLocalNotification")]
    pub fn post_local_notification(this: &SystemPreferences, event: &JsString, user_info: &Object);

    #[wasm_bindgen(method, js_name = "postNotification")]
    pub fn post_notification(
        this: &SystemPreferences,
        event: &JsString,
        user_info: &Object,
        deliver_immediately: Option<bool>,
    );

    #[wasm_bindgen(method, js_name = "postWorkspaceNotification")]
    pub fn post_workspace_notification(this: &SystemPreferences, event: &JsString, user_info: &Object);

    #[must_use]
    #[wasm_bindgen(method, js_name = "promptTouchID")]
    pub fn prompt_touch_id(this: &SystemPreferences, reason: &JsString) -> Promise;

    #[wasm_bindgen(method, js_name = "registerDefaults")]
    pub fn register_defaults(this: &SystemPreferences, defaults: &Object);

    #[wasm_bindgen(method, js_name = "removeUserDefaults")]
    pub fn remove_user_defaults(this: &SystemPreferences, key: &JsString);

    #[wasm_bindgen(method, js_name = "setUserDefaults")]
    pub fn set_user_defaults(this: &SystemPreferences, key: &JsString, kind: &JsString, value: &JsString);

    #[wasm_bindgen(method, js_name = "subscribeLocalNotification")]
    pub fn subscribe_local_notification(this: &SystemPreferences, event: &JsString, callback: &Function) -> u32;

    #[wasm_bindgen(method, js_name = "subscribeNotification")]
    pub fn subscribe_notification(this: &SystemPreferences, event: &JsString, callback: &Function) -> u32;

    #[wasm_bindgen(method, js_name = "subscribeWorkspaceNotification")]
    pub fn subscribe_workspace_notification(this: &SystemPreferences, event: &JsString, callback: &Function);

    #[wasm_bindgen(method, js_name = "unsubscribeLocalNotification")]
    pub fn unsubscribe_local_notification(this: &SystemPreferences, id: u32);

    #[wasm_bindgen(method, js_name = "unsubscribeNotification")]
    pub fn unsubscribe_notification(this: &SystemPreferences, id: u32);

    #[wasm_bindgen(method, js_name = "unsubscribeWorkspaceNotification")]
    pub fn unsubscribe_workspace_notification(this: &SystemPreferences, id: u32);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "appLevelAppearance")]
    pub fn app_level_appearance(this: &SystemPreferences) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "appLevelAppearance")]
    pub fn set_app_level_appearance(this: &SystemPreferences, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "effectiveAppearance")] // readonly
    pub fn effective_appearance(this: &SystemPreferences) -> JsString;
}
