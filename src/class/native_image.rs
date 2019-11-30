use crate::interface::{
    AddRepresentationOptions,
    CreateFromBitmapOptions,
    CreateFromBufferOptions,
    Rectangle,
    ResizeOptions,
    Size,
    ToBitmapOptions,
    ToDataUrlOptions,
    ToPngOptions,
};
use js_sys::{Array, JsString, Number, Object};
use node_sys::Buffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub type NativeImage;

    //****************//
    // Static Methods //
    //****************//

    #[wasm_bindgen(static_method_of = NativeImage, js_name = "createEmpty")]
    pub fn create_empty() -> NativeImage;

    #[wasm_bindgen(static_method_of = NativeImage, js_name = "createFromBitmap")]
    pub fn create_from_bitmap(buffer: &Buffer, options: CreateFromBitmapOptions) -> NativeImage;

    #[wasm_bindgen(static_method_of = NativeImage, js_name = "createFromBuffer")]
    pub fn create_from_buffer(buffer: &Buffer, options: Option<CreateFromBufferOptions>) -> NativeImage;

    #[wasm_bindgen(static_method_of = NativeImage, js_name = "createFromDataURL")]
    pub fn create_from_data_url(data_url: &JsString) -> NativeImage;

    #[wasm_bindgen(static_method_of = NativeImage, js_name = "createFromNamedImage")]
    pub fn create_from_named_image(image_name: &JsString, hsl_shift: Option<Array>) -> NativeImage;

    #[wasm_bindgen(static_method_of = NativeImage, js_name = "createFromPath")]
    pub fn create_from_path(path: &JsString) -> NativeImage;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "addRepresentation")]
    pub fn add_representation(this: &NativeImage, options: AddRepresentationOptions);

    #[wasm_bindgen(method)]
    pub fn crop(this: &NativeImage, rect: &Rectangle) -> NativeImage;

    #[wasm_bindgen(method, js_name = "getAspectRatio")]
    pub fn get_aspect_ratio(this: &NativeImage, rect: &Rectangle) -> Number;

    #[wasm_bindgen(method, js_name = "getBitmap")]
    pub fn get_bitmap(this: &NativeImage, options: Option<ToBitmapOptions>) -> Number;

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
    pub fn to_jpeg(this: &NativeImage, quality: &Number) -> Buffer;

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
