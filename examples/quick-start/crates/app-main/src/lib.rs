use electron_sys::{app, global_shortcut, shell, BrowserWindow};
use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};

fn create_window() -> BrowserWindow {
    let win = BrowserWindow::new(Some(&{
        let res = Object::new();
        Reflect::set(&res, &"width".into(), &640.into()).unwrap();
        Reflect::set(&res, &"height".into(), &480.into()).unwrap();
        res
    }));
    win
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let on_ready = Closure::wrap(Box::new(|| {
        let win = create_window();
        // load the html file
        win.load_file(&"..\\..\\..\\index.html".into());
        // change the window title
        win.set_title(&"Hello Electron from Rust! ⚛️🦀🕸🚀".into());
        // register accelerator: Ctrl+Space => opens About panel
        let on_space = Closure::wrap(Box::new(move || {
            shell.beep();
            app.show_about_panel();
        }) as Box<dyn Fn()>);
        global_shortcut.register(&"Ctrl+Space".into(), on_space.as_ref().unchecked_ref());
        on_space.forget();
    }) as Box<dyn Fn()>);
    app.on("ready".into(), on_ready.as_ref().unchecked_ref());
    on_ready.forget();
    Ok(())
}
