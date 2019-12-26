use crate::chart::{
    Chart,
    ChartConfiguration,
    ChartData,
    ChartDataSets,
    ChartLegendLabelOptions,
    ChartLegendOptions,
    ChartOptions,
    ChartTitleOptions,
};
use js_sys::Function;
use node_sys::{os, CpuInfo};
use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlCanvasElement;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    static CHART: Chart = {
        let document = web_sys::window().unwrap_throw().document().unwrap_throw();
        let context = document
            .get_element_by_id("text-input")
            .unwrap_throw()
            .unchecked_into::<HtmlCanvasElement>();
        let options = {
            let mut options = ChartConfiguration::new();
            options.set_kind(Some("doughnut".into()));
            options.set_data(Some({
                let mut data = ChartData::new();
                data.set_labels(Some(vec![
                    "User Time (ms)".into(),
                    "System Time (ms)".into(),
                    "Idle Time (ms)".into(),
                ].into_boxed_slice()));
                data.set_datasets(Some(get_datasets()));
                data
            }));
            options.set_options(Some({
                let mut options = ChartOptions::new();
                options.set_maintain_aspect_ratio(Some(false));
                options.set_title(Some({
                    let mut title = ChartTitleOptions::new();
                    title.set_display(Some(true));
                    title.set_text(Some("CPU Activity".into()));
                    title.set_font_color(Some("rgb(250, 250, 250)".into()));
                    title.set_font_size(Some(16u8));
                    title
                }));
                options.set_legend(Some({
                    let mut legend = ChartLegendOptions::new();
                    legend.set_display(Some(true));
                    legend.set_labels(Some({
                        let mut labels = ChartLegendLabelOptions::new();
                        labels.set_font_color(Some("rgb(250, 250, 250)".into()));
                        labels.set_font_size(Some(12u8));
                        labels
                    }));
                    legend
                }));
                options
            }));
            options
        };
        Chart::new(&context, options)
    };
}

thread_local! {
    static LAST_MEASURE_TIMES: RefCell<Vec<[f64; 3]>> = RefCell::new(vec![]);
}

#[allow(dead_code)]
fn get_cpus() -> impl Iterator<Item = CpuInfo> {
    os::cpus().into_vec().into_iter().map(JsCast::unchecked_into)
}

#[allow(dead_code)]
fn set_last_measure_times() {
    LAST_MEASURE_TIMES.with(|times| {
        let mut times = times.borrow_mut();
        for (i, cpu) in get_cpus().enumerate() {
            times[i] = get_cpu_times(&cpu);
    }
    });
}

#[allow(dead_code)]
fn get_datasets() -> Box<[JsValue]> {
    let mut datasets = vec![];
    for cpu in get_cpus() {
        let cpu_data = ChartDataSets::new();
        cpu_data.set_data(Some(
            get_cpu_times(&cpu)
                .to_vec()
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ));
        cpu_data.set_background_color(Some(
            vec![
                "rgba(255,  99, 132, 1)".into(),
                "rgba( 54, 162, 235, 1)".into(),
                "rgba(255, 206,  86, 1)".into(),
            ]
            .into_boxed_slice(),
        ));
        datasets.push(cpu_data.into());
}
    datasets.into_boxed_slice()
}

fn update_datasets() {
    CHART.with(|chart| {
        LAST_MEASURE_TIMES.with(|times| {
            let times = times.borrow();
            for (i, cpu) in get_cpus().enumerate() {
                let mut data = get_cpu_times(&cpu);
                data[0] -= times[i][0];
                data[1] -= times[i][1];
                data[2] -= times[i][2];
                let data = data
                    .to_vec()
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                let cpu_data = &chart.data().datasets().unwrap_throw()[i];
                let cpu_data = cpu_data.unchecked_ref::<ChartDataSets>();
                cpu_data.set_data(Some(data));
            }
        });
        chart.update();
        set_last_measure_times();
    });
}

#[allow(dead_code)]
fn get_cpu_times(cpu: &CpuInfo) -> [f64; 3] {
    let times = cpu.times();
    [times.user(), times.sys(), times.idle()]
}

#[allow(dead_code)]
fn draw_chart() {
    let window = web_sys::window().unwrap_throw();
    let clo = Closure::wrap(Box::new(update_datasets) as Box<dyn Fn()>);
    let handler = clo.as_ref().unchecked_ref::<Function>();
    let timeout = 1000;
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(handler, timeout)
        .unwrap_throw();
    clo.forget();
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    set_last_measure_times();
    draw_chart();
    Ok(())
}
