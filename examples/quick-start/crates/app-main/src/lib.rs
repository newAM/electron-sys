use electron_sys::{app, browser_window, BrowserWindow};
use wasm_bindgen::prelude::*;

#[allow(non_upper_case_globals)]
static mut mainWindow: Option<BrowserWindow> = None;

fn create_window() -> Result<(), JsValue> {
    let options = browser_window::Options {
        height: 800,
        width: 600,
    };
    let window = BrowserWindow::new(Some(options));
    window.load_file("..\\..\\..\\..\\..\\index.html".into()); // FIXME
    unsafe { mainWindow = Some(window) };
    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use wasm_bindgen::JsCast;
    console_error_panic_hook::set_once();
    let clo = Closure::wrap(Box::new(|| {
        create_window().unwrap();
    }) as Box<dyn FnMut()>);
    app.on("ready".into(), clo.as_ref().unchecked_ref());
    clo.forget();
    Ok(())
}
