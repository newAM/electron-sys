use electron_sys::{app, global_shortcut, shell, BrowserWindow, BrowserWindowOptions};
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn create_window() -> BrowserWindow {
    BrowserWindow::new(Some({
        let mut opts = <BrowserWindowOptions as Default>::default();
        opts.set_width(Some(640));
        opts.set_height(Some(480));
        opts
    }))
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let on_ready = Closure::wrap(Box::new(|| {
        // create the electron browser window
        let win = create_window();
        // load the html file
        win.load_file(&"..\\..\\..\\index.html", None);
        // change the window title
        win.set_title(&"Hello Electron from Rust! ⚛️🦀🕸🚀");
        // register accelerator: Ctrl+Space => opens About panel
        {
            let on_space = Closure::wrap(Box::new(move || {
                shell.beep();
                app.show_about_panel();
            }) as Box<dyn Fn()>);
            global_shortcut.register(&"Ctrl+Space", on_space.as_ref().unchecked_ref());
            on_space.forget();
        }
        // open the dev tools panel (undocked)
        {
            let activate = Some(false);
            let mode = "undocked".into();
            let options = Some(electron_sys::OpenDevToolsOptions::new(activate, mode));
            win.web_contents().open_dev_tools(options);
        }
    }) as Box<dyn Fn()>);
    app.on("ready", on_ready.as_ref().unchecked_ref());
    on_ready.forget();
    Ok(())
}
