use crate::{
    class::{Debugger, Session},
    interface::{
        EnableDeviceEmulationOptions,
        FindInPageOptions,
        InputEvent,
        InsertCssOptions,
        Item,
        LoadFileOptions,
        LoadUrlOptions,
        OpenDevToolsOptions,
        PrintToPdfOptions,
        Rectangle,
        WebContentsPrintOptions,
    },
};
use js_sys::{Function, JsString, Promise};
use node_sys::events::EventEmitter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    /// Docs: http://electronjs.org/docs/api/web-contents
    pub type WebContents;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = WebContents)]
    pub fn from_id(id: usize) -> WebContents;

    #[wasm_bindgen(static_method_of = WebContents)]
    pub fn get_all_web_contents() -> Box<[JsValue]>;

    #[wasm_bindgen(static_method_of = WebContents)]
    pub fn get_focused_web_contents() -> WebContents;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "addWorkSpace")]
    pub fn add_work_space(this: &WebContents, path: &str);

    #[wasm_bindgen(method, js_name = "beginFrameSubscription")]
    pub fn begin_frame_subscription(this: &WebContents, only_dirty: bool, callback: &Function);

    #[wasm_bindgen(method, js_name = "canGoBack")]
    pub fn can_go_back(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "canGoForward")]
    pub fn can_go_forward(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "canGoForward")]
    pub fn can_go_to_offset(this: &WebContents, offset: u32) -> bool;

    #[must_use]
    #[wasm_bindgen(method, js_name = "capturePage")]
    pub fn capture_page(this: &WebContents, rectangle: Option<Rectangle>) -> Promise;

    #[wasm_bindgen(method, js_name = "clearHistory")]
    pub fn clear_history(this: &WebContents);

    #[wasm_bindgen(method, js_name = "closeDevTools")]
    pub fn close_dev_tools(this: &WebContents);

    #[wasm_bindgen(method)]
    pub fn copy(this: &WebContents);

    #[wasm_bindgen(method, js_name = "copyImageAt")]
    pub fn copy_image_at(this: &WebContents, x: usize, y: usize);

    #[wasm_bindgen(method)]
    pub fn cut(this: &WebContents);

    #[wasm_bindgen(method)]
    pub fn delete(this: &WebContents);

    #[wasm_bindgen(method, js_name = "disableDeviceEmulation")]
    pub fn disable_device_emulation(this: &WebContents);

    #[wasm_bindgen(method, js_name = "downloadURL")]
    pub fn download_url(this: &WebContents);

    #[wasm_bindgen(method, js_name = "enableDeviceEmulation")]
    pub fn enable_device_emulation(this: &WebContents, options: EnableDeviceEmulationOptions);

    #[wasm_bindgen(method, js_name = "endFrameSubscription")]
    pub fn end_frame_subscription(this: &WebContents);

    #[must_use]
    #[wasm_bindgen(method, js_name = "executeJavaScript")]
    pub fn execute_java_script(this: &WebContents, code: &str, user_gesture: Option<bool>) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "executeJavaScriptInIsolatedWorld")]
    pub fn execute_java_script_in_isolated_world(
        this: &WebContents,
        world_id: u32,
        scripts: Box<[JsValue]>,
        user_gesture: Option<bool>,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = "findInPage")]
    pub fn find_in_page(this: &WebContents, text: &str, options: Option<FindInPageOptions>) -> usize;

    #[wasm_bindgen(method)]
    pub fn focus(this: &WebContents);

    #[wasm_bindgen(method, js_name = "getAllSharedWorkers")]
    pub fn get_all_shared_workers(this: &WebContents) -> Box<[JsValue]>;

    #[wasm_bindgen(method, js_name = "getOSProcessId")]
    pub fn get_os_process_id(this: &WebContents) -> u32;

    #[wasm_bindgen(method, js_name = "getPrinters")]
    pub fn get_printers(this: &WebContents);

    #[wasm_bindgen(method, js_name = "getProcessId")]
    pub fn get_process_id(this: &WebContents) -> u32;

    #[wasm_bindgen(method, js_name = "getTitle")]
    pub fn get_title(this: &WebContents) -> JsString;

    #[wasm_bindgen(method, js_name = "getType")]
    pub fn get_type(this: &WebContents) -> JsString;

    #[wasm_bindgen(method, js_name = "getURL")]
    pub fn get_url(this: &WebContents) -> JsString;

    #[wasm_bindgen(method, js_name = "getWebRTCIPHandlingPolicy")]
    pub fn get_web_rtc_ip_handling_policy(this: &WebContents) -> JsString;

    #[wasm_bindgen(method, js_name = "goBack")]
    pub fn go_back(this: &WebContents);

    #[wasm_bindgen(method, js_name = "goForward")]
    pub fn go_forward(this: &WebContents);

    #[wasm_bindgen(method, js_name = "goToIndex")]
    pub fn go_to_index(this: &WebContents, index: usize);

    #[wasm_bindgen(method, js_name = "goToOffset")]
    pub fn go_to_offset(this: &WebContents, index: usize);

    #[must_use]
    #[wasm_bindgen(method, js_name = "insertCSS")]
    pub fn insert_css(this: &WebContents, css: &str, options: Option<InsertCssOptions>) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "insertText")]
    pub fn insert_text(this: &WebContents, text: &str) -> Promise;

    #[wasm_bindgen(method, js_name = "inspectElement")]
    pub fn inspect_element(this: &WebContents, x: usize, y: usize);

    #[wasm_bindgen(method, js_name = "inspectServiceWorker")]
    pub fn inspect_service_worker(this: &WebContents);

    #[wasm_bindgen(method, js_name = "inspectSharedWorker")]
    pub fn inspect_shared_worker(this: &WebContents);

    #[wasm_bindgen(method, js_name = "inspectSharedWorkerById")]
    pub fn inspect_shared_worker_by_id(this: &WebContents, worker_id: &str);

    #[wasm_bindgen(method)]
    pub fn invalidate(this: &WebContents);

    #[wasm_bindgen(method, js_name = "isCrashed")]
    pub fn is_crashed(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isCurrentlyAudible")]
    pub fn is_currently_audible(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isDestroyed")]
    pub fn is_destroyed(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isDevToolsFocused")]
    pub fn is_dev_tools_focused(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isDevToolsOpened")]
    pub fn is_dev_tools_opened(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isFocused")]
    pub fn is_focused(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isLoading")]
    pub fn is_loading(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isLoadingMainFrame")]
    pub fn is_loading_main_frame(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isOffscreen")]
    pub fn is_offscreen(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isPainting")]
    pub fn is_painting(this: &WebContents) -> bool;

    #[wasm_bindgen(method, js_name = "isWaitingForResponse")]
    pub fn is_waiting_for_response(this: &WebContents) -> bool;

    #[must_use]
    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &WebContents, file_path: &str, options: Option<LoadFileOptions>) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "loadURL")]
    pub fn load_url(this: &WebContents, url: &str, options: Option<LoadUrlOptions>) -> Promise;

    #[wasm_bindgen(method, js_name = "openDevTools")]
    pub fn open_dev_tools(this: &WebContents, options: Option<OpenDevToolsOptions>);

    #[wasm_bindgen(method)]
    pub fn paste(this: &WebContents);

    #[wasm_bindgen(method, js_name = "pasteAndMatchStyle")]
    pub fn paste_and_match_style(this: &WebContents);

    #[wasm_bindgen(method)]
    pub fn print(this: &WebContents, options: Option<WebContentsPrintOptions>, callback: Option<&Function>);

    #[must_use]
    #[wasm_bindgen(method, js_name = "printToPDF")]
    pub fn print_to_pdf(this: &WebContents, options: PrintToPdfOptions) -> Promise;

    #[wasm_bindgen(method)]
    pub fn redo(this: &WebContents);

    #[wasm_bindgen(method)]
    pub fn reload(this: &WebContents);

    #[wasm_bindgen(method, js_name = "reloadIgnoringCache")]
    pub fn reload_ignoring_cache(this: &WebContents);

    #[must_use]
    #[wasm_bindgen(method, js_name = "removeInsertedCSS")]
    pub fn remove_inserted_css(this: &WebContents, key: &str) -> Promise;

    #[wasm_bindgen(method, js_name = "removeWorkSpace")]
    pub fn remove_work_space(this: &WebContents, path: &str);

    #[wasm_bindgen(method)]
    pub fn replace(this: &WebContents, text: &str);

    #[wasm_bindgen(method, js_name = "replaceMisspelling")]
    pub fn replaceMisspelling(this: &WebContents, text: &str);

    #[must_use]
    #[wasm_bindgen(method, js_name = "savePage")]
    pub fn save_page(this: &WebContents, full_path: &str, save_type: &str) -> Promise;

    #[wasm_bindgen(method, js_name = "selectAll")]
    pub fn select_all(this: &WebContents);

    #[wasm_bindgen(method, variadic)]
    pub fn send(this: &WebContents, channel: &str, args: Box<[JsValue]>);

    #[wasm_bindgen(method, js_name = "sendInputEvent")]
    pub fn send_input_event(this: &WebContents, input_event: &InputEvent);

    #[wasm_bindgen(method, variadic, js_name = "sendToFrame")]
    pub fn send_to_frame(this: &WebContents, frame_id: usize, channel: &str, args: Box<[JsValue]>);

    #[wasm_bindgen(method, js_name = "setBackgroundThrottling")]
    pub fn set_background_throttling(this: &WebContents, allowed: bool);

    #[wasm_bindgen(method, js_name = "setDevToolsWebContents")]
    pub fn set_dev_tools_web_contents(this: &WebContents, dev_tools_web_contents: &WebContents);

    #[wasm_bindgen(method, js_name = "setIgnoreMenuShortcuts")]
    pub fn set_ignore_menu_shortcuts(this: &WebContents, ignore: bool);

    #[must_use]
    #[wasm_bindgen(method, js_name = "setLayoutZoomLevelLimits")]
    pub fn set_layout_zoom_level_limits(this: &WebContents, min: f32, max: f32) -> Promise;

    #[must_use]
    #[wasm_bindgen(method, js_name = "setVisualZoomLevelLimits")]
    pub fn set_visual_zoom_level_limits(this: &WebContents, min: f32, max: f32) -> Promise;

    #[wasm_bindgen(method, js_name = "setWebRTCIPHandlingPolicy")]
    pub fn set_web_rtc_ip_handling_policy(this: &WebContents, policy: &str);

    #[wasm_bindgen(method, js_name = "showDefinitionForSelection")]
    pub fn show_definition_for_selection(this: &WebContents);

    #[wasm_bindgen(method, js_name = "startDrag")]
    pub fn start_drag(this: &WebContents, item: Item);

    #[wasm_bindgen(method, js_name = "startPainting")]
    pub fn start_painting(this: &WebContents);

    #[wasm_bindgen(method)]
    pub fn stop(this: &WebContents);

    #[wasm_bindgen(method, js_name = "stopFindInPage")]
    pub fn stop_finding_in_page(this: &WebContents, action: &str);

    #[wasm_bindgen(method, js_name = "stopPainting")]
    pub fn stop_painting(this: &WebContents);

    #[wasm_bindgen(method, js_name = "takeHeapSnapshot")]
    pub fn take_heap_snapshot(this: &WebContents);

    #[wasm_bindgen(method, js_name = "toggleDevtools")]
    pub fn toggle_dev_tools(this: &WebContents);

    #[wasm_bindgen(method)]
    pub fn undo(this: &WebContents);

    #[wasm_bindgen(method)]
    pub fn unselect(this: &WebContents);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "audioMuted")]
    pub fn audio_muted(this: &WebContents) -> bool;

    #[wasm_bindgen(method, setter, js_name = "audioMuted")]
    pub fn set_audio_muted(this: &WebContents, value: bool);

    #[wasm_bindgen(method, getter)] // readonly
    pub fn debugger(this: &WebContents) -> Debugger;

    #[wasm_bindgen(method, getter, js_name = "devToolsWebContents")] // readonly
    pub fn dev_tools_web_contents(this: &WebContents) -> WebContents;

    #[wasm_bindgen(method, getter, js_name = "frameRate")]
    pub fn frame_rate(this: &WebContents) -> f32;

    #[wasm_bindgen(method, setter, js_name = "frameRate")]
    pub fn set_frame_rate(this: &WebContents, value: f32);

    #[wasm_bindgen(method, getter, js_name = "hostWebContents")] // readonly
    pub fn host_web_contents(this: &WebContents) -> WebContents;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn id(this: &WebContents) -> usize;

    #[wasm_bindgen(method, getter)] // readonly
    pub fn session(this: &WebContents) -> Session;

    #[wasm_bindgen(method, getter)]
    pub fn user_agent(this: &WebContents) -> JsString;

    #[wasm_bindgen(method, getter, js_name = "zoomFactor")]
    pub fn zoom_factor(this: &WebContents) -> f32;

    #[wasm_bindgen(method, setter, js_name = "zoomFactor")]
    pub fn set_zoom_factor(this: &WebContents, value: f32);

    #[wasm_bindgen(method, setter, js_name = "zoomLevel")]
    pub fn zoom_level(this: &WebContents) -> f32;

    #[wasm_bindgen(method, setter, js_name = "zoomLevel")]
    pub fn set_zoom_level(this: &WebContents, value: f32);
}
