//! All models related to [collections]
//!
//! [collections]: https://trakt.docs.apiary.io/#reference/users/collection
use crate::models::{Movie, Show};
use chrono::{DateTime, Utc};

/// A movie in a [collection] of a user.
///
/// [collection]: https://trakt.docs.apiary.io/#reference/users/collection/get-collection
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionMovie {
    pub collected_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub movie: Movie,
}

/// A show in a [collection] of a user.
///
/// [collection]: https://trakt.docs.apiary.io/#reference/users/collection/get-collection
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionShow {
    pub last_collected_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub show: Show,
    pub seasons: Vec<CollectionSeason>,
}

/// A season in a [collection] of a user.
///
/// [collection]: https://trakt.docs.apiary.io/#reference/users/collection/get-collection
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionSeason {
    pub number: u32,
    pub episodes: Vec<CollectionEpisode>,
}

/// An episode in a [collection] of a user.
///
/// [collection]: https://trakt.docs.apiary.io/#reference/users/collection/get-collection
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionEpisode {
    pub number: u32,
    pub collected_at: DateTime<Utc>,
}
