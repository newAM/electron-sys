use crate::{
    class::NativeImage,
    interface::{CreateFromBitmapOptions, CreateFromBufferOptions},
};
use js_sys::JsString;
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type NativeImageModule;

    #[wasm_bindgen(js_name = "nativeImage")]
    pub static native_image: NativeImageModule;

    #[wasm_bindgen(method, js_name = "createEmpty")]
    pub fn create_empty(this: &NativeImageModule) -> NativeImage;

    #[wasm_bindgen(method, js_name = "createFromBitmap")]
    pub fn create_from_bitmap(
        this: &NativeImageModule,
        buffer: &Buffer,
        options: CreateFromBitmapOptions,
    ) -> NativeImage;

    #[wasm_bindgen(method, js_name = "createFromBuffer")]
    pub fn create_from_buffer(
        this: &NativeImageModule,
        buffer: &Buffer,
        options: Option<CreateFromBufferOptions>,
    ) -> NativeImage;

    #[wasm_bindgen(method, js_name = "createFromDataURL")]
    pub fn create_from_data_url(this: &NativeImageModule, data_url: &JsString) -> NativeImage;

    #[wasm_bindgen(method, js_name = "createFromNamedImage")]
    pub fn create_from_named_image(
        this: &NativeImageModule,
        image_name: &JsString,
        hsl_shift: Option<Box<[JsValue]>>,
    ) -> NativeImage;

    #[wasm_bindgen(method, js_name = "createFromPath")]
    pub fn create_from_path(this: &NativeImageModule, path: &JsString) -> NativeImage;
}
