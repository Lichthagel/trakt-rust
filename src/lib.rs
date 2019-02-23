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
pub mod pagination;
pub mod prelude;
pub mod selectors;
#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "sync")]
pub use crate::sync::{Result, TraktApi};

#[cfg(test)]
mod tests {
    use mockito::Mock;

    pub fn mock(method: &str, path: &str, client_id: &str) -> Mock {
        mockito::mock(method, path)
            .with_header("trakt-api-version", "2")
            .with_header("trakt-api-key", client_id)
            .with_header("Content-Type", "application/json")
    }

    pub fn auth_mock(method: &str, path: &str, client_id: &str, access_token: &str) -> Mock {
        mock(method, path, client_id)
            .with_header("Authorization", &format!("Bearer {}", access_token))
    }

}
