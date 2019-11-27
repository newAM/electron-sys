use crate::interface::FeedUrlOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "electron")]
extern {
    #[wasm_bindgen]
    pub type AutoUpdater;

    #[wasm_bindgen(js_name = "autoUpdater")]
    pub static auto_updater: AutoUpdater;

    /// Asks the server whether there is an update. You must call `set_feed_url` before using this
    /// API.
    #[wasm_bindgen(method, js_name = "checkForUpdates")]
    pub fn check_for_updates(this: &AutoUpdater);

    /// The current update feed URL.
    #[wasm_bindgen(method, js_name = "getFeedURL")]
    pub fn get_feed_url(this: &AutoUpdater);

    /// Restarts the app and installs the update after it has been downloaded. It should only be
    /// called after `update-downloaded` has been emitted.
    #[wasm_bindgen(method, js_name = "quitAndInstall")]
    pub fn quit_and_install(this: &AutoUpdater);

    /// Sets the `url` and initialize the auto updater.
    #[wasm_bindgen(method, js_name = "setFeedURL")]
    pub fn set_feed_url(this: &AutoUpdater, options: FeedUrlOptions);
}
