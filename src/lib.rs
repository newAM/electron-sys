//! Raw bindings to the Electron API for projects using wasm-bindgen

#![deny(clippy::all)]
// #![deny(missing_docs)] // FIXME: wasm-bindgen macros break this

pub(crate) mod app;

pub(crate) mod browser_window;

pub(crate) mod global_shortcut;

pub use crate::{app::*, browser_window::*, global_shortcut::*};
