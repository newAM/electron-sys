use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_name = "GPUFeatureStatus")]
    #[derive(Clone, Debug, PartialEq)]
    pub type GpuFeatureStatus;

    #[wasm_bindgen(method, getter, js_name = "2d_canvas")]
    pub fn twoD_canvas(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter, js_name = "2d_canvas")]
    pub fn set_twoD_canvas(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn flash_3d(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_flash_3d(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn flash_stage3d(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_flash_stage3d(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn flash_stage3d_baseline(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_flash_stage3d_baseline(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn gpu_compositing(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_gpu_compositing(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn multiple_raster_threads(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_multiple_raster_threads(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn native_gpu_memory_buffers(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_native_gpu_memory_buffers(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn rasterization(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_rasterization(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn video_decode(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_video_decode(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn video_encode(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_video_encode(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn vpx_decode(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_vpx_decode(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn webgl(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_webgl(this: &GpuFeatureStatus, value: JsString);

    #[wasm_bindgen(method, getter)]
    pub fn weblg2(this: &GpuFeatureStatus) -> JsString;

    #[wasm_bindgen(method, setter)]
    pub fn set_weblg2(this: &GpuFeatureStatus, value: JsString);
}
