use crate::{
    class::{BrowserView, Menu, NativeImage, TouchBar, WebContents},
    interface::{
        AppDetailsOptions,
        BrowserWindowOptions,
        IgnoreMouseEventsOptions,
        LoadFileOptions,
        LoadUrlOptions,
        ProgressBarOptions,
        Rectangle,
        Size,
        VisibleOnAllWorkspacesOptions,
    },
};
use js_sys::{Array, Function, JsString, Map};
use node_sys::{events::EventEmitter, Buffer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = EventEmitter)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    /// Docs: http://electronjs.org/docs/api/browser-window
    pub type BrowserWindow;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "addDevToolsExtension")]
    pub fn add_dev_tools_extension(path: &JsString);

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "addExtension")]
    pub fn add_extension(path: &JsString);

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "fromBrowserView")]
    pub fn from_browser_view(browser_view: &BrowserView) -> Option<BrowserWindow>;

    // FIXME: should return Option<BrowserWindow>?
    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "from_id")]
    pub fn from_id(id: u32) -> BrowserWindow;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "from_web_contents")]
    pub fn from_web_contents(web_contents: &WebContents) -> Option<BrowserWindow>;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "getAllWindows")]
    pub fn get_all_windows() -> Array;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "getDevToolsExtension")]
    pub fn get_dev_tools_extension() -> Map;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "getFocusedWindow")]
    pub fn get_focused_window() -> Option<BrowserWindow>;

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "removeDevToolsExtension")]
    pub fn remove_dev_tools_extension(name: &JsString);

    #[wasm_bindgen(static_method_of = BrowserWindow, js_name = "removeExtension")]
    pub fn remove_extension(name: &JsString);

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<BrowserWindowOptions>) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "addBrowserView")]
    pub fn add_browser_view(this: &BrowserWindow, browser_view: &BrowserView);

    #[wasm_bindgen(method, js_name = "addTabbedWindow")]
    pub fn add_tabbed_window(this: &BrowserWindow, browser_view: &BrowserWindow);

    #[wasm_bindgen(method)]
    pub fn blur(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "blurWebView")]
    pub fn blur_web_view(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "capturePage")]
    pub fn capture_page(this: &BrowserWindow, rectangle: Option<Rectangle>);

    #[wasm_bindgen(method)]
    pub fn center(this: &BrowserWindow);

    #[wasm_bindgen(method)]
    pub fn close(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "closeFilePreview")]
    pub fn close_file_preview(this: &BrowserWindow);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "flashFrame")]
    pub fn flash_frame(this: &BrowserWindow);

    #[wasm_bindgen(method)]
    pub fn focus(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "focusOnWebView")]
    pub fn focus_on_web_view(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "getBounds")]
    pub fn get_bounds(this: &BrowserWindow) -> Rectangle;

    #[wasm_bindgen(method, js_name = "getBrowserView")]
    pub fn get_browser_view(this: &BrowserWindow) -> Option<BrowserView>;

    #[wasm_bindgen(method, js_name = "getBrowserViews")]
    pub fn get_browser_views(this: &BrowserWindow) -> Array;

    #[wasm_bindgen(method, js_name = "getChildWindows")]
    pub fn get_child_windows(this: &BrowserWindow) -> Array;

    #[wasm_bindgen(method, js_name = "getContentBounds")]
    pub fn get_content_bounds(this: &BrowserWindow) -> Rectangle;

    #[wasm_bindgen(method, js_name = "getContentSize")]
    pub fn get_content_size(this: &BrowserWindow) -> Box<[u32]>;

    #[wasm_bindgen(method, js_name = "getMaximumSize")]
    pub fn get_maximum_size(this: &BrowserWindow) -> Box<[u32]>;

    #[wasm_bindgen(method, js_name = "getMediaSourceId")]
    pub fn get_media_source_id(this: &BrowserWindow) -> JsString;

    #[wasm_bindgen(method, js_name = "getMinimumSize")]
    pub fn get_minimum_size(this: &BrowserWindow) -> Array;

    #[wasm_bindgen(method, js_name = "getNativeWindowHandle")]
    pub fn get_native_window_handle(this: &BrowserWindow) -> Buffer;

    #[wasm_bindgen(method, js_name = "getNormalBounds")]
    pub fn get_normal_bounds(this: &BrowserWindow) -> Rectangle;

    #[wasm_bindgen(method, js_name = "getOpacity")]
    pub fn get_opacity(this: &BrowserWindow) -> f32;

    #[wasm_bindgen(method, js_name = "getParentWindow")]
    pub fn get_parent_window(this: &BrowserWindow) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "getPosition")]
    pub fn get_position(this: &BrowserWindow) -> Array;

    #[wasm_bindgen(method, js_name = "getRepresentedFilename")]
    pub fn get_represented_filename(this: &BrowserWindow) -> Array;

    #[wasm_bindgen(method, js_name = "getSize")]
    pub fn get_size(this: &BrowserWindow) -> Array;

    #[wasm_bindgen(method, js_name = "getTitle")]
    pub fn get_title(this: &BrowserWindow) -> JsString;

    #[wasm_bindgen(method, js_name = "hasShadow")]
    pub fn has_shadow(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method)]
    pub fn hide(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "hookWindowMessage")]
    pub fn hook_window_message(this: &BrowserWindow, message: u32, callback: &Function);

    #[wasm_bindgen(method, js_name = "isAlwaysOnTop")]
    pub fn is_always_on_top(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isDestroyed")]
    pub fn is_destroyed(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isDocumentEdited")]
    pub fn is_document_edited(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isEnabled")]
    pub fn is_enabled(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isFocued")]
    pub fn is_focused(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isFullScreenable")]
    pub fn is_full_screenable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isKiosk")]
    pub fn is_kiosk(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isMaximized")]
    pub fn is_maximized(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isMenuBarVisible")]
    pub fn is_menu_bar_visible(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isMinimized")]
    pub fn is_minimized(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isModal")]
    pub fn is_modal(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isNormal")]
    pub fn is_normal(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isSimpleFullScreen")]
    pub fn is_simple_full_screen(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isVisible")]
    pub fn is_visible(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isVisibleOnAllWorkspaces")]
    pub fn is_visible_on_all_workspaces(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, js_name = "isWindowMessageHooked")]
    pub fn is_window_message_hooked(this: &BrowserWindow, message: u32) -> bool;

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &BrowserWindow, path: &JsString, options: Option<LoadFileOptions>);

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_url(this: &BrowserWindow, url: &JsString, options: Option<LoadUrlOptions>);

    #[wasm_bindgen(method)]
    pub fn maximize(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "mergeAllWindows")]
    pub fn merge_all_windows(this: &BrowserWindow);

    #[wasm_bindgen(method)]
    pub fn minimize(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "moveAbove")]
    pub fn move_above(this: &BrowserWindow, media_source_id: JsString);

    #[wasm_bindgen(method, js_name = "moveTabToNewWindow")]
    pub fn move_tab_to_new_window(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "moveTop")]
    pub fn move_top(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "previewFile")]
    pub fn preview_file(this: &BrowserWindow, path: &JsString, display_name: Option<&JsString>);

    #[wasm_bindgen(method)]
    pub fn reload(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "removeBrowserView")]
    pub fn remove_browser_view(this: &BrowserWindow, browser_view: &BrowserView);

    #[wasm_bindgen(method, js_name = "removeMenu")]
    pub fn remove_menu(this: &BrowserWindow);

    #[wasm_bindgen(method)]
    pub fn restore(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "selectNextTab")]
    pub fn select_next_tab(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "selectPreviousTab")]
    pub fn select_previous_tab(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "setAlwaysOnTop")]
    pub fn set_always_on_top(this: &BrowserWindow, flag: bool, level: Option<&JsString>, relative_level: Option<i32>);

    #[wasm_bindgen(method, js_name = "setAppDetails")]
    pub fn set_app_details(this: &BrowserWindow, options: AppDetailsOptions);

    #[wasm_bindgen(method, js_name = "setAspectRatio")]
    pub fn set_aspect_ratio(this: &BrowserWindow, aspect_ratio: f32, extra_size: Option<Size>);

    #[wasm_bindgen(method, js_name = "setAutoHideCursor")]
    pub fn set_auto_hide_cursor(this: &BrowserWindow, auto_hide: bool);

    #[wasm_bindgen(method, js_name = "setBackgroundColor")]
    pub fn set_background_color(this: &BrowserWindow, background_color: &JsString);

    #[wasm_bindgen(method, js_name = "setBounds")]
    pub fn set_bounds(this: &BrowserWindow, bounds: Rectangle, animate: Option<bool>);

    #[wasm_bindgen(method, js_name = "setBrowserView")]
    pub fn set_browser_view(this: &BrowserWindow, browser_view: &BrowserView);

    #[wasm_bindgen(method, js_name = "setContentBounds")]
    pub fn set_content_bounds(this: &BrowserWindow, bounds: Rectangle, animate: Option<bool>);

    #[wasm_bindgen(method, js_name = "setContentProtection")]
    pub fn set_content_protection(this: &BrowserWindow, enable: bool);

    #[wasm_bindgen(method, js_name = "setContentSize")]
    pub fn set_content_size(this: &BrowserWindow, width: u32, height: u32, animate: Option<bool>);

    #[wasm_bindgen(method, js_name = "setDocumentEdited")]
    pub fn set_document_edited(this: &BrowserWindow, edited: bool);

    #[wasm_bindgen(method, js_name = "setEnabled")]
    pub fn set_enabled(this: &BrowserWindow, edited: bool);

    #[wasm_bindgen(method, js_name = "setFocusable")]
    pub fn set_focusable(this: &BrowserWindow, focusable: bool);

    #[wasm_bindgen(method, js_name = "setFullScreen")]
    pub fn set_full_screen(this: &BrowserWindow, flag: bool);

    #[wasm_bindgen(method, js_name = "setHasShadow")]
    pub fn set_has_shadow(this: &BrowserWindow, has_shadow: bool);

    #[wasm_bindgen(method, js_name = "setIcon")]
    pub fn set_icon(this: &BrowserWindow, icon: &NativeImage);

    #[wasm_bindgen(method, js_name = "setIgnoreMouseEvents")]
    pub fn set_ignore_mouse_events(this: &BrowserWindow, ignore: bool, options: Option<IgnoreMouseEventsOptions>);

    #[wasm_bindgen(method, js_name = "setKiosk")]
    pub fn set_kiosk(this: &BrowserWindow, flag: bool);

    #[wasm_bindgen(method, js_name = "setMaximizable")]
    pub fn set_maximumizable(this: &BrowserWindow, maximizable: bool);

    #[wasm_bindgen(method, js_name = "setMaximumSize")]
    pub fn set_maximum_size(this: &BrowserWindow, width: u32, height: u32);

    #[wasm_bindgen(method, js_name = "setMenu")]
    pub fn set_menu(this: &BrowserWindow, menu: Option<Menu>);

    #[wasm_bindgen(method, js_name = "setMenuBarVisibility")]
    pub fn set_menu_bar_visibility(this: &BrowserWindow, visible: bool);

    #[wasm_bindgen(method, js_name = "setMinimumSize")]
    pub fn set_minimum_size(this: &BrowserWindow, width: u32, height: u32);

    #[wasm_bindgen(method, js_name = "setOpacity")]
    pub fn set_opacity(this: &BrowserWindow, opacity: f32);

    #[wasm_bindgen(method, js_name = "setOverlayIcon")]
    pub fn set_overlay_icon(this: &BrowserWindow, overlay: Option<&NativeImage>, description: &JsString);

    #[wasm_bindgen(method, js_name = "setParentWindow")]
    pub fn set_parent_window(this: &BrowserWindow, parent: Option<&BrowserWindow>);

    #[wasm_bindgen(method, js_name = "setPosition")]
    pub fn set_position(this: &BrowserWindow, x: u32, y: u32, animate: Option<bool>);

    #[wasm_bindgen(method, js_name = "setProgressBar")]
    pub fn set_progress_bar(this: &BrowserWindow, progress: u32, options: Option<ProgressBarOptions>);

    #[wasm_bindgen(method, js_name = "setRepresentedFilename")]
    pub fn set_represented_filename(this: &BrowserWindow, filename: &JsString);

    #[wasm_bindgen(method, js_name = "setShape")]
    pub fn set_shape(this: &BrowserWindow, rectangles: &Array);

    #[wasm_bindgen(method, js_name = "setSheetOffset")]
    pub fn set_sheet_offset(this: &BrowserWindow, y: i32, x: Option<i32>);

    #[wasm_bindgen(method, js_name = "setSimpleFullScreen")]
    pub fn set_simple_full_screen(this: &BrowserWindow, flag: bool);

    #[wasm_bindgen(method, js_name = "setSize")]
    pub fn set_size(this: &BrowserWindow, width: u32, height: u32, animate: Option<bool>);

    #[wasm_bindgen(method, js_name = "setSkipTaskbar")]
    pub fn set_skip_taskbar(this: &BrowserWindow, skip: bool);

    #[wasm_bindgen(method, js_name = "setThumbarButtons")]
    pub fn set_thumbar_buttons(this: &BrowserWindow, buttons: &Array) -> bool;

    #[wasm_bindgen(method, js_name = "setThumbnailClip")]
    pub fn set_thumbnail_clip(this: &BrowserWindow, region: Rectangle);

    #[wasm_bindgen(method, js_name = "setThumbnailToolTip")]
    pub fn set_thumbnail_tool_tip(this: &BrowserWindow, tool_tip: &JsString);

    #[wasm_bindgen(method, js_name = "setTitle")]
    pub fn set_title(this: &BrowserWindow, title: &JsString);

    #[wasm_bindgen(method, js_name = "setTouchBar")]
    pub fn set_touch_bar(this: &BrowserWindow, touch_bar: Option<&TouchBar>);

    #[wasm_bindgen(method, js_name = "setVibrancy")]
    pub fn set_vibrancy(this: &BrowserWindow, kind: Option<&JsString>);

    #[wasm_bindgen(method, js_name = "setVisibleOnAllWorkspaces")]
    pub fn set_visible_on_all_workspaces(
        this: &BrowserWindow,
        visible: bool,
        options: Option<VisibleOnAllWorkspacesOptions>,
    );

    #[wasm_bindgen(method, js_name = "setWindowButtonVisibility")]
    pub fn set_window_button_visibility(this: &BrowserWindow, visible: bool);

    #[wasm_bindgen(method)]
    pub fn show(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "showAppDetails")]
    pub fn show_app_details(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "showAspectRatio")]
    pub fn show_aspect_ratio(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "showDefinitionForSelection")]
    pub fn show_definition_for_selection(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "showInactive")]
    pub fn show_inactive(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "toggleTabBar")]
    pub fn toggle_tab_bar(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "unhookAllWindowMessages")]
    pub fn unhook_all_window_messages(this: &BrowserWindow);

    #[wasm_bindgen(method, js_name = "unhookWindowMessage")]
    pub fn unhook_window_messages(this: &BrowserWindow, message: u32);

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "accessibleTitle")]
    pub fn accessible_title(this: &BrowserWindow) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "accessibleTitle")]
    pub fn set_accessible_title(this: &BrowserWindow, value: JsString);

    #[wasm_bindgen(method, getter, js_name = "autoHideMenuBar")]
    pub fn auto_hide_menu_bar(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter, js_name = "autoHideMenuBar")]
    pub fn set_auto_hide_menu_bar(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn closable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter, js_name = "closable")]
    pub fn set_closable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter, js_name = "excludedFromShownWindowsMenu")]
    pub fn excluded_from_shown_windows_menu(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter, js_name = "excludedFromShownWindowsMenu")]
    pub fn set_excluded_from_shown_windows_menu(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter, js_name = "fullScreenable")]
    pub fn full_screenable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter, js_name = "fullScreenable")]
    pub fn set_full_screenable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)] // readonly
    pub fn id(this: &BrowserWindow) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn maximizable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_maximizable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn minimizable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_minimizable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn movable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_movable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter)]
    pub fn resizable(this: &BrowserWindow) -> bool;

    #[wasm_bindgen(method, setter)]
    pub fn set_resizable(this: &BrowserWindow, value: bool);

    #[wasm_bindgen(method, getter, js_name = "webContents")] // readonly
    pub fn web_contents(this: &BrowserWindow) -> WebContents;
}
