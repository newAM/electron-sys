use electron_sys::{app, BrowserWindow};
use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};

fn create_window() -> Result<BrowserWindow, JsValue> {
    let win = BrowserWindow::new(Some(&{
        let res = Object::new();
        Reflect::set(&res, &"width".into(), &640.into()).unwrap();
        Reflect::set(&res, &"height".into(), &480.into()).unwrap();
        res
    }));
    win.load_file("..\\..\\..\\..\\..\\index.html".into()); // FIXME
    Ok(win)
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let clo = Closure::wrap(Box::new(|| {
        let win = create_window().unwrap();
        win.set_title(&"Hello Electron from Rust! ⚛️🦀🕸🚀".into());
        app.show_about_panel();
    }) as Box<dyn FnMut()>);
    app.on("ready".into(), clo.as_ref().unchecked_ref());
    clo.forget();
    Ok(())
}
