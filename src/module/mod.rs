//! Raw bindings to the Electron API for projects using wasm-bindgen

#![deny(clippy::all)]
// #![deny(missing_docs)] // FIXME: wasm-bindgen macros break this

pub(crate) mod app;
pub(crate) mod auto_updater;
pub(crate) mod browser_window;
pub(crate) mod clipboard;
pub(crate) mod content_tracing;
pub(crate) mod context_bridge;
pub(crate) mod crash_reporter;
pub(crate) mod desktop_capturer;
pub(crate) mod dialog;
pub(crate) mod global_shortcut;
pub(crate) mod in_app_purchase;
pub(crate) mod ipc_main;
pub(crate) mod ipc_renderer;
pub(crate) mod native_image;
pub(crate) mod native_theme;
pub(crate) mod net;
pub(crate) mod net_log;
pub(crate) mod power_monitor;
pub(crate) mod power_save_blocker;
pub(crate) mod process;
pub(crate) mod protocol;
pub(crate) mod remote;
pub(crate) mod screen;
pub(crate) mod session;
pub(crate) mod shell;
pub(crate) mod system_preferences;
pub(crate) mod web_contents;
pub(crate) mod web_frame;

pub use app::*;
pub use browser_window::*;
pub use clipboard::*;
pub use content_tracing::*;
pub use context_bridge::*;
pub use crash_reporter::*;
pub use desktop_capturer::*;
pub use dialog::*;
pub use global_shortcut::*;
pub use in_app_purchase::*;
pub use ipc_main::*;
pub use ipc_renderer::*;
pub use native_image::*;
pub use native_theme::*;
pub use net::*;
pub use net_log::*;
pub use power_monitor::*;
pub use power_save_blocker::*;
pub use process::*;
pub use protocol::*;
pub use remote::*;
pub use screen::*;
pub use session::*;
pub use shell::*;
pub use system_preferences::*;
pub use web_contents::*;
pub use web_frame::*;
