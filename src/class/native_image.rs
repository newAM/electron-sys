use crate::interface::{
    AddRepresentationOptions,
    Rectangle,
    ResizeOptions,
    Size,
    ToBitmapOptions,
    ToDataUrlOptions,
    ToPngOptions,
};
use js_sys::{JsString, Object};
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type NativeImage;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "addRepresentation")]
    pub fn add_representation(this: &NativeImage, options: AddRepresentationOptions);

    #[wasm_bindgen(method)]
    pub fn crop(this: &NativeImage, rectangle: Rectangle) -> NativeImage;

    #[wasm_bindgen(method, js_name = "getAspectRatio")]
    pub fn get_aspect_ratio(this: &NativeImage, rectangle: Rectangle) -> f32;

    #[wasm_bindgen(method, js_name = "getBitmap")]
    pub fn get_bitmap(this: &NativeImage, options: Option<ToBitmapOptions>) -> Buffer;

    #[wasm_bindgen(method, js_name = "getNativeHandle")]
    pub fn get_native_handle(this: &NativeImage) -> Buffer;

    #[wasm_bindgen(method, js_name = "getSize")]
    pub fn get_size(this: &NativeImage) -> Size;

    #[wasm_bindgen(method, js_name = "isEmpty")]
    pub fn is_empty(this: &NativeImage) -> bool;

    #[wasm_bindgen(method)]
    pub fn resize(this: &NativeImage, options: ResizeOptions) -> NativeImage;

    #[wasm_bindgen(method, js_name = "toBitmap")]
    pub fn to_bitmap(this: &NativeImage, options: Option<ToBitmapOptions>) -> Buffer;

    #[wasm_bindgen(method, js_name = "toDataURL")]
    pub fn to_data_url(this: &NativeImage, options: Option<ToDataUrlOptions>) -> JsString;

    #[wasm_bindgen(method, js_name = "toJPEG")]
    pub fn to_jpeg(this: &NativeImage, quality: f32) -> Buffer;

    #[wasm_bindgen(method, js_name = "toPNG")]
    pub fn to_png(this: &NativeImage, options: Option<ToPngOptions>) -> Buffer;

    //*********************//
    // Instance Properties //
    //*********************//

    #[wasm_bindgen(method, getter, js_name = "isMacTemplateImage")]
    pub fn is_mac_template_image(this: &NativeImage) -> bool;

    #[wasm_bindgen(method, setter, js_name = "isMacTemplateImage")]
    pub fn set_is_mac_template_image(this: &NativeImage, value: bool);
}
