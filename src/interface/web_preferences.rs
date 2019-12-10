use crate::{class::Session, interface::DefaultFontFamily};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct WebPreferences {
    accessible_title: Option<JsString>,
    additional_arguments: Option<Box<[JsValue]>>,
    affinity: Option<JsString>,
    allow_running_insecure_content: Option<bool>,
    autoplay_policy: Option<JsString>,
    background_throttling: Option<bool>,
    context_isolation: Option<bool>,
    default_encoding: Option<JsString>,
    default_font_family: Option<DefaultFontFamily>,
    default_font_size: Option<u32>,
    default_monospace_font_size: Option<u32>,
    dev_tools: Option<bool>,
    disable_blink_features: Option<JsString>,
    disable_html_fullscreen_window_resize: Option<bool>,
    enable_blink_features: Option<JsString>,
    enable_remote_module: Option<bool>,
    experimental_features: Option<bool>,
    images: Option<bool>,
    javascript: Option<bool>,
    minimum_font_size: Option<u32>,
    native_window_open: Option<bool>,
    navigate_on_drag_drop: Option<bool>,
    node_integration_in_sub_frames: Option<bool>,
    node_integration_in_worker: Option<bool>,
    node_integration: Option<bool>,
    offscreen: Option<bool>,
    partition: Option<JsString>,
    plugins: Option<bool>,
    preload: Option<JsString>,
    safe_dialogs_message: Option<JsString>,
    safe_dialogs: Option<bool>,
    sandbox: Option<bool>,
    scroll_bounce: Option<bool>,
    session: Option<Session>,
    spellcheck: Option<bool>,
    text_areas_are_resizable: Option<bool>,
    web_security: Option<bool>,
    webgl: Option<bool>,
    webview_tag: Option<bool>,
    zoom_factor: Option<f32>,
}

#[wasm_bindgen]
impl WebPreferences {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        accessible_title: Option<JsString>,
        additional_arguments: Option<Box<[JsValue]>>,
        affinity: Option<JsString>,
        allow_running_insecure_content: Option<bool>,
        autoplay_policy: Option<JsString>,
        background_throttling: Option<bool>,
        context_isolation: Option<bool>,
        default_encoding: Option<JsString>,
        default_font_family: Option<DefaultFontFamily>,
        default_font_size: Option<u32>,
        default_monospace_font_size: Option<u32>,
        dev_tools: Option<bool>,
        disable_blink_features: Option<JsString>,
        disable_html_fullscreen_window_resize: Option<bool>,
        enable_blink_features: Option<JsString>,
        enable_remote_module: Option<bool>,
        experimental_features: Option<bool>,
        images: Option<bool>,
        javascript: Option<bool>,
        minimum_font_size: Option<u32>,
        native_window_open: Option<bool>,
        navigate_on_drag_drop: Option<bool>,
        node_integration_in_sub_frames: Option<bool>,
        node_integration_in_worker: Option<bool>,
        node_integration: Option<bool>,
        offscreen: Option<bool>,
        partition: Option<JsString>,
        plugins: Option<bool>,
        preload: Option<JsString>,
        safe_dialogs_message: Option<JsString>,
        safe_dialogs: Option<bool>,
        sandbox: Option<bool>,
        scroll_bounce: Option<bool>,
        session: Option<Session>,
        spellcheck: Option<bool>,
        text_areas_are_resizable: Option<bool>,
        web_security: Option<bool>,
        webgl: Option<bool>,
        webview_tag: Option<bool>,
        zoom_factor: Option<f32>,
    ) -> WebPreferences {
        WebPreferences {
            accessible_title,
            additional_arguments,
            affinity,
            allow_running_insecure_content,
            autoplay_policy,
            background_throttling,
            context_isolation,
            default_encoding,
            default_font_family,
            default_font_size,
            default_monospace_font_size,
            dev_tools,
            disable_blink_features,
            disable_html_fullscreen_window_resize,
            enable_blink_features,
            enable_remote_module,
            experimental_features,
            images,
            javascript,
            minimum_font_size,
            native_window_open,
            navigate_on_drag_drop,
            node_integration_in_sub_frames,
            node_integration_in_worker,
            node_integration,
            offscreen,
            partition,
            plugins,
            preload,
            safe_dialogs_message,
            safe_dialogs,
            sandbox,
            scroll_bounce,
            session,
            spellcheck,
            text_areas_are_resizable,
            web_security,
            webgl,
            webview_tag,
            zoom_factor,
        }
    }

    #[wasm_bindgen(getter, js_name = "accessibleTitle")]
    pub fn accessible_title(&self) -> Option<JsString> {
        self.accessible_title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_accessible_title(&mut self, value: Option<JsString>) {
        self.accessible_title = value;
    }

    #[wasm_bindgen(getter, js_name = "additionalArguments")]
    pub fn additional_arguments(&self) -> Option<Box<[JsValue]>> {
        self.additional_arguments.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_additional_arguments(&mut self, value: Option<Box<[JsValue]>>) {
        self.additional_arguments = value;
    }

    #[wasm_bindgen(getter)]
    pub fn affinity(&self) -> Option<JsString> {
        self.affinity.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_affinity(&mut self, value: Option<JsString>) {
        self.affinity = value;
    }

    #[wasm_bindgen(getter, js_name = "allowRunningInsecureContent")]
    pub fn allow_running_insecure_content(&self) -> Option<bool> {
        self.allow_running_insecure_content
    }

    #[wasm_bindgen(setter)]
    pub fn set_allow_running_insecure_content(&mut self, value: Option<bool>) {
        self.allow_running_insecure_content = value;
    }

    #[wasm_bindgen(getter, js_name = "autoplayPolicy")]
    pub fn autoplay_policy(&self) -> Option<JsString> {
        self.autoplay_policy.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_autoplay_policy(&mut self, value: Option<JsString>) {
        self.autoplay_policy = value;
    }

    #[wasm_bindgen(getter, js_name = "backgroundThrottling")]
    pub fn background_throttling(&self) -> Option<bool> {
        self.background_throttling
    }

    #[wasm_bindgen(setter)]
    pub fn set_background_throttling(&mut self, value: Option<bool>) {
        self.background_throttling = value;
    }

    #[wasm_bindgen(getter, js_name = "contextIsolation")]
    pub fn context_isolation(&self) -> Option<bool> {
        self.context_isolation
    }

    #[wasm_bindgen(setter)]
    pub fn set_context_isolation(&mut self, value: Option<bool>) {
        self.context_isolation = value;
    }

    #[wasm_bindgen(getter, js_name = "defaultEncoding")]
    pub fn default_encoding(&self) -> Option<JsString> {
        self.default_encoding.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_default_encoding(&mut self, value: Option<JsString>) {
        self.default_encoding = value;
    }

    #[wasm_bindgen(getter, js_name = "defaultFontFamily")]
    pub fn default_font_family(&self) -> Option<DefaultFontFamily> {
        self.default_font_family.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_default_font_family(&mut self, value: Option<DefaultFontFamily>) {
        self.default_font_family = value;
    }

    #[wasm_bindgen(getter, js_name = "defaultFontSize")]
    pub fn default_font_size(&self) -> Option<u32> {
        self.default_font_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_default_font_size(&mut self, value: Option<u32>) {
        self.default_font_size = value;
    }

    #[wasm_bindgen(getter, js_name = "defaultMonospaceFontSize")]
    pub fn default_monospace_font_size(&self) -> Option<u32> {
        self.default_monospace_font_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_default_monospace_font_size(&mut self, value: Option<u32>) {
        self.default_monospace_font_size = value;
    }

    #[wasm_bindgen(getter, js_name = "devTools")]
    pub fn dev_tools(&self) -> Option<bool> {
        self.dev_tools
    }

    #[wasm_bindgen(setter)]
    pub fn set_dev_tools(&mut self, value: Option<bool>) {
        self.dev_tools = value;
    }

    #[wasm_bindgen(getter, js_name = "disableBlinkFeatures")]
    pub fn disable_blink_features(&self) -> Option<JsString> {
        self.disable_blink_features.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_disable_blink_features(&mut self, value: Option<JsString>) {
        self.disable_blink_features = value;
    }

    #[wasm_bindgen(getter, js_name = "disableHtmlFullscreenWindowResize")]
    pub fn disable_html_fullscreen_window_resize(&self) -> Option<bool> {
        self.disable_html_fullscreen_window_resize
    }

    #[wasm_bindgen(setter)]
    pub fn set_disable_html_fullscreen_window_resize(&mut self, value: Option<bool>) {
        self.disable_html_fullscreen_window_resize = value;
    }

    #[wasm_bindgen(getter, js_name = "enableBlinkFeatures")]
    pub fn enable_blink_features(&self) -> Option<JsString> {
        self.enable_blink_features.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_enable_blink_features(&mut self, value: Option<JsString>) {
        self.enable_blink_features = value;
    }

    #[wasm_bindgen(getter, js_name = "enableRemoteModule")]
    pub fn enable_remote_module(&self) -> Option<bool> {
        self.enable_remote_module
    }

    #[wasm_bindgen(setter)]
    pub fn set_enable_remote_module(&mut self, value: Option<bool>) {
        self.enable_remote_module = value;
    }

    #[wasm_bindgen(getter, js_name = "experimentalFeatures")]
    pub fn experimental_features(&self) -> Option<bool> {
        self.experimental_features
    }

    #[wasm_bindgen(setter)]
    pub fn set_experimental_features(&mut self, value: Option<bool>) {
        self.experimental_features = value;
    }

    #[wasm_bindgen(getter)]
    pub fn images(&self) -> Option<bool> {
        self.images
    }

    #[wasm_bindgen(setter)]
    pub fn set_images(&mut self, value: Option<bool>) {
        self.images = value;
    }

    #[wasm_bindgen(getter)]
    pub fn javascript(&self) -> Option<bool> {
        self.javascript
    }

    #[wasm_bindgen(setter)]
    pub fn set_javascript(&mut self, value: Option<bool>) {
        self.javascript = value;
    }

    #[wasm_bindgen(getter, js_name = "minimumFontSize")]
    pub fn minimum_font_size(&self) -> Option<u32> {
        self.minimum_font_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_minimum_font_size(&mut self, value: Option<u32>) {
        self.minimum_font_size = value;
    }

    #[wasm_bindgen(getter, js_name = "nativeWindowOpen")]
    pub fn native_window_open(&self) -> Option<bool> {
        self.native_window_open
    }

    #[wasm_bindgen(setter)]
    pub fn set_native_window_open(&mut self, value: Option<bool>) {
        self.native_window_open = value;
    }

    #[wasm_bindgen(getter, js_name = "navigateOnDragDrop")]
    pub fn navigate_on_drag_drop(&self) -> Option<bool> {
        self.navigate_on_drag_drop
    }

    #[wasm_bindgen(setter)]
    pub fn set_navigate_on_drag_drop(&mut self, value: Option<bool>) {
        self.navigate_on_drag_drop = value;
    }

    #[wasm_bindgen(getter, js_name = "nodeIntegrationInSubFrames")]
    pub fn node_integration_in_sub_frames(&self) -> Option<bool> {
        self.node_integration_in_sub_frames
    }

    #[wasm_bindgen(setter)]
    pub fn set_node_integration_in_sub_frames(&mut self, value: Option<bool>) {
        self.node_integration_in_sub_frames = value;
    }

    #[wasm_bindgen(getter, js_name = "nodeIntegrationInWorker")]
    pub fn node_integration_in_worker(&self) -> Option<bool> {
        self.node_integration_in_worker
    }

    #[wasm_bindgen(setter)]
    pub fn set_node_integration_in_worker(&mut self, value: Option<bool>) {
        self.node_integration_in_worker = value;
    }

    #[wasm_bindgen(getter, js_name = "nodeIntegration")]
    pub fn node_integration(&self) -> Option<bool> {
        self.node_integration
    }

    #[wasm_bindgen(setter)]
    pub fn set_node_integration(&mut self, value: Option<bool>) {
        self.node_integration = value;
    }

    #[wasm_bindgen(getter)]
    pub fn offscreen(&self) -> Option<bool> {
        self.offscreen
    }

    #[wasm_bindgen(setter)]
    pub fn set_offscreen(&mut self, value: Option<bool>) {
        self.offscreen = value;
    }

    #[wasm_bindgen(getter)]
    pub fn partition(&self) -> Option<JsString> {
        self.partition.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_partition(&mut self, value: Option<JsString>) {
        self.partition = value;
    }

    #[wasm_bindgen(getter)]
    pub fn plugins(&self) -> Option<bool> {
        self.plugins
    }

    #[wasm_bindgen(setter)]
    pub fn set_plugins(&mut self, value: Option<bool>) {
        self.plugins = value;
    }

    #[wasm_bindgen(getter)]
    pub fn preload(&self) -> Option<JsString> {
        self.preload.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_preload(&mut self, value: Option<JsString>) {
        self.preload = value;
    }

    #[wasm_bindgen(getter, js_name = "safeDialogsMessage")]
    pub fn safe_dialogs_message(&self) -> Option<JsString> {
        self.safe_dialogs_message.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_safe_dialogs_message(&mut self, value: Option<JsString>) {
        self.safe_dialogs_message = value;
    }

    #[wasm_bindgen(getter, js_name = "safeDialogs")]
    pub fn safe_dialogs(&self) -> Option<bool> {
        self.safe_dialogs
    }

    #[wasm_bindgen(setter)]
    pub fn set_safe_dialogs(&mut self, value: Option<bool>) {
        self.safe_dialogs = value;
    }

    #[wasm_bindgen(getter)]
    pub fn sandbox(&self) -> Option<bool> {
        self.sandbox
    }

    #[wasm_bindgen(setter)]
    pub fn set_sandbox(&mut self, value: Option<bool>) {
        self.sandbox = value;
    }

    #[wasm_bindgen(getter, js_name = "scrollBounce")]
    pub fn scroll_bounce(&self) -> Option<bool> {
        self.scroll_bounce
    }

    #[wasm_bindgen(setter)]
    pub fn set_scroll_bounce(&mut self, value: Option<bool>) {
        self.scroll_bounce = value;
    }

    #[wasm_bindgen(getter)]
    pub fn session(&self) -> Option<Session> {
        self.session.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_session(&mut self, value: Option<Session>) {
        self.session = value;
    }

    #[wasm_bindgen(getter)]
    pub fn spellcheck(&self) -> Option<bool> {
        self.spellcheck
    }

    #[wasm_bindgen(setter)]
    pub fn set_spellcheck(&mut self, value: Option<bool>) {
        self.spellcheck = value;
    }

    #[wasm_bindgen(getter, js_name = "textAreasAreResizable")]
    pub fn text_areas_are_resizable(&self) -> Option<bool> {
        self.text_areas_are_resizable
    }

    #[wasm_bindgen(setter)]
    pub fn set_text_areas_are_resizable(&mut self, value: Option<bool>) {
        self.text_areas_are_resizable = value;
    }

    #[wasm_bindgen(getter, js_name = "webSecurity")]
    pub fn web_security(&self) -> Option<bool> {
        self.web_security
    }

    #[wasm_bindgen(setter)]
    pub fn set_web_security(&mut self, value: Option<bool>) {
        self.web_security = value;
    }

    #[wasm_bindgen(getter)]
    pub fn webgl(&self) -> Option<bool> {
        self.webgl
    }

    #[wasm_bindgen(setter)]
    pub fn set_webgl(&mut self, value: Option<bool>) {
        self.webgl = value;
    }

    #[wasm_bindgen(getter, js_name = "webviewTag")]
    pub fn webview_tag(&self) -> Option<bool> {
        self.webview_tag
    }

    #[wasm_bindgen(setter)]
    pub fn set_webview_tag(&mut self, value: Option<bool>) {
        self.webview_tag = value;
    }

    #[wasm_bindgen(getter, js_name = "zoomFactor")]
    pub fn zoom_factor(&self) -> Option<f32> {
        self.zoom_factor
    }

    #[wasm_bindgen(setter)]
    pub fn set_zoom_factor(&mut self, value: Option<f32>) {
        self.zoom_factor = value;
    }
}
