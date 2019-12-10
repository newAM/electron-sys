use crate::{
    class::BrowserWindow,
    interface::{
        CertificateTrustDialogOptions,
        MessageBoxOptions,
        MessageBoxSyncOptions,
        OpenDialogOptions,
        OpenDialogSyncOptions,
        SaveDialogOptions,
        SaveDialogSyncOptions,
    },
};
use js_sys::{JsString, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type Dialog;

    pub static dialog: Dialog;

    #[must_use]
    #[wasm_bindgen(method, js_name = "showCertificateTrustDialog")]
    pub fn show_certificate_trust_dialog(
        this: &Dialog,
        browser_window: Option<&BrowserWindow>, // FIXME: verify this works with Option<_> for overloads
        options: CertificateTrustDialogOptions,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = "showErrBox")]
    pub fn show_err_box(this: &Dialog, title: &JsString, content: &JsString);

    #[must_use]
    #[wasm_bindgen(method, js_name = "showMessageBox")]
    pub fn show_message_box(
        this: &Dialog,
        browser_window: Option<&BrowserWindow>,
        options: MessageBoxOptions,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = "showMessageBoxSync")]
    pub fn show_message_box_sync(
        this: &Dialog,
        browser_window: Option<&BrowserWindow>,
        options: MessageBoxSyncOptions,
    ) -> usize;

    #[must_use]
    #[wasm_bindgen(method, js_name = "showOpenDialog")]
    pub fn show_open_dialog(
        this: &Dialog,
        browser_window: Option<&BrowserWindow>,
        options: OpenDialogOptions,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = "showOpenDialogSync")]
    pub fn show_open_dialog_sync(
        this: &Dialog,
        browser_window: Option<&BrowserWindow>,
        options: OpenDialogSyncOptions,
    ) -> Option<Box<[JsValue]>>;

    #[must_use]
    #[wasm_bindgen(method, js_name = "showSaveDialog")]
    pub fn show_save_dialog(
        this: &Dialog,
        browser_window: Option<&BrowserWindow>,
        options: SaveDialogOptions,
    ) -> Promise;

    #[wasm_bindgen(method, js_name = "showSaveDialogSync")]
    pub fn show_save_dialog_sync(
        this: &Dialog,
        browser_window: Option<&BrowserWindow>,
        options: SaveDialogSyncOptions,
    ) -> Option<JsString>;
}
