//! All models related to [networks]
//!
//! [networks]: https://trakt.docs.apiary.io/#reference/networks

/// A [network]
///
/// [network]: https://trakt.docs.apiary.io/#reference/networks
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Network {
    pub name: String,
}
