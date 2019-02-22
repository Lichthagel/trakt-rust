//! # trakt-rust
//!
//! A api wrapper in Rust for trakt.tv.
//! [README]
//!
//! ## License
//!
//! [MIT][license]
//!
//! [license]: https://github.com/Lichthagel/trakt-rust/blob/master/LICENSE
//! [README]: https://github.com/Lichthagel/trakt-rust/blob/master/README.md

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate serde;
#[cfg(test)]
#[macro_use]
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
