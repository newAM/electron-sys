use crate::interface::{Dpi, Margins};
use js_sys::{JsString, Number, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Default)]
pub struct WebContentsPrintOptions {
    collate: Option<bool>,
    color: Option<bool>,
    copies: Option<u32>,
    device_name: Option<JsString>,
    dpi: Option<Dpi>,
    duplex_mode: Option<JsString>,
    footer: Option<JsString>,
    header: Option<JsString>,
    landscape: Option<bool>,
    margins: Option<Margins>,
    page_ranges: Option<Object>,
    pages_per_sheet: Option<u32>,
    print_background: Option<bool>,
    scale_factor: Option<Number>,
    silent: Option<bool>,
}

#[wasm_bindgen]
impl WebContentsPrintOptions {
    #[allow(clippy::too_many_arguments)]
    #[wasm_bindgen(constructor)]
    pub fn new_with_values(
        collate: Option<bool>,
        color: Option<bool>,
        copies: Option<u32>,
        device_name: Option<JsString>,
        dpi: Option<Dpi>,
        duplex_mode: Option<JsString>,
        footer: Option<JsString>,
        header: Option<JsString>,
        landscape: Option<bool>,
        margins: Option<Margins>,
        page_ranges: Option<Object>,
        pages_per_sheet: Option<u32>,
        print_background: Option<bool>,
        scale_factor: Option<Number>,
        silent: Option<bool>,
    ) -> WebContentsPrintOptions {
        WebContentsPrintOptions {
            collate,
            color,
            copies,
            device_name,
            dpi,
            duplex_mode,
            footer,
            header,
            landscape,
            margins,
            page_ranges,
            pages_per_sheet,
            print_background,
            scale_factor,
            silent,
        }
    }

    pub fn new() -> WebContentsPrintOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn collate(&self) -> Option<bool> {
        self.collate
    }

    #[wasm_bindgen(setter)]
    pub fn set_collate(&mut self, value: Option<bool>) {
        self.collate = value;
    }

    #[wasm_bindgen(getter)]
    pub fn color(&self) -> Option<bool> {
        self.color
    }

    #[wasm_bindgen(setter)]
    pub fn set_color(&mut self, value: Option<bool>) {
        self.color = value;
    }

    #[wasm_bindgen(getter)]
    pub fn copies(&self) -> Option<u32> {
        self.copies
    }

    #[wasm_bindgen(setter)]
    pub fn set_copies(&mut self, value: Option<u32>) {
        self.copies = value;
    }

    #[wasm_bindgen(getter)]
    pub fn device_name(&self) -> Option<JsString> {
        self.device_name.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_device_name(&mut self, value: Option<JsString>) {
        self.device_name = value;
    }

    #[wasm_bindgen(getter)]
    pub fn dpi(&self) -> Option<Dpi> {
        self.dpi
    }

    #[wasm_bindgen(setter)]
    pub fn set_dpi(&mut self, value: Option<Dpi>) {
        self.dpi = value;
    }

    #[wasm_bindgen(getter)]
    pub fn duplex_mode(&self) -> Option<JsString> {
        self.duplex_mode.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_duplex_mode(&mut self, value: Option<JsString>) {
        self.duplex_mode = value;
    }

    #[wasm_bindgen(getter)]
    pub fn footer(&self) -> Option<JsString> {
        self.footer.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_footer(&mut self, value: Option<JsString>) {
        self.footer = value;
    }

    #[wasm_bindgen(getter)]
    pub fn header(&self) -> Option<JsString> {
        self.header.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_header(&mut self, value: Option<JsString>) {
        self.header = value;
    }

    #[wasm_bindgen(getter)]
    pub fn landscape(&self) -> Option<bool> {
        self.landscape
    }

    #[wasm_bindgen(setter)]
    pub fn set_landscape(&mut self, value: Option<bool>) {
        self.landscape = value;
    }

    #[wasm_bindgen(getter)]
    pub fn margins(&self) -> Option<Margins> {
        self.margins.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_margins(&mut self, value: Option<Margins>) {
        self.margins = value;
    }

    #[wasm_bindgen(getter)]
    pub fn page_ranges(&self) -> Option<Object> {
        self.page_ranges.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_page_ranges(&mut self, value: Option<Object>) {
        self.page_ranges = value;
    }

    #[wasm_bindgen(getter)]
    pub fn pages_per_sheet(&self) -> Option<u32> {
        self.pages_per_sheet
    }

    #[wasm_bindgen(setter)]
    pub fn set_pages_per_sheet(&mut self, value: Option<u32>) {
        self.pages_per_sheet = value;
    }

    #[wasm_bindgen(getter)]
    pub fn print_background(&self) -> Option<bool> {
        self.print_background
    }

    #[wasm_bindgen(setter)]
    pub fn set_print_background(&mut self, value: Option<bool>) {
        self.print_background = value;
    }

    #[wasm_bindgen(getter)]
    pub fn scale_factor(&self) -> Option<Number> {
        self.scale_factor.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_scale_factor(&mut self, value: Option<Number>) {
        self.scale_factor = value;
    }

    #[wasm_bindgen(getter)]
    pub fn silent(&self) -> Option<bool> {
        self.silent
    }

    #[wasm_bindgen(setter)]
    pub fn set_silent(&mut self, value: Option<bool>) {
        self.silent = value;
    }
}
