//! # trakt-rust
//!
//! A api wrapper in Rust for trakt.tv
//!
//! ## Installation
//!
//! Not yet released but you may add
//! ```toml
//! trakt = { git = "https://github.com/Lichthagel/trakt-rust" }
//! ```
//! to your dependencies
//!
//! ## Examples
//!
//! TODO
//!
//! ## License
//!
//! [MIT][license]
//!
//! [license]: https://github.com/Lichthagel/trakt-rust/blob/master/LICENSE

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod macros;
#[cfg(feature = "async")]
pub mod asyn;
pub mod error;
pub mod extended_info;
pub mod filters;
pub mod models;
pub mod prelude;
pub mod pagination;
pub mod selectors;
#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "sync")]
pub use crate::sync::{Result, TraktApi};
