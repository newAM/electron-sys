use js_sys::{Function, JsString, Object};
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Options {
    pub height: u32,
    pub width: u32,
}

#[wasm_bindgen(module = "electron")]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    pub type EventEmitter;

    #[wasm_bindgen(method)]
    pub fn on(this: &EventEmitter, event: JsString, cb: &Function);

    #[wasm_bindgen(extends = EventEmitter)]
    pub type App;

    pub static app: App;

    #[wasm_bindgen(method, js_name = "getAppPath")]
    pub fn get_app_path(this: &App) -> JsString;

    #[wasm_bindgen(extends = EventEmitter)]
    pub type BrowserWindow;

    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<Options>) -> BrowserWindow;

    #[wasm_bindgen(method, js_name = "loadFile")]
    pub fn load_file(this: &BrowserWindow, path: JsString);
}

#[allow(non_upper_case_globals)]
static mut mainWindow: Option<BrowserWindow> = None;

fn create_window() -> Result<(), JsValue> {
    // use std::path::Path;
    let options = Options {
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
