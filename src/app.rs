use js_sys::{Array, JsString, Promise};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub type CommandLine;
}

#[wasm_bindgen]
extern {
    pub type Dock;
}

#[wasm_bindgen]
extern {
    pub type Menu;
}

#[wasm_bindgen]
extern {
    pub type NativeImage;
}

#[wasm_bindgen]
extern {
    pub type Task;
}

#[wasm_bindgen]
pub struct GetFileIconOptions {
    size: JsString,
}

#[wasm_bindgen]
impl GetFileIconOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(size: JsString) -> GetFileIconOptions {
        GetFileIconOptions { size }
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> JsString {
        self.size.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_size(&mut self, size: JsString) {
        self.size = size;
    }
}

#[wasm_bindgen]
pub struct JumpListSettings {
    min_items: usize,
    removed_items: Array,
}

#[wasm_bindgen]
impl JumpListSettings {
    #[wasm_bindgen(constructor)]
    pub fn new(min_items: usize, removed_items: Array) -> JumpListSettings {
        JumpListSettings {
            min_items,
            removed_items,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn min_items(&self) -> usize {
        self.min_items
    }

    #[wasm_bindgen(setter)]
    pub fn set_min_items(&mut self, min_items: usize) {
        self.min_items = min_items;
    }

    #[wasm_bindgen(getter)]
    pub fn removed_items(&self) -> Array {
        self.removed_items.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_removed_items(&mut self, removed_items: Array) {
        self.removed_items = removed_items;
    }
}

#[wasm_bindgen]
pub struct RelaunchOptions {
    args: JsString,
    exec_path: Option<JsString>,
}

#[wasm_bindgen]
impl RelaunchOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(args: JsString, exec_path: Option<JsString>) -> RelaunchOptions {
        RelaunchOptions { args, exec_path }
    }

    #[wasm_bindgen(getter)]
    pub fn args(&self) -> JsString {
        self.args.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_args(&mut self, args: JsString) {
        self.args = args;
    }

    #[wasm_bindgen(getter)]
    pub fn exec_path(&self) -> JsString {
        self.args.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_exec_path(&mut self, exec_path: Option<JsString>) {
        self.exec_path = exec_path;
    }
}

#[wasm_bindgen(module = "electron")]
extern {
    /// The electron app interface type.
    #[wasm_bindgen(extends = EventEmitter)]
    pub type App;

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
    // #[cfg(macos)]
    #[wasm_bindgen(method)]
    pub fn hide(this: &App);

    /// Shows application windows after they were hidden. Does not automatically focus them.
    // #[cfg(macos)]
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
    // #[cfg(any(macos, windows))]
    #[wasm_bindgen(method, js_name = "addRecentDocument")]
    pub fn add_recent_document(this: &App, path: JsString);

    /// Clears the recent documents list.
    // #[cfg(any(macos, windows))]
    #[wasm_bindgen(method, js_name = "clearRecentDocuments")]
    pub fn clear_recent_documents(this: &App);

    /// Returns whether the call succeeded.
    #[wasm_bindgen(method, js_name = "setAsDefaultProtocolClient")]
    pub fn set_as_default_protocol_client(
        this: &App,
        protocol: JsString,
        /* #[cfg(windows)] path: Option<JsString>,
         * #[cfg(windows)] args: JsString, */
    ) -> bool;

    /// Returns whether the call succeeded.
    // #[cfg(any(macos, windows))]
    #[wasm_bindgen(method, js_name = "removeAsDefaultProtocolClient")]
    pub fn remove_as_default_protocol_client(
        this: &App,
        protocol: JsString,
        /* #[cfg(windows)] path: Option<JsString>,
         * #[cfg(windows)] args: JsString, */
    ) -> bool;

    /// This method checks if the current executable is the default handler for a protocol (aka URI
    /// scheme). If so, it will return true. Otherwise, it will return false.
    #[wasm_bindgen(method, js_name = "isDefaultProtocolClient")]
    pub fn is_default_protocol_client(
        this: &App,
        protocol: JsString,
        /* #[cfg(windows)] path: Option<JsString>,
         * #[cfg(windows)] args: JsString, */
    ) -> bool;

    /// Adds `tasks` to the `Tasks` category of the Jump List on Windows.
    // #[cfg(windows)]
    #[wasm_bindgen(method, js_name = "setUserTasks")]
    pub fn set_user_tasks(this: &App, task: Array) -> bool;

    // #[cfg(windows)]
    #[wasm_bindgen(method, js_name = "getJumpListSettings")]
    pub fn get_jump_list_settings(this: &App) -> JumpListSettings;

    /// Sets or removes a custom Jump List for the application
    // #[cfg(windows)]
    #[wasm_bindgen(method, js_name = "setJumpList")]
    pub fn set_jump_list(this: &App, categories: Option<Array>) -> JsString;

    // #[cfg(any(macos, windows))]
    #[wasm_bindgen(method, getter, js_name = "accessibilitySupportEnabled")]
    pub fn is_accessibility_support_enabled(this: &App) -> bool;

    // #[cfg(any(macos, windows))]
    #[wasm_bindgen(method, setter, js_name = "accessibilitySupportEnabled")]
    pub fn set_accessibility_support_enabled(this: &App, enabled: bool);

    // #[cfg(any(macos, linux))]
    #[wasm_bindgen(method, js_name = "showAboutPanel")]
    pub fn show_about_panel(this: &App);

    #[wasm_bindgen(method, getter, js_name = "applicationMenu")]
    pub fn application_menu(this: &App) -> Option<Menu>;

    #[wasm_bindgen(method, setter, js_name = "applicationMenu")]
    pub fn set_application_menu(this: &App, menu: Option<Menu>);

    // #[cfg(any(linux, macos))]
    #[wasm_bindgen(method, getter, js_name = "badgeCount")]
    pub fn badge_count(this: &App);

    // #[cfg(any(linux, macos))]
    #[wasm_bindgen(method, setter, js_name = "badgeCount")]
    pub fn set_badge_count(this: &App, count: usize);

    #[wasm_bindgen(method, getter, js_name = "commandLine")]
    pub fn command_line(this: &App) -> CommandLine;

    // #[cfg(macos)]
    #[wasm_bindgen(method, getter)]
    pub fn dock(this: &App) -> Dock;

    #[wasm_bindgen(method, getter, js_name = "isPackaged")]
    pub fn is_packaged(this: &App) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &App) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "setName")]
    pub fn set_name(this: &App, name: &JsString);

    #[wasm_bindgen(method, getter, js_name = "userAgentFallback")]
    pub fn user_agent_fallback(this: &App) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "setUserAgentFallback")]
    pub fn set_user_agent_fallback(this: &App, user_agent_fallback: &JsString);

    #[wasm_bindgen(method, getter, js_name = "allowRendererProcessReuse")]
    pub fn allow_renderer_process_reuse(this: &App) -> bool;

    #[wasm_bindgen(method, setter, js_name = "setAllowRendererProcessReuse")]
    pub fn set_allow_renderer_process_reuse(this: &App, allow_reuse: bool);

    /// The electron app.
    pub static app: App;
}
