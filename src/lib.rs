mod client;
mod error;
mod types;

#[cfg(feature = "wasm")]
mod wasm;

pub use client::DiscourseClient;
pub use error::{Error, Result};
pub use types::*;

#[cfg(feature = "wasm")]
pub use wasm::WasmDiscourseClient;
