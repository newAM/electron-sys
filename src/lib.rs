//! Raw bindings to the Electron API for projects using wasm-bindgen

#![deny(clippy::all)]
// #![deny(missing_docs)] // FIXME: wasm-bindgen macros break this

pub(crate) mod module;
pub(crate) mod interface;
pub use module::*;
pub use interface::*;
