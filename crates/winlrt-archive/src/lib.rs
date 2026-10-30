//! winlrt arşiv motoru — Faz 1: ZIP + `.wlrt` iskeleti.

pub mod error;
pub mod profile;
pub mod wlrt;
pub mod zip_io;

pub use error::{ArchiveError, Result};
