//! Raw bindings to the Electron API for projects using wasm-bindgen

#![deny(clippy::all)]
// #![deny(missing_docs)] // FIXME: wasm-bindgen macros break this

pub(crate) mod app;
/// The browser window.
pub mod browser_window;

pub use crate::{app::*, browser_window::*};
