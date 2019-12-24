use electron_sys::{app, process, BrowserWindow, BrowserWindowOptions, WebPreferences};
use js_sys::JsString;
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let ready = Closure::wrap(Box::new(|| {
        // Create a new window
        let window = BrowserWindow::new(Some({
            let mut opts = <BrowserWindowOptions as Default>::default();
            // Set the initial width to 800px
            opts.set_width(Some(640));
            // Set the initial height to 600px
            opts.set_height(Some(480));
            // Don't show the window until it ready, this prevents any white flickering
            opts.set_show(Some(false));
            opts.set_web_preferences(Some({
                let mut opts = <WebPreferences as Default>::default();
                // Disable node integration in remote page
                opts.set_node_integration(Some(false));
                opts
            }));
            opts
        }));

        // URL is argument to npm start
        let default = JsString::from("https://duckduckgo.com").into();
        let args = process.argv();
        let url = args.get(2).unwrap_or(&default).unchecked_ref::<JsString>();
        window.load_url(String::from(url).as_str(), None);

        let ready_to_show = {
            let window = window.clone();
            Closure::wrap(Box::new(move || {
                window.maximize();
                window.show();
            }) as Box<dyn Fn()>)
        };
        // Show window when page is ready
        window.once("ready-to-show", ready_to_show.as_ref().unchecked_ref());
        ready_to_show.forget();
    }) as Box<dyn Fn()>);

    // Wait until the app is ready
    app.on("ready", ready.as_ref().unchecked_ref());
    ready.forget();

    Ok(())
}
