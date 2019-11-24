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

pub(crate) mod net_log;

pub(crate) mod net;

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

pub use crate::{
    app::*,
    browser_window::*,
    clipboard::*,
    content_tracing::*,
    context_bridge::*,
    crash_reporter::*,
    desktop_capturer::*,
    dialog::*,
    global_shortcut::*,
    in_app_purchase::*,
    ipc_main::*,
    ipc_renderer::*,
    native_image::*,
    native_theme::*,
    net::*,
    net_log::*,
    power_monitor::*,
    power_save_blocker::*,
    process::*,
    protocol::*,
    remote::*,
    screen::*,
    session::*,
    shell::*,
    system_preferences::*,
    web_contents::*,
    web_frame::*,
};
