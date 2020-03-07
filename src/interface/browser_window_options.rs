use crate::{
    class::{BrowserWindow, NativeImage},
    interface::WebPreferences,
};
use js_sys::{JsString, Object};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq)]
    pub type BrowserWindowOptions;

    #[wasm_bindgen(method, getter, js_name = "acceptFirstMouse")]
    pub fn accept_first_mouse(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "acceptFirstMouse")]
    pub fn set_accept_first_mouse(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "alwaysOnTop")]
    pub fn always_on_top(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "alwaysOnTop")]
    pub fn set_always_on_top(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "autoHideMenuBar")]
    pub fn auto_hide_menu_bar(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "autoHideMenuBar")]
    pub fn set_auto_hide_menu_bar(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "backgroundColor")]
    pub fn background_color(this: &BrowserWindowOptions) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "backgroundColor")]
    pub fn set_background_color(this: &BrowserWindowOptions, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn center(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_center(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn closable(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_closable(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "darkTheme")]
    pub fn dark_theme(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "darkTheme")]
    pub fn set_dark_theme(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "disableAutoHideCursor")]
    pub fn disable_auto_hide_cursor(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "disableAutoHideCursor")]
    pub fn set_disable_auto_hide_cursor(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "enableLargerThanScreen")]
    pub fn enable_larger_than_screen(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "enableLargerThanScreen")]
    pub fn set_enable_larger_than_screen(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn focusable(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_focusable(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn frame(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_frame(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "fullscreenWindowTitle")]
    pub fn fullscreen_window_title(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "fullscreenWindowTitle")]
    pub fn set_fullscreen_window_title(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn fullscreen(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_fullscreen(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn fullscreenable(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_fullscreenable(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "hasShadow")]
    pub fn has_shadow(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "hasShadow")]
    pub fn set_has_shadow(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &BrowserWindowOptions) -> Option<usize>;

    #[wasm_bindgen(method, setter)]
    pub fn set_height(this: &BrowserWindowOptions, value: Option<usize>);

    #[wasm_bindgen(method, getter)]
    pub fn icon(this: &BrowserWindowOptions) -> Option<NativeImage>;

    #[wasm_bindgen(method, setter)]
    pub fn set_icon(this: &BrowserWindowOptions, value: Option<NativeImage>);

    #[wasm_bindgen(method, getter, js_name = "type")]
    pub fn kind(this: &BrowserWindowOptions) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "type")]
    pub fn set_kind(this: &BrowserWindowOptions, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn kiosk(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_kiosk(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "maxHeight")]
    pub fn max_height(this: &BrowserWindowOptions) -> Option<usize>;

    #[wasm_bindgen(method, setter, js_name = "maxHeight")]
    pub fn set_max_height(this: &BrowserWindowOptions, value: Option<usize>);

    #[wasm_bindgen(method, getter, js_name = "maxWidth")]
    pub fn max_width(this: &BrowserWindowOptions) -> Option<usize>;

    #[wasm_bindgen(method, setter, js_name = "maxWidth")]
    pub fn set_max_width(this: &BrowserWindowOptions, value: Option<usize>);

    #[wasm_bindgen(method, getter)]
    pub fn maximizable(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_maximizable(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "minHeight")]
    pub fn min_height(this: &BrowserWindowOptions) -> Option<usize>;

    #[wasm_bindgen(method, setter, js_name = "minHeight")]
    pub fn set_min_height(this: &BrowserWindowOptions, value: Option<usize>);

    #[wasm_bindgen(method, getter, js_name = "minWidth")]
    pub fn min_width(this: &BrowserWindowOptions) -> Option<usize>;

    #[wasm_bindgen(method, setter, js_name = "minWidth")]
    pub fn set_min_width(this: &BrowserWindowOptions, value: Option<usize>);

    #[wasm_bindgen(method, getter)]
    pub fn minimizable(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_minimizable(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn modal(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_modal(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn movable(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_movable(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn opacity(this: &BrowserWindowOptions) -> Option<f32>;

    #[wasm_bindgen(method, setter)]
    pub fn set_opacity(this: &BrowserWindowOptions, value: Option<f32>);

    #[wasm_bindgen(method, getter, js_name = "painWhenInitiallyHidden")]
    pub fn paint_when_initially_hidden(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "painWhenInitiallyHidden")]
    pub fn set_paint_when_initially_hidden(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn parent(this: &BrowserWindowOptions) -> Option<BrowserWindow>;

    #[wasm_bindgen(method, setter)]
    pub fn set_parent(this: &BrowserWindowOptions, value: Option<BrowserWindow>);

    #[wasm_bindgen(method, getter)]
    pub fn resizable(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_resizable(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn show(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_show(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "simpleFullscreen")]
    pub fn simple_fullscreen(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "simpleFullscreen")]
    pub fn set_simple_fullscreen(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "skipTaskbar")]
    pub fn skip_taskbar(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "skipTaskbar")]
    pub fn set_skip_taskbar(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "tabbingIdentifier")]
    pub fn tabbing_identifier(this: &BrowserWindowOptions) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "tabbingIdentifier")]
    pub fn set_tabbing_identifier(this: &BrowserWindowOptions, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "thickFrame")]
    pub fn thick_frame(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "thickFrame")]
    pub fn set_thick_frame(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "titleBarStyle")]
    pub fn title_bar_style(this: &BrowserWindowOptions) -> Option<JsString>;

    #[wasm_bindgen(method, setter, js_name = "titleBarStyle")]
    pub fn set_title_bar_style(this: &BrowserWindowOptions, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn title(this: &BrowserWindowOptions) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_title(this: &BrowserWindowOptions, value: Option<JsString>);

    #[wasm_bindgen(method, getter)]
    pub fn transparent(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_transparent(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "useContentSize")]
    pub fn use_content_size(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "useContentSize")]
    pub fn set_use_content_size(this: &BrowserWindowOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn vibrancy(this: &BrowserWindowOptions) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_vibrancy(this: &BrowserWindowOptions, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "webPreferences")]
    pub fn web_preferences(this: &BrowserWindowOptions) -> Option<WebPreferences>;

    #[wasm_bindgen(method, setter, js_name = "webPreferences")]
    pub fn set_web_preferences(this: &BrowserWindowOptions, value: Option<WebPreferences>);

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &BrowserWindowOptions) -> Option<usize>;

    #[wasm_bindgen(method, setter)]
    pub fn set_width(this: &BrowserWindowOptions, value: Option<usize>);

    #[wasm_bindgen(method, getter)]
    pub fn x(this: &BrowserWindowOptions) -> Option<usize>;

    #[wasm_bindgen(method, setter)]
    pub fn set_x(this: &BrowserWindowOptions, value: Option<usize>);

    #[wasm_bindgen(method, getter)]
    pub fn y(this: &BrowserWindowOptions) -> Option<usize>;

    #[wasm_bindgen(method, setter)]
    pub fn set_y(this: &BrowserWindowOptions, value: Option<usize>);

    #[wasm_bindgen(method, getter, js_name = "zoomToPageWidth")]
    pub fn zoom_to_page_width(this: &BrowserWindowOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "zoomToPageWidth")]
    pub fn set_zoom_to_page_width(this: &BrowserWindowOptions, value: Option<bool>);
}

impl Default for BrowserWindowOptions {
    fn default() -> Self {
        Object::new().unchecked_into()
    }
}
