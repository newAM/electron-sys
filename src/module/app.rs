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
use js_sys::{Array, Function, JsString, Promise};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    /// The electron app interface type.
    #[wasm_bindgen(extends = EventEmitter)]
    pub type App;

    /// The electron app.
    pub static app: App;

    /// Try to close all windows.
    #[wasm_bindgen(method)]
    pub fn quit(this: &App);

    /// Exits immediately with `exit_code`. `exit_code` defaults to 0.
    #[wasm_bindgen(method)]
    pub fn exit(this: &App, exit_code: Option<i32>);

    /// Relaunches the app when current instance exits.
    #[wasm_bindgen(method)]
    pub fn relaunch(this: &App, options: Option<RelaunchOptions>);

    /// Returns `true` if Electron has finished initializing, `false` otherwise.
    #[wasm_bindgen(method, js_name = "isReady")]
    pub fn is_ready(this: &App) -> bool;

    /// May be used as a convenient alternative to checking `app.is_ready()` or subscribing to the
    /// `ready` event if the app is not ready yet.
    #[must_use]
    #[wasm_bindgen(method, js_name = "whenReady")]
    pub fn when_ready(this: &App) -> Promise;

    /// On Linux, focuses on the first visible window. On macOS, makes the application the active
    /// app. On Windows, focuses on the application's first window.
    #[wasm_bindgen(method)]
    pub fn focus(this: &App);

    /// Hides all application windows without minimizing them.
    #[wasm_bindgen(method)]
    pub fn hide(this: &App);

    /// Shows application windows after they were hidden. Does not automatically focus them.
    #[wasm_bindgen(method)]
    pub fn show(this: &App);

    /// Sets or creates a directory your app's logs which can then be manipulated with
    /// `app.get_path()` or `app.set_path(path_name, new_path)`.
    #[wasm_bindgen(method, js_name = "setAppLogsPath")]
    pub fn set_app_logs_path(this: &App, path: Option<JsString>);

    /// Returns the current application directory.
    #[wasm_bindgen(method, js_name = "getAppPath")]
    pub fn get_app_path(this: &App) -> JsString;

    /// Returns the path to a special directory or file associated with name. On failure, an `Error`
    /// is thrown.
    #[wasm_bindgen(method, catch, js_name = "getPath")]
    pub fn get_path(this: &App) -> Result<JsString, JsValue>;

    /// Returns a promise with the app's icon, which is a `NativeImage`.
    #[must_use]
    #[wasm_bindgen(method, js_name = "getFileIcon")]
    pub fn get_file_icon(this: &App, path: &JsString, options: Option<GetFileIconOptions>) -> Promise;

    /// Overrides the `path` to a special directory or file associated with `name`. If the path
    /// specifies a directory that does not exist, an `Error` is thrown.
    #[wasm_bindgen(method, catch, js_name = "setPath")]
    pub fn set_path(this: &App, name: &JsString, path: &JsString) -> Result<(), JsValue>;

    /// Returns the version of the loaded application.
    #[wasm_bindgen(method, js_name = "getVersion")]
    pub fn get_version(this: &App) -> JsString;

    /// Returns the current application locale.
    #[wasm_bindgen(method, js_name = "getLocale")]
    pub fn get_locale(this: &App);

    /// Returns the operating system's locale two-letter ISO 3166 country code.
    #[wasm_bindgen(method, js_name = "getLocaleCountryCode")]
    pub fn get_locale_country_code(this: &App);

    /// Adds path to the recent documents list.
    #[wasm_bindgen(method, js_name = "addRecentDocument")]
    pub fn add_recent_document(this: &App, path: JsString);

    /// Clears the recent documents list.
    #[wasm_bindgen(method, js_name = "clearRecentDocuments")]
    pub fn clear_recent_documents(this: &App);

    /// Returns whether the call succeeded.
    #[wasm_bindgen(method, js_name = "setAsDefaultProtocolClient")]
    pub fn set_as_default_protocol_client(
        this: &App,
        protocol: JsString,
        path: Option<JsString>,
        args: JsString,
    ) -> bool;

    /// Returns whether the call succeeded.
    #[wasm_bindgen(method, js_name = "removeAsDefaultProtocolClient")]
    pub fn remove_as_default_protocol_client(
        this: &App,
        protocol: JsString,
        path: Option<JsString>,
        args: JsString,
    ) -> bool;

    /// This method checks if the current executable is the default handler for a protocol (aka URI
    /// scheme). If so, it will return true. Otherwise, it will return false.
    #[wasm_bindgen(method, js_name = "isDefaultProtocolClient")]
    pub fn is_default_protocol_client(this: &App, protocol: JsString, path: Option<JsString>, args: JsString) -> bool;

    /// Adds `tasks` to the `Tasks` category of the Jump List on Windows.
    #[wasm_bindgen(method, js_name = "setUserTasks")]
    pub fn set_user_tasks(this: &App, task: Array) -> bool;

    #[wasm_bindgen(method, js_name = "getJumpListSettings")]
    pub fn get_jump_list_settings(this: &App) -> JumpListSettings;

    /// Sets or removes a custom Jump List for the application
    #[wasm_bindgen(method, js_name = "setJumpList")]
    pub fn set_jump_list(this: &App, categories: Option<Array>) -> JsString;

    /// The return value of this method indicates whether or not this instance of your application
    /// successfully obtained the lock.
    #[wasm_bindgen(method, js_name = "requestSingleInstanceLock")]
    pub fn request_single_instance_lock(this: &App) -> bool;

    /// This method returns whether or not this instance of your app is currently holding the single
    /// instance lock.
    #[wasm_bindgen(method, js_name = "hasSingleInstanceLock")]
    pub fn has_single_instance_lock(this: &App) -> bool;

    /// Releases all locks that were created by `requestSingleInstanceLock`.
    #[wasm_bindgen(method, js_name = "releaseSingleInstanceLock")]
    pub fn release_single_instance_lock(this: &App);

    /// Creates an `NSUserActivity` and sets it as the current activity.
    #[wasm_bindgen(method, js_name = "setUserActivity")]
    pub fn set_user_activity(this: &App, user_info: &JsValue, webpage_url: Option<JsString>);

    /// Returns the type of the currently running activity.
    #[wasm_bindgen(method, js_name = "getCurrentActivityType")]
    pub fn get_current_activity_type(this: &App) -> JsString;

    /// Invalidates the current Handoff user activity.
    #[wasm_bindgen(method, js_name = "invalidateCurrentActivity")]
    pub fn invalidate_current_activity(this: &App);

    /// Marks the current Handoff user activity as inactive without invalidating it.
    #[wasm_bindgen(method, js_name = "resignCurrentActivity")]
    pub fn resign_current_activity(this: &App);

    /// Updates the current activity if its type matches `type`, merging the entries from
    /// `user_info` into its current `user_info` dictionary.
    #[wasm_bindgen(method, js_name = "updateCurrentActivity")]
    pub fn update_current_activity(this: &App, kind: &JsString, user_info: &JsValue);

    /// Changes the Application User Model ID to `id`.
    #[wasm_bindgen(method, js_name = "setAppUserModelId")]
    pub fn set_app_user_model_id(this: &App, id: JsString);

    /// Imports the certificate in pkcs12 format into the platform certificate store.
    #[wasm_bindgen(method, js_name = "importCertificate")]
    pub fn import_certificate(this: &App, options: ImportCertificateOptions, callback: &Function);

    /// Disables hardware acceleration for current app.
    #[wasm_bindgen(method, js_name = "disableHardwareAcceleration")]
    pub fn disable_hardware_acceleration(this: &App);

    /// By default, Chromium disables 3D APIs (e.g. WebGL) until restart on a per domain basis if
    /// the GPU processes crashes too frequently. This function disables that behaviour.
    #[wasm_bindgen(method, js_name = "disableDomainBlockingFor3DAPIs")]
    pub fn disable_domain_blocking_for_3d_apis(this: &App);

    /// Returns an array of `ProcessMetric` objects that correspond to memory and CPU
    /// usage statistics of all the processes associated with the app.
    #[wasm_bindgen(method, js_name = "getAppMetrics")]
    pub fn get_app_metrics(this: &App) -> Array;

    /// Returns the graphics feature status from chrome://gpu/.
    #[wasm_bindgen(method, js_name = "getGPUFeatureStatus")]
    pub fn get_gpu_feature_status(this: &App) -> GpuFeatureStatus;

    #[must_use]
    #[wasm_bindgen(method, js_name = "getGPUInfo")]
    pub fn get_gpu_info(this: &App, info_type: &JsString) -> Promise;

    /// Returns whether the current desktop environment is Unity launcher.
    #[wasm_bindgen(method, js_name = "isUnityRunning")]
    pub fn is_unity_running(this: &App) -> bool;

    #[wasm_bindgen(method, js_name = "getLoginItemSettings")]
    pub fn get_login_item_settings(this: &App, options: Option<GetLoginItemSettingsOptions>) -> GetLoginItemSettings;

    /// Set the app's login item settings.
    #[wasm_bindgen(method, js_name = "setLoginItemSettings")]
    pub fn set_login_item_settings(this: &App, settings: Option<SetLoginItemSettings>);

    /// Set the about panel options.
    #[wasm_bindgen(method, js_name = "setAboutPanelOptions")]
    pub fn set_about_panel_options(this: &App, options: SetAboutPanelOptions);

    /// Returns whether or not the current OS version allows for native emoji pickers.
    #[wasm_bindgen(method, js_name = "isEmojiPanelSupported")]
    pub fn is_emoji_panel_supported(this: &App);

    /// Show the platform's native emoji picker.
    #[wasm_bindgen(method, js_name = "showEmojiPanel")]
    pub fn show_emoji_panel(this: &App);

    /// Returns a function which must be called once you have finished accessing the scoped file.
    #[wasm_bindgen(method, js_name = "startAccessingSecurityScopedResource")]
    pub fn start_accessing_security_scoped_resource(this: &App, bookmark_data: &JsString) -> Function;

    /// Enables full sandbox mode on the app.
    #[wasm_bindgen(method, js_name = "enableSandbox")]
    pub fn enable_sandbox(this: &App);

    /// Returns whether the application is currently running from the systems Application folder.
    #[wasm_bindgen(method, js_name = "isInApplicationsFolder")]
    pub fn is_in_applications_folder(this: &App) -> bool;

    /// Returns whether the move was successful. Please note that if the move is successful, your
    /// application will quit and relaunch.
    #[wasm_bindgen(method, js_name = "moveToApplicationsFolder")]
    pub fn move_to_applications_folder(this: &App, options: Option<MoveToApplicationsFolderOptions>);

    //************//
    // Properties //
    //************//

    /// A `Boolean` property that's `true` if Chrome's accessibility support is enabled, `false`
    /// otherwise.
    #[wasm_bindgen(method, getter, js_name = "accessibilitySupportEnabled")]
    pub fn accessibility_support_enabled(this: &App) -> bool;

    #[wasm_bindgen(method, setter, js_name = "accessibilitySupportEnabled")]
    pub fn set_accessibility_support_enabled(this: &App, value: bool);

    #[wasm_bindgen(method, js_name = "showAboutPanel")]
    pub fn show_about_panel(this: &App);

    /// A property that returns `Menu` if one has been set and `null` otherwise.
    #[wasm_bindgen(method, getter, js_name = "applicationMenu")]
    pub fn application_menu(this: &App) -> Option<Menu>;

    #[wasm_bindgen(method, setter, js_name = "applicationMenu")]
    pub fn set_application_menu(this: &App, value: Option<Menu>);

    /// An `Integer` property that returns the badge count for current app. Setting the count to `0`
    /// will hide the badge.
    #[wasm_bindgen(method, getter, js_name = "badgeCount")]
    pub fn badge_count(this: &App);

    #[wasm_bindgen(method, setter, js_name = "badgeCount")]
    pub fn set_badge_count(this: &App, value: usize);

    /// A `CommandLine` interface that allows you to read and manipulate the command line arguments
    /// that Chromium uses.
    #[wasm_bindgen(method, getter, js_name = "commandLine")]
    pub fn command_line(this: &App) -> CommandLine;

    /// A `Dock` interface that allows you to perform actions on your app icon in the user's dock on
    /// macOS.
    #[wasm_bindgen(method, getter)]
    pub fn dock(this: &App) -> Dock;

    /// A `Boolean` property that returns `true` if the app is packaged, `false` otherwise.
    #[wasm_bindgen(method, getter, js_name = "isPackaged")]
    pub fn is_packaged(this: &App) -> bool;

    /// A `String` property that indicates the current application's name, which is the name in the
    /// application's `package.json` file.
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &App) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "setName")]
    pub fn set_name(this: &App, value: JsString);

    /// A `String` which is the user agent string Electron will use as a global fallback.
    #[wasm_bindgen(method, getter, js_name = "userAgentFallback")]
    pub fn user_agent_fallback(this: &App) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "setUserAgentFallback")]
    pub fn set_user_agent_fallback(this: &App, value: JsString);

    /// A `Boolean` which when `true` disables the overrides that Electron has in place to ensure
    /// renderer processes are restarted on every navigation. The current default value for this
    /// property is `false`.
    #[wasm_bindgen(method, getter, js_name = "allowRendererProcessReuse")]
    pub fn allow_renderer_process_reuse(this: &App) -> bool;

    #[wasm_bindgen(method, setter, js_name = "setAllowRendererProcessReuse")]
    pub fn set_allow_renderer_process_reuse(this: &App, value: bool);
}