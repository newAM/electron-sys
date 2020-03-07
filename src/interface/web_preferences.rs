use crate::{class::Session, interface::DefaultFontFamily};
use js_sys::{JsString, Object};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type WebPreferences;

    #[wasm_bindgen(method, getter, js_name = "accessibleTitle")]
    pub fn accessible_title(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "accessibleTitle")]
    pub fn set_accessible_title(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "additionalArguments")]
    pub fn additional_arguments(this: &WebPreferences) -> Option<Box<[JsValue]>>;

    #[wasm_bindgen(method, setter, js_name = "additionalArguments")]
    pub fn set_additional_arguments(this: &WebPreferences, value: Option<Box<[JsValue]>>);

    #[wasm_bindgen(method, getter)]
    pub fn affinity(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_affinity(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "allowRunningInsecureContent")]
    pub fn allow_running_insecure_content(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "allowRunningInsecureContent")]
    pub fn set_allow_running_insecure_content(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "autoplayPolicy")]
    pub fn autoplay_policy(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "autoplayPolicy")]
    pub fn set_autoplay_policy(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "backgroundThrottling")]
    pub fn background_throttling(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "backgroundThrottling")]
    pub fn set_background_throttling(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "contextIsolation")]
    pub fn context_isolation(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "contextIsolation")]
    pub fn set_context_isolation(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "defaultEncoding")]
    pub fn default_encoding(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "defaultEncoding")]
    pub fn set_default_encoding(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "defaultFontFamily")]
    pub fn default_font_family(this: &WebPreferences) -> Option<DefaultFontFamily>;

    #[wasm_bindgen(method, setter, js_name = "defaultFontFamily")]
    pub fn set_default_font_family(this: &WebPreferences, value: Option<DefaultFontFamily>);

    #[wasm_bindgen(method, getter, js_name = "defaultFontSize")]
    pub fn default_font_size(this: &WebPreferences) -> Option<u32>;

    #[wasm_bindgen(method, setter, js_name = "defaultFontSize")]
    pub fn set_default_font_size(this: &WebPreferences, value: Option<u32>);

    #[wasm_bindgen(method, getter, js_name = "defaultMonospaceFontSize")]
    pub fn default_monospace_font_size(this: &WebPreferences) -> Option<u32>;

    #[wasm_bindgen(method, setter, js_name = "defaultMonospaceFontSize")]
    pub fn set_default_monospace_font_size(this: &WebPreferences, value: Option<u32>);

    #[wasm_bindgen(method, getter, js_name = "devTools")]
    pub fn dev_tools(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "devTools")]
    pub fn set_dev_tools(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "disableBlinkFeatures")]
    pub fn disable_blink_features(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "disableBlinkFeatures")]
    pub fn set_disable_blink_features(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "disableHtmlFullscreenWindowResize")]
    pub fn disable_html_fullscreen_window_resize(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "disableHtmlFullscreenWindowResize")]
    pub fn set_disable_html_fullscreen_window_resize(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "enableBlinkFeatures")]
    pub fn enable_blink_features(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "enableBlinkFeatures")]
    pub fn set_enable_blink_features(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "enableRemoteModule")]
    pub fn enable_remote_module(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "enableRemoteModule")]
    pub fn set_enable_remote_module(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "experimentalFeatures")]
    pub fn experimental_features(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "experimentalFeatures")]
    pub fn set_experimental_features(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn images(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_images(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn javascript(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_javascript(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "minimumFontSize")]
    pub fn minimum_font_size(this: &WebPreferences) -> Option<u32>;

    #[wasm_bindgen(method, setter, js_name = "minimumFontSize")]
    pub fn set_minimum_font_size(this: &WebPreferences, value: Option<u32>);

    #[wasm_bindgen(method, getter, js_name = "nativeWindowOpen")]
    pub fn native_window_open(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "nativeWindowOpen")]
    pub fn set_native_window_open(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "navigateOnDragDrop")]
    pub fn navigate_on_drag_drop(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "navigateOnDragDrop")]
    pub fn set_navigate_on_drag_drop(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "nodeIntegrationInSubFrames")]
    pub fn node_integration_in_sub_frames(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "nodeIntegrationInSubFrames")]
    pub fn set_node_integration_in_sub_frames(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "nodeIntegrationInWorker")]
    pub fn node_integration_in_worker(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "nodeIntegrationInWorker")]
    pub fn set_node_integration_in_worker(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "nodeIntegration")]
    pub fn node_integration(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "nodeIntegration")]
    pub fn set_node_integration(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn offscreen(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_offscreen(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn partition(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_partition(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn plugins(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_plugins(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn preload(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_preload(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "safeDialogsMessage")]
    pub fn safe_dialogs_message(this: &WebPreferences) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "safeDialogsMessage")]
    pub fn set_safe_dialogs_message(this: &WebPreferences, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "safeDialogs")]
    pub fn safe_dialogs(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "safeDialogs")]
    pub fn set_safe_dialogs(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn sandbox(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_sandbox(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "scrollBounce")]
    pub fn scroll_bounce(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "scrollBounce")]
    pub fn set_scroll_bounce(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn session(this: &WebPreferences) -> Option<Session>;

    #[wasm_bindgen(method, setter)]
    pub fn set_session(this: &WebPreferences, value: Option<Session>);

    #[wasm_bindgen(method, getter)]
    pub fn spellcheck(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_spellcheck(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "textAreasAreResizable")]
    pub fn text_areas_are_resizable(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "textAreasAreResizable")]
    pub fn set_text_areas_are_resizable(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "webSecurity")]
    pub fn web_security(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "webSecurity")]
    pub fn set_web_security(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn webgl(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_webgl(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "webviewTag")]
    pub fn webview_tag(this: &WebPreferences) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "webviewTag")]
    pub fn set_webview_tag(this: &WebPreferences, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "zoomFactor")]
    pub fn zoom_factor(this: &WebPreferences) -> Option<f32>;

    #[wasm_bindgen(method, setter, js_name = "zoomFactor")]
    pub fn set_zoom_factor(this: &WebPreferences, value: Option<f32>);
}

impl Default for WebPreferences {
    fn default() -> Self {
        Object::new().unchecked_into()
    }
}
