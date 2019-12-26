use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartConfiguration {
    kind: Option<JsString>,
    data: Option<ChartData>,
    options: Option<ChartOptions>,
}

#[wasm_bindgen]
impl ChartConfiguration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartConfiguration {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "type")]
    pub fn kind(&self) -> Option<JsString> {
        self.kind.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_kind(&mut self, value: Option<JsString>) {
        self.kind = value;
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Option<ChartData> {
        self.data.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, value: Option<ChartData>) {
        self.data = value;
    }

    #[wasm_bindgen(getter)]
    pub fn options(&self) -> Option<ChartOptions> {
        self.options.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_options(&mut self, value: Option<ChartOptions>) {
        self.options = value;
    }
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartData {
    labels: Option<Box<[JsValue]>>,
    datasets: Option<Box<[JsValue]>>,
}

#[wasm_bindgen]
impl ChartData {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartData {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn labels(&self) -> Option<Box<[JsValue]>> {
        self.labels.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_labels(&mut self, value: Option<Box<[JsValue]>>) {
        self.labels = value;
    }

    #[wasm_bindgen(getter)]
    pub fn datasets(&self) -> Option<Box<[JsValue]>> {
        self.datasets.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_datasets(&mut self, value: Option<Box<[JsValue]>>) {
        self.datasets = value;
    }
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartLegendOptions {
    display: Option<bool>,
    labels: Option<ChartLegendLabelOptions>,
}

#[wasm_bindgen]
impl ChartLegendOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartLegendOptions {
        Default::default()
    }

    #[wasm_bindgen(getter)]
    pub fn display(&self) -> Option<bool> {
        self.display
    }

    #[wasm_bindgen(setter)]
    pub fn set_display(&mut self, value: Option<bool>) {
        self.display = value;
    }

    #[wasm_bindgen(getter)]
    pub fn labels(&self) -> Option<ChartLegendLabelOptions> {
        self.labels.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_labels(&mut self, value: Option<ChartLegendLabelOptions>) {
        self.labels = value;
    }
}

#[wasm_bindgen]
#[derive(Debug, Default, Clone)]
pub struct ChartLegendLabelOptions {
    font_color: Option<JsString>,
    font_size: Option<u8>,
}

#[wasm_bindgen]
impl ChartLegendLabelOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChartLegendLabelOptions {
        Default::default()
    }

    #[wasm_bindgen(getter, js_name = "fontColor")]
    pub fn font_color(&self) -> Option<JsString> {
        self.font_color.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_font_color(&mut self, value: Option<JsString>) {
        self.font_color = value;
    }

    #[wasm_bindgen(getter, js_name = "fontSize")]
    pub fn font_size(&self) -> Option<u8> {
        self.font_size
    }

    #[wasm_bindgen(setter)]
    pub fn set_font_size(&mut self, value: Option<u8>) {
        self.font_size = value;
    }
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone)]
    pub type ChartTitleOptions;

    #[wasm_bindgen(method, getter)]
    pub fn display(this: &ChartTitleOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter)]
    pub fn set_display(this: &ChartTitleOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter, js_name = "fontColor")]
    pub fn font_color(this: &ChartTitleOptions) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_font_color(this: &ChartTitleOptions, value: Option<JsString>);

    #[wasm_bindgen(method, getter, js_name = "fontSize")]
    pub fn font_size(this: &ChartTitleOptions) -> Option<u8>;

    #[wasm_bindgen(method, setter, js_name = "fontSize")]
    pub fn set_font_size(this: &ChartTitleOptions, value: Option<u8>);

    #[wasm_bindgen(method, getter)]
    pub fn text(this: &ChartTitleOptions) -> Option<JsString>;

    #[wasm_bindgen(method, setter)]
    pub fn set_text(this: &ChartTitleOptions, value: Option<JsString>);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone)]
    pub type ChartOptions;

    #[wasm_bindgen(method, getter)]
    pub fn legend(this: &ChartOptions) -> Option<ChartLegendOptions>;

    #[wasm_bindgen(method, setter)]
    pub fn set_legend(this: &ChartOptions, value: Option<ChartLegendOptions>);

    #[wasm_bindgen(method, getter, js_name = "maintainAspectRatio")]
    pub fn maintain_aspect_ratio(this: &ChartOptions) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "maintainAspectRatio")]
    pub fn set_maintain_aspect_ratio(this: &ChartOptions, value: Option<bool>);

    #[wasm_bindgen(method, getter)]
    pub fn title(this: &ChartOptions) -> Option<ChartTitleOptions>;

    #[wasm_bindgen(method, setter)]
    pub fn set_title(this: &ChartOptions, value: Option<ChartTitleOptions>);
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone)]
    pub type ChartDataSets;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &ChartDataSets) -> Option<Box<[JsValue]>>;

    #[wasm_bindgen(method, setter)]
    pub fn set_data(this: &ChartDataSets, value: Option<Box<[JsValue]>>);

    #[wasm_bindgen(method, getter, js_name = "backgroundColor")]
    pub fn background_color(this: &ChartDataSets) -> Option<Box<[JsValue]>>;

    #[wasm_bindgen(method, setter, js_name = "backgroundColor")]
    pub fn set_background_color(this: &ChartDataSets, value: Option<Box<[JsValue]>>);
}

#[wasm_bindgen]
extern {
    #[derive(Debug, Clone)]
    pub type Chart;

    #[wasm_bindgen(constructor)]
    pub fn new(context: &HtmlCanvasElement, options: ChartConfiguration) -> Chart;

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &Chart) -> ChartData;

    #[wasm_bindgen(method)]
    pub fn update(this: &Chart);
}
