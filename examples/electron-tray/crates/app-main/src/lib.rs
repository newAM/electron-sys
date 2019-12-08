use electron_sys::{app, native_image, BrowserWindow, BrowserWindowOptions, Tray, WebPreferences};
use js_sys::{Array, Math};
use node_sys::path;
use wasm_bindgen::{prelude::*, JsCast};

// calculate the appropriate browser window position
fn get_window_position(window: &BrowserWindow, tray: &Tray) -> (u32, u32) {
    let window_bounds = window.get_bounds();
    let tray_bounds = tray.get_bounds();
    let x =
        Math::round(tray_bounds.x() as f64 + (tray_bounds.width() as f64 / 2.0) - (window_bounds.width() as f64 / 2.0))
            as u32;
    let y = Math::round(tray_bounds.y() as f64 + tray_bounds.height() as f64 + 4.0) as u32;
    (x, y)
}

// show the browser window
fn show_window(window: &BrowserWindow, tray: &Tray) {
    let (x, y) = get_window_position(window, tray);
    window.set_position(x, y, Some(false));
    window.show();
    window.focus();
}

// toggle the browser window (calculating the position with respect to the tray item)
fn toggle_window(window: &BrowserWindow, tray: &Tray) {
    if window.is_visible() {
        window.hide()
    } else {
        show_window(window, tray);
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    // hide the dock
    app.dock().hide();

    let clo = Closure::wrap(Box::new(|| {
        // create the browser window
        let window = BrowserWindow::new(Some({
            let mut opts = <BrowserWindowOptions as Default>::default();
            opts.set_width(Some(300));
            opts.set_height(Some(450));
            opts.set_show(Some(false));
            opts.set_frame(Some(false));
            opts.set_web_preferences(Some({
                let mut opts = <WebPreferences as Default>::default();
                opts.set_background_throttling(Some(false));
                opts
            }));
            opts
        }));
        // create the tray item
        let tray = {
            let path = path::join(&{
                let val = Array::new();
                val.push(&"..".into());
                val.push(&"assets".into());
                val.push(&"electron-icon.png".into());
                val
            });
            let icon = native_image.create_from_path(&path);
            Tray::new(&icon)
        };
        // configure tray item actions
        {
            let clo = {
                let window = window.clone();
                let tray = tray.clone();
                Closure::wrap(Box::new(move || toggle_window(&window, &tray)) as Box<dyn Fn()>)
            };
            let toggle_window = clo.as_ref().unchecked_ref();
            tray.on("right-click", toggle_window);
            tray.on("double-click", toggle_window);
            tray.on("click", toggle_window);
            clo.forget();
        }
        // load the url for the browser window
        let url = "https://duckduckgo.com".into();
        window.load_url(&url, None);
        // show the browser window when ready
        {
            let clo = {
                let window = window.clone();
                let tray = tray.clone();
                Closure::wrap(Box::new(move || {
                    show_window(&window, &tray);
                }) as Box<dyn Fn()>)
            };
            let ready_to_show = clo.as_ref().unchecked_ref();
            window.once("ready-to-show", ready_to_show);
            clo.forget();
        }
        // hide the browser window when focus is lost
        {
            let clo = {
                let window = window.clone();
                Closure::wrap(Box::new(move || {
                    window.hide();
                }) as Box<dyn Fn()>)
            };
            let blur = clo.as_ref().unchecked_ref();
            window.on("blur", blur);
            clo.forget();
        }
    }) as Box<dyn Fn()>);
    let ready = clo.as_ref().unchecked_ref();
    app.once("ready", ready);
    clo.forget();

    Ok(())
}
