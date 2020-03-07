use electron_sys::{app, BrowserWindow, BrowserWindowOptions, WebPreferences};
use node_sys::path;
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let on_ready = Closure::wrap(Box::new(|| {
        // create the electron browser window
        let win = BrowserWindow::new(Some({
            let opts = <BrowserWindowOptions as Default>::default();
            opts.set_width(Some(800));
            opts.set_height(Some(600));
            opts.set_web_preferences(Some({
                let prefs = <WebPreferences as Default>::default();
                prefs.set_preload(Some(path::resolve(vec!["preload.js".into()].into_boxed_slice())));
                prefs
            }));
            opts
        }));
        // load the html file
        win.load_file(&"../../../index.html", None);
    }) as Box<dyn Fn()>);
    app.on("ready", on_ready.as_ref().unchecked_ref());
    on_ready.forget();
    Ok(())
}
