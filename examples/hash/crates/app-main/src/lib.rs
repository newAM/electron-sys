use electron_sys::{app, BrowserWindow, BrowserWindowOptions};
use node_sys::path;
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let on_ready = Closure::wrap(Box::new(|| {
        // create the electron browser window
        let win = BrowserWindow::new(Some({
            let mut opts = <BrowserWindowOptions as Default>::default();
            opts.set_width(Some(800));
            opts.set_height(Some(600));
            opts
        }));
        // set the preloads (for node integration; needed for usage of "crypto" module)
        {
            let preload_path = path::resolve(vec!["preload.js".into()].into_boxed_slice());
            win.web_contents()
                .session()
                .set_preloads(vec![preload_path.into()].into_boxed_slice());
        }
        // load the html file
        win.load_file(&"../../../index.html", None);
    }) as Box<dyn Fn()>);
    app.on("ready", on_ready.as_ref().unchecked_ref());
    on_ready.forget();
    Ok(())
}
