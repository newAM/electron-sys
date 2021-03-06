use electron_sys::{app, BrowserWindow, BrowserWindowOptions, WebPreferences};
use node_sys::path;
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let ready = Closure::wrap(Box::new(|| {
        // create the electron browser window
        let win = BrowserWindow::new(Some({
            let opts = <BrowserWindowOptions as Default>::default();
            opts.set_width(Some(500));
            opts.set_height(Some(400));
            opts.set_title_bar_style(Some("hiddenInset".into()));
            opts.set_background_color(Some("#111".into()));
            opts.set_show(Some(false));
            opts.set_web_preferences(Some({
                let prefs = <WebPreferences as Default>::default();
                prefs.set_preload(Some(path::resolve(vec!["preload.js".into()].into_boxed_slice())));
                prefs
            }));
            opts
        }));
        // load the html file
        win.load_file(&"../../../index.html", None);
        // show window when ready
        let ready_to_show = {
            let win = win.clone();
            Closure::wrap(Box::new(move || {
                win.show();
            }) as Box<dyn Fn()>)
        };
        win.once("ready-to-show", ready_to_show.as_ref().unchecked_ref());
        ready_to_show.forget();
    }) as Box<dyn Fn()>);
    app.once("ready", ready.as_ref().unchecked_ref());
    ready.forget();
    Ok(())
}
