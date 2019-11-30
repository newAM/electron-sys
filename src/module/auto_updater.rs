use crate::interface::FeedUrlOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type AutoUpdater;

    #[wasm_bindgen(js_name = "autoUpdater")]
    pub static auto_updater: AutoUpdater;

    //******************//
    // Instance Methods //
    //******************//

    #[wasm_bindgen(method, js_name = "checkForUpdates")]
    pub fn check_for_updates(this: &AutoUpdater);

    #[wasm_bindgen(method, js_name = "getFeedURL")]
    pub fn get_feed_url(this: &AutoUpdater);

    #[wasm_bindgen(method, js_name = "quitAndInstall")]
    pub fn quit_and_install(this: &AutoUpdater);

    #[wasm_bindgen(method, js_name = "setFeedURL")]
    pub fn set_feed_url(this: &AutoUpdater, options: FeedUrlOptions);
}
