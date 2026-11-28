//! TMATS library foundations; parsing and other TMATS APIs are not implemented.
//!
//! The retired prototype is preserved at the repository's `prototype-0` tag.
//! The required `std` feature is on by default; this is not a `no_std` promise.

/// Library version, shared with the CLI through the workspace manifest.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

mod span;

pub use span::Span;
