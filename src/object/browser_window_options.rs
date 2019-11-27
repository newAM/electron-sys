use crate::{
    module::BrowserWindow,
    object::{NativeImage, WebPreferences},
};
use js_sys::{JsString, Number};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct BrowserWindowOptions {
    accept_first_mouse: Option<bool>,
    always_on_top: Option<bool>,
    auto_hide_menu_bar: Option<bool>,
    background_color: Option<JsString>,
    center: Option<bool>,
    closable: Option<bool>,
    dark_theme: Option<bool>,
    disable_auto_hide_cursor: Option<bool>,
    enable_larger_than_screen: Option<bool>,
    focusable: Option<bool>,
    frame: Option<bool>,
    fullscreen_window_title: Option<bool>,
    fullscreen: Option<bool>,
    fullscreenable: Option<bool>,
    has_shadow: Option<bool>,
    height: Option<usize>,
    icon: Option<NativeImage>, // FIXME: NativeImage
    kind: Option<JsString>,
    kiosk: Option<bool>,
    max_height: Option<usize>,
    max_width: Option<usize>,
    maximizable: Option<bool>,
    min_height: Option<usize>,
    min_width: Option<usize>,
    minimizable: Option<bool>,
    modal: Option<bool>,
    movable: Option<bool>,
    opacity: Option<Number>,
    paint_when_initially_hidden: Option<bool>,
    parent: Option<BrowserWindow>,
    resizable: Option<bool>,
    show: Option<bool>,
    simple_fullscreen: Option<bool>,
    skip_taskbar: Option<bool>,
    tabbing_identifier: Option<JsString>,
    thick_frame: Option<bool>,
    title_bar_style: Option<JsString>,
    title: Option<JsString>,
    transparent: Option<bool>,
    use_content_size: Option<bool>,
    vibrancy: Option<JsString>,
    web_preferences: Option<WebPreferences>,
    width: Option<usize>,
    x: Option<usize>,
    y: Option<usize>,
    zoom_to_page_width: Option<bool>,
}

#[wasm_bindgen]
impl BrowserWindowOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        accept_first_mouse: Option<bool>,
        always_on_top: Option<bool>,
        auto_hide_menu_bar: Option<bool>,
        background_color: Option<JsString>,
        center: Option<bool>,
        closable: Option<bool>,
        dark_theme: Option<bool>,
        disable_auto_hide_cursor: Option<bool>,
        enable_larger_than_screen: Option<bool>,
        focusable: Option<bool>,
        frame: Option<bool>,
        fullscreen_window_title: Option<bool>,
        fullscreen: Option<bool>,
        fullscreenable: Option<bool>,
        has_shadow: Option<bool>,
        height: Option<usize>,
        icon: Option<NativeImage>,
        kind: Option<JsString>,
        kiosk: Option<bool>,
        max_height: Option<usize>,
        max_width: Option<usize>,
        maximizable: Option<bool>,
        min_height: Option<usize>,
        min_width: Option<usize>,
        minimizable: Option<bool>,
        modal: Option<bool>,
        movable: Option<bool>,
        opacity: Option<Number>,
        paint_when_initially_hidden: Option<bool>,
        parent: Option<BrowserWindow>,
        resizable: Option<bool>,
        show: Option<bool>,
        simple_fullscreen: Option<bool>,
        skip_taskbar: Option<bool>,
        tabbing_identifier: Option<JsString>,
        thick_frame: Option<bool>,
        title_bar_style: Option<JsString>,
        title: Option<JsString>,
        transparent: Option<bool>,
        use_content_size: Option<bool>,
        vibrancy: Option<JsString>,
        web_preferences: Option<WebPreferences>,
        width: Option<usize>,
        x: Option<usize>,
        y: Option<usize>,
        zoom_to_page_width: Option<bool>,
    ) -> BrowserWindowOptions {
        BrowserWindowOptions {
            accept_first_mouse,
            always_on_top,
            auto_hide_menu_bar,
            background_color,
            center,
            closable,
            dark_theme,
            disable_auto_hide_cursor,
            enable_larger_than_screen,
            focusable,
            frame,
            fullscreen_window_title,
            fullscreen,
            fullscreenable,
            has_shadow,
            height,
            icon,
            kind,
            kiosk,
            max_height,
            max_width,
            maximizable,
            min_height,
            min_width,
            minimizable,
            modal,
            movable,
            opacity,
            paint_when_initially_hidden,
            parent,
            resizable,
            show,
            simple_fullscreen,
            skip_taskbar,
            tabbing_identifier,
            thick_frame,
            title_bar_style,
            title,
            transparent,
            use_content_size,
            vibrancy,
            web_preferences,
            width,
            x,
            y,
            zoom_to_page_width,
        }
    }

    pub fn new() -> BrowserWindowOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn accept_first_mouse(&self) -> Option<bool> {
        self.accept_first_mouse
    }

    #[wasm_bindgen(setter)]
    pub fn set_accept_first_mouse(&mut self, value: Option<bool>) {
        self.accept_first_mouse = value;
    }

    #[wasm_bindgen(getter)]
    pub fn always_on_top(&self) -> Option<bool> {
        self.always_on_top
    }

    #[wasm_bindgen(setter)]
    pub fn set_always_on_top(&mut self, value: Option<bool>) {
        self.always_on_top = value;
    }

    #[wasm_bindgen(getter)]
    pub fn auto_hide_menu_bar(&self) -> Option<bool> {
        self.auto_hide_menu_bar
    }

    #[wasm_bindgen(setter)]
    pub fn set_auto_hide_menu_bar(&mut self, value: Option<bool>) {
        self.auto_hide_menu_bar = value;
    }

    #[wasm_bindgen(getter)]
    pub fn background_color(&self) -> Option<JsString> {
        self.background_color.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_background_color(&mut self, value: Option<JsString>) {
        self.background_color = value;
    }

    #[wasm_bindgen(getter)]
    pub fn center(&self) -> Option<bool> {
        self.center
    }

    #[wasm_bindgen(setter)]
    pub fn set_center(&mut self, value: Option<bool>) {
        self.center = value;
    }

    #[wasm_bindgen(getter)]
    pub fn closable(&self) -> Option<bool> {
        self.closable
    }

    #[wasm_bindgen(setter)]
    pub fn set_closable(&mut self, value: Option<bool>) {
        self.closable = value;
    }

    #[wasm_bindgen(getter)]
    pub fn dark_theme(&self) -> Option<bool> {
        self.dark_theme
    }

    #[wasm_bindgen(setter)]
    pub fn set_dark_theme(&mut self, value: Option<bool>) {
        self.dark_theme = value;
    }

    #[wasm_bindgen(getter)]
    pub fn disable_auto_hide_cursor(&self) -> Option<bool> {
        self.disable_auto_hide_cursor
    }

    #[wasm_bindgen(setter)]
    pub fn set_disable_auto_hide_cursor(&mut self, value: Option<bool>) {
        self.disable_auto_hide_cursor = value;
    }

    #[wasm_bindgen(getter)]
    pub fn enable_larger_than_screen(&self) -> Option<bool> {
        self.enable_larger_than_screen
    }

    #[wasm_bindgen(setter)]
    pub fn set_enable_larger_than_screen(&mut self, value: Option<bool>) {
        self.enable_larger_than_screen = value;
    }

    #[wasm_bindgen(getter)]
    pub fn focusable(&self) -> Option<bool> {
        self.focusable
    }

    #[wasm_bindgen(setter)]
    pub fn set_focusable(&mut self, value: Option<bool>) {
        self.focusable = value;
    }

    #[wasm_bindgen(getter)]
    pub fn frame(&self) -> Option<bool> {
        self.frame
    }

    #[wasm_bindgen(setter)]
    pub fn set_frame(&mut self, value: Option<bool>) {
        self.frame = value;
    }

    #[wasm_bindgen(getter)]
    pub fn fullscreen_window_title(&self) -> Option<bool> {
        self.fullscreen_window_title
    }

    #[wasm_bindgen(setter)]
    pub fn set_fullscreen_window_title(&mut self, value: Option<bool>) {
        self.fullscreen_window_title = value;
    }

    #[wasm_bindgen(getter)]
    pub fn fullscreen(&self) -> Option<bool> {
        self.fullscreen
    }

    #[wasm_bindgen(setter)]
    pub fn set_fullscreen(&mut self, value: Option<bool>) {
        self.fullscreen = value;
    }

    #[wasm_bindgen(getter)]
    pub fn fullscreenable(&self) -> Option<bool> {
        self.fullscreenable
    }

    #[wasm_bindgen(setter)]
    pub fn set_fullscreenable(&mut self, value: Option<bool>) {
        self.fullscreenable = value;
    }

    #[wasm_bindgen(getter)]
    pub fn has_shadow(&self) -> Option<bool> {
        self.has_shadow
    }

    #[wasm_bindgen(setter)]
    pub fn set_has_shadow(&mut self, value: Option<bool>) {
        self.has_shadow = value;
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> Option<usize> {
        self.height
    }

    #[wasm_bindgen(setter)]
    pub fn set_height(&mut self, value: Option<usize>) {
        self.height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn icon(&self) -> Option<NativeImage> {
        self.icon.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_icon(&mut self, value: Option<NativeImage>) {
        self.icon = value;
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> Option<JsString> {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: Option<JsString>) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn kiosk(&self) -> Option<bool> {
        self.kiosk
    }

    #[wasm_bindgen(setter)]
    pub fn set_kiosk(&mut self, value: Option<bool>) {
        self.kiosk = value;
    }

    #[wasm_bindgen(getter)]
    pub fn max_height(&self) -> Option<usize> {
        self.max_height
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_height(&mut self, value: Option<usize>) {
        self.max_height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn max_width(&self) -> Option<usize> {
        self.max_width
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_width(&mut self, value: Option<usize>) {
        self.max_width = value;
    }

    #[wasm_bindgen(getter)]
    pub fn maximizable(&self) -> Option<bool> {
        self.maximizable
    }

    #[wasm_bindgen(setter)]
    pub fn set_maximizable(&mut self, value: Option<bool>) {
        self.maximizable = value;
    }

    #[wasm_bindgen(getter)]
    pub fn min_height(&self) -> Option<usize> {
        self.min_height
    }

    #[wasm_bindgen(setter)]
    pub fn set_min_height(&mut self, value: Option<usize>) {
        self.min_height = value;
    }

    #[wasm_bindgen(getter)]
    pub fn min_width(&self) -> Option<usize> {
        self.min_width
    }

    #[wasm_bindgen(setter)]
    pub fn set_min_width(&mut self, value: Option<usize>) {
        self.min_width = value;
    }

    #[wasm_bindgen(getter)]
    pub fn minimizable(&self) -> Option<bool> {
        self.minimizable
    }

    #[wasm_bindgen(setter)]
    pub fn set_minimizable(&mut self, value: Option<bool>) {
        self.minimizable = value;
    }

    #[wasm_bindgen(getter)]
    pub fn modal(&self) -> Option<bool> {
        self.modal
    }

    #[wasm_bindgen(setter)]
    pub fn set_modal(&mut self, value: Option<bool>) {
        self.modal = value;
    }

    #[wasm_bindgen(getter)]
    pub fn movable(&self) -> Option<bool> {
        self.movable
    }

    #[wasm_bindgen(setter)]
    pub fn set_movable(&mut self, value: Option<bool>) {
        self.movable = value;
    }

    #[wasm_bindgen(getter)]
    pub fn opacity(&self) -> Option<Number> {
        self.opacity.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_opacity(&mut self, value: Option<Number>) {
        self.opacity = value;
    }

    #[wasm_bindgen(getter)]
    pub fn paint_when_initially_hidden(&self) -> Option<bool> {
        self.paint_when_initially_hidden
    }

    #[wasm_bindgen(setter)]
    pub fn set_paint_when_initially_hidden(&mut self, value: Option<bool>) {
        self.paint_when_initially_hidden = value;
    }

    #[wasm_bindgen(getter)]
    pub fn parent(&self) -> Option<BrowserWindow> {
        self.parent.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_parent(&mut self, value: Option<BrowserWindow>) {
        self.parent = value;
    }

    #[wasm_bindgen(getter)]
    pub fn resizable(&self) -> Option<bool> {
        self.resizable
    }

    #[wasm_bindgen(setter)]
    pub fn set_resizable(&mut self, value: Option<bool>) {
        self.resizable = value;
    }

    #[wasm_bindgen(getter)]
    pub fn show(&self) -> Option<bool> {
        self.show
    }

    #[wasm_bindgen(setter)]
    pub fn set_show(&mut self, value: Option<bool>) {
        self.show = value;
    }

    #[wasm_bindgen(getter)]
    pub fn simple_fullscreen(&self) -> Option<bool> {
        self.simple_fullscreen
    }

    #[wasm_bindgen(setter)]
    pub fn set_simple_fullscreen(&mut self, value: Option<bool>) {
        self.simple_fullscreen = value;
    }

    #[wasm_bindgen(getter)]
    pub fn skip_taskbar(&self) -> Option<bool> {
        self.skip_taskbar
    }

    #[wasm_bindgen(setter)]
    pub fn set_skip_taskbar(&mut self, value: Option<bool>) {
        self.skip_taskbar = value;
    }

    #[wasm_bindgen(getter)]
    pub fn tabbing_identifier(&self) -> Option<JsString> {
        self.tabbing_identifier.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_tabbing_identifier(&mut self, value: Option<JsString>) {
        self.tabbing_identifier = value;
    }

    #[wasm_bindgen(getter)]
    pub fn thick_frame(&self) -> Option<bool> {
        self.thick_frame
    }

    #[wasm_bindgen(setter)]
    pub fn set_thick_frame(&mut self, value: Option<bool>) {
        self.thick_frame = value;
    }

    #[wasm_bindgen(getter)]
    pub fn title_bar_style(&self) -> Option<JsString> {
        self.title_bar_style.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title_bar_style(&mut self, value: Option<JsString>) {
        self.title_bar_style = value;
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> Option<JsString> {
        self.title.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, value: Option<JsString>) {
        self.title = value;
    }

    #[wasm_bindgen(getter)]
    pub fn transparent(&self) -> Option<bool> {
        self.transparent
    }

    #[wasm_bindgen(setter)]
    pub fn set_transparent(&mut self, value: Option<bool>) {
        self.transparent = value;
    }

    #[wasm_bindgen(getter)]
    pub fn use_content_size(&self) -> Option<bool> {
        self.use_content_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_use_content_size(&mut self, value: Option<bool>) {
        self.use_content_size = value;
    }

    #[wasm_bindgen(getter)]
    pub fn vibrancy(&self) -> Option<JsString> {
        self.vibrancy.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_vibrancy(&mut self, value: Option<JsString>) {
        self.vibrancy = value;
    }

    #[wasm_bindgen(getter)]
    pub fn web_preferences(&self) -> Option<WebPreferences> {
        self.web_preferences.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_web_preferences(&mut self, value: Option<WebPreferences>) {
        self.web_preferences = value;
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> Option<usize> {
        self.width
    }

    #[wasm_bindgen(setter)]
    pub fn set_width(&mut self, value: Option<usize>) {
        self.width = value;
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> Option<usize> {
        self.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, value: Option<usize>) {
        self.x = value;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> Option<usize> {
        self.y
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, value: Option<usize>) {
        self.y = value;
    }

    #[wasm_bindgen(getter)]
    pub fn zoom_to_page_width(&self) -> Option<bool> {
        self.zoom_to_page_width
    }

    #[wasm_bindgen(setter)]
    pub fn set_zoom_to_page_width(&mut self, value: Option<bool>) {
        self.zoom_to_page_width = value;
    }
}
