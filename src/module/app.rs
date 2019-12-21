use crate::{
    class::{CommandLine, Dock, Menu},
    interface::{
        GetFileIconOptions,
        GetLoginItemSettings,
        GetLoginItemSettingsOptions,
        GpuFeatureStatus,
        ImportCertificateOptions,
        JumpListSettings,
        MoveToApplicationsFolderOptions,
        RelaunchOptions,
        SetAboutPanelOptions,
        SetLoginItemSettings,
    },
};
use js_sys::{Function, JsString, Promise};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    pub type App;

    pub static app: App;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method)]
    pub fn quit(this: &App);

    #[wasm_bindgen(method)]
    pub fn exit(this: &App, exit_code: Option<i32>);

    #[wasm_bindgen(method)]
    pub fn relaunch(this: &App, options: Option<RelaunchOptions>);

    #[wasm_bindgen(method, js_name = "isReady")]
    pub fn is_ready(this: &App) -> bool;

    #[must_use]
    #[wasm_bindgen(method, js_name = "whenReady")]
    pub fn when_ready(this: &App) -> Promise;

    #[wasm_bindgen(method)]
    pub fn focus(this: &App);

    #[wasm_bindgen(method)]
    pub fn hide(this: &App);

    #[wasm_bindgen(method)]
    pub fn show(this: &App);

    #[wasm_bindgen(method, js_name = "setAppLogsPath")]
    pub fn set_app_logs_path(this: &App, path: Option<&str>);

    #[wasm_bindgen(method, js_name = "getAppPath")]
    pub fn get_app_path(this: &App) -> JsString;

    #[wasm_bindgen(method, catch, js_name = "getPath")]
    pub fn get_path(this: &App) -> Result<JsString, JsValue>;

    #[must_use]
    #[wasm_bindgen(method, js_name = "getFileIcon")]
    pub fn get_file_icon(this: &App, path: &str, options: Option<GetFileIconOptions>) -> Promise;

    #[wasm_bindgen(method, catch, js_name = "setPath")]
    pub fn set_path(this: &App, name: &str, path: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name = "getVersion")]
    pub fn get_version(this: &App) -> JsString;

    #[wasm_bindgen(method, js_name = "getLocale")]
    pub fn get_locale(this: &App);

    #[wasm_bindgen(method, js_name = "getLocaleCountryCode")]
    pub fn get_locale_country_code(this: &App);

    #[wasm_bindgen(method, js_name = "addRecentDocument")]
    pub fn add_recent_document(this: &App, path: &str);

    #[wasm_bindgen(method, js_name = "clearRecentDocuments")]
    pub fn clear_recent_documents(this: &App);

    #[wasm_bindgen(method, js_name = "setAsDefaultProtocolClient")]
    pub fn set_as_default_protocol_client(this: &App, protocol: &str, path: Option<&str>, args: &str) -> bool;

    #[wasm_bindgen(method, js_name = "removeAsDefaultProtocolClient")]
    pub fn remove_as_default_protocol_client(this: &App, protocol: &str, path: Option<&str>, args: &str) -> bool;

    #[wasm_bindgen(method, js_name = "isDefaultProtocolClient")]
    pub fn is_default_protocol_client(this: &App, protocol: &str, path: Option<&str>, args: &str) -> bool;

    #[wasm_bindgen(method, js_name = "setUserTasks")]
    pub fn set_user_tasks(this: &App, task: Box<[JsValue]>) -> bool;

    #[wasm_bindgen(method, js_name = "getJumpListSettings")]
    pub fn get_jump_list_settings(this: &App) -> JumpListSettings;

    #[wasm_bindgen(method, js_name = "setJumpList")]
    pub fn set_jump_list(this: &App, categories: Option<Box<[JsValue]>>) -> JsString;

    #[wasm_bindgen(method, js_name = "requestSingleInstanceLock")]
    pub fn request_single_instance_lock(this: &App) -> bool;

    #[wasm_bindgen(method, js_name = "hasSingleInstanceLock")]
    pub fn has_single_instance_lock(this: &App) -> bool;

    #[wasm_bindgen(method, js_name = "releaseSingleInstanceLock")]
    pub fn release_single_instance_lock(this: &App);

    #[wasm_bindgen(method, js_name = "setUserActivity")]
    pub fn set_user_activity(this: &App, user_info: &JsValue, webpage_url: Option<&str>);

    #[wasm_bindgen(method, js_name = "getCurrentActivityType")]
    pub fn get_current_activity_type(this: &App) -> JsString;

    #[wasm_bindgen(method, js_name = "invalidateCurrentActivity")]
    pub fn invalidate_current_activity(this: &App);

    #[wasm_bindgen(method, js_name = "resignCurrentActivity")]
    pub fn resign_current_activity(this: &App);

    #[wasm_bindgen(method, js_name = "updateCurrentActivity")]
    pub fn update_current_activity(this: &App, kind: &str, user_info: &JsValue);

    #[wasm_bindgen(method, js_name = "setAppUserModelId")]
    pub fn set_app_user_model_id(this: &App, id: &str);

    #[wasm_bindgen(method, js_name = "importCertificate")]
    pub fn import_certificate(this: &App, options: ImportCertificateOptions, callback: &Function);

    #[wasm_bindgen(method, js_name = "disableHardwareAcceleration")]
    pub fn disable_hardware_acceleration(this: &App);

    #[wasm_bindgen(method, js_name = "disableDomainBlockingFor3DAPIs")]
    pub fn disable_domain_blocking_for_3d_apis(this: &App);

    #[wasm_bindgen(method, js_name = "getAppMetrics")]
    pub fn get_app_metrics(this: &App) -> Box<[JsValue]>;

    #[wasm_bindgen(method, js_name = "getGPUFeatureStatus")]
    pub fn get_gpu_feature_status(this: &App) -> GpuFeatureStatus;

    #[must_use]
    #[wasm_bindgen(method, js_name = "getGPUInfo")]
    pub fn get_gpu_info(this: &App, info_type: &str) -> Promise;

    #[wasm_bindgen(method, js_name = "isUnityRunning")]
    pub fn is_unity_running(this: &App) -> bool;

    #[wasm_bindgen(method, js_name = "getLoginItemSettings")]
    pub fn get_login_item_settings(this: &App, options: Option<GetLoginItemSettingsOptions>) -> GetLoginItemSettings;

    #[wasm_bindgen(method, js_name = "setLoginItemSettings")]
    pub fn set_login_item_settings(this: &App, settings: Option<SetLoginItemSettings>);

    #[wasm_bindgen(method, js_name = "setAboutPanelOptions")]
    pub fn set_about_panel_options(this: &App, options: SetAboutPanelOptions);

    #[wasm_bindgen(method, js_name = "isEmojiPanelSupported")]
    pub fn is_emoji_panel_supported(this: &App);

    #[wasm_bindgen(method, js_name = "showEmojiPanel")]
    pub fn show_emoji_panel(this: &App);

    #[wasm_bindgen(method, js_name = "startAccessingSecurityScopedResource")]
    pub fn start_accessing_security_scoped_resource(this: &App, bookmark_data: &str) -> Function;

    #[wasm_bindgen(method, js_name = "enableSandbox")]
    pub fn enable_sandbox(this: &App);

    #[wasm_bindgen(method, js_name = "isInApplicationsFolder")]
    pub fn is_in_applications_folder(this: &App) -> bool;

    #[wasm_bindgen(method, js_name = "moveToApplicationsFolder")]
    pub fn move_to_applications_folder(this: &App, options: Option<MoveToApplicationsFolderOptions>);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "accessibilitySupportEnabled")]
    pub fn accessibility_support_enabled(this: &App) -> bool;

    #[wasm_bindgen(method, setter, js_name = "accessibilitySupportEnabled")]
    pub fn set_accessibility_support_enabled(this: &App, value: bool);

    #[wasm_bindgen(method, js_name = "showAboutPanel")]
    pub fn show_about_panel(this: &App);

    #[wasm_bindgen(method, getter, js_name = "applicationMenu")]
    pub fn application_menu(this: &App) -> Option<Menu>;

    #[wasm_bindgen(method, setter, js_name = "applicationMenu")]
    pub fn set_application_menu(this: &App, value: Option<Menu>);

    #[wasm_bindgen(method, getter, js_name = "badgeCount")]
    pub fn badge_count(this: &App);

    #[wasm_bindgen(method, setter, js_name = "badgeCount")]
    pub fn set_badge_count(this: &App, value: usize);

    #[wasm_bindgen(method, getter, js_name = "commandLine")] // readonly
    pub fn command_line(this: &App) -> CommandLine;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn dock(this: &App) -> Dock;

    #[wasm_bindgen(method, getter, js_name = "isPackaged")] // readonly
    pub fn is_packaged(this: &App) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &App) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "setName")]
    pub fn set_name(this: &App, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "userAgentFallback")]
    pub fn user_agent_fallback(this: &App) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "setUserAgentFallback")]
    pub fn set_user_agent_fallback(this: &App, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "allowRendererProcessReuse")]
    pub fn allow_renderer_process_reuse(this: &App) -> bool;

    #[wasm_bindgen(method, setter, js_name = "setAllowRendererProcessReuse")]
    pub fn set_allow_renderer_process_reuse(this: &App, value: bool);
}
