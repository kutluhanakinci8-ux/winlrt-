//! winlrt arşiv motoru — Faz 1: ZIP + `.wlrt` v0.

pub mod bench;
pub mod error;
pub mod profile;
pub mod wlrt;
pub mod zip_io;

pub use error::{ArchiveError, Result};
