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
            opts.set_width(Some(640));
            opts.set_height(Some(480));
            opts
        }));
        // set the preloads
        {
            let preload_path = path::resolve(vec!["preload.js".into()].into_boxed_slice());
            win.web_contents()
                .session()
                .set_preloads(vec![preload_path.into()].into_boxed_slice());
        }
        // load the html file
        win.load_file(&"../../../index.html".into(), None);
        // open the dev tools panel (undocked)
        // {
        //     let activate = Some(false);
        //     let mode = "undocked".into();
        //     let options = Some(electron_sys::OpenDevToolsOptions::new(activate, mode));
        //     win.web_contents().open_dev_tools(options);
        // }
    }) as Box<dyn Fn()>);
    app.on("ready".into(), on_ready.as_ref().unchecked_ref());
    on_ready.forget();
    Ok(())
}
