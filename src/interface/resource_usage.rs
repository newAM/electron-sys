use crate::interface::MemoryUsageDetails;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceUsage {
    css_style_sheets: MemoryUsageDetails,
    fonts: MemoryUsageDetails,
    images: MemoryUsageDetails,
    other: MemoryUsageDetails,
    scripts: MemoryUsageDetails,
    xsl_style_sheets: MemoryUsageDetails,
}

#[wasm_bindgen]
impl ResourceUsage {
    #[wasm_bindgen]
    pub fn new(
        css_style_sheets: MemoryUsageDetails,
        fonts: MemoryUsageDetails,
        images: MemoryUsageDetails,
        other: MemoryUsageDetails,
        scripts: MemoryUsageDetails,
        xsl_style_sheets: MemoryUsageDetails,
    ) -> ResourceUsage {
        ResourceUsage {
            css_style_sheets,
            fonts,
            images,
            other,
            scripts,
            xsl_style_sheets,
        }
    }

    #[wasm_bindgen(getter, js_name = "cssStyleSheets")]
    pub fn css_style_sheets(&self) -> MemoryUsageDetails {
        self.css_style_sheets.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_css_style_sheets(&mut self, value: MemoryUsageDetails) {
        self.css_style_sheets = value;
    }

    #[wasm_bindgen(getter)]
    pub fn fonts(&self) -> MemoryUsageDetails {
        self.fonts.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_fonts(&mut self, value: MemoryUsageDetails) {
        self.fonts = value;
    }

    #[wasm_bindgen(getter)]
    pub fn images(&self) -> MemoryUsageDetails {
        self.images.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_images(&mut self, value: MemoryUsageDetails) {
        self.images = value;
    }

    #[wasm_bindgen(getter)]
    pub fn other(&self) -> MemoryUsageDetails {
        self.other.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_other(&mut self, value: MemoryUsageDetails) {
        self.other = value;
    }

    #[wasm_bindgen(getter)]
    pub fn scripts(&self) -> MemoryUsageDetails {
        self.scripts.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_scripts(&mut self, value: MemoryUsageDetails) {
        self.scripts = value;
    }

    #[wasm_bindgen(getter, js_name = "xslStyleSheets")]
    pub fn xsl_style_sheets(&self) -> MemoryUsageDetails {
        self.xsl_style_sheets.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_xsl_style_sheets(&mut self, value: MemoryUsageDetails) {
        self.xsl_style_sheets = value;
    }
}
