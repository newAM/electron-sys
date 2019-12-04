use crate::interface::Size;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
pub struct PrintToPdfOptions {
    landscape: Option<bool>,
    margins_type: Option<usize>,
    page_size: Option<Size>,
    print_background: Option<bool>,
    print_selection_only: Option<bool>,
}

#[wasm_bindgen]
impl PrintToPdfOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        landscape: Option<bool>,
        margins_type: Option<usize>,
        page_size: Option<Size>,
        print_background: Option<bool>,
        print_selection_only: Option<bool>,
    ) -> PrintToPdfOptions {
        PrintToPdfOptions {
            landscape,
            margins_type,
            page_size,
            print_background,
            print_selection_only,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn landscape(&self) -> Option<bool> {
        self.landscape
    }

    #[wasm_bindgen(getter, js_name = "marginsType")]
    pub fn margins_type(&self) -> Option<usize> {
        self.margins_type
    }

    #[wasm_bindgen(getter, js_name = "pageSize")]
    pub fn page_size(&self) -> Option<Size> {
        self.page_size
    }

    #[wasm_bindgen(getter, js_name = "printBackground")]
    pub fn print_background(&self) -> Option<bool> {
        self.print_background
    }

    #[wasm_bindgen(getter, js_name = "printSelectionOnly")]
    pub fn print_selection_only(&self) -> Option<bool> {
        self.print_selection_only
    }
}
