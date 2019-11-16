use electron_sys::{app, BrowserWindow};
use js_sys::{Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};

fn create_window() -> Result<(), JsValue> {
    let window = BrowserWindow::new(Some(&{
        let res = Object::new();
        Reflect::set(&res, &"height".into(), &800.into()).unwrap();
        Reflect::set(&res, &"width".into(), &600.into()).unwrap();
        res
    }));
    window.load_file("..\\..\\..\\..\\..\\index.html".into()); // FIXME
    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let clo = Closure::wrap(Box::new(|| {
        create_window().unwrap();
    }) as Box<dyn FnMut()>);
    app.on("ready".into(), clo.as_ref().unchecked_ref());
    clo.forget();
    Ok(())
}
