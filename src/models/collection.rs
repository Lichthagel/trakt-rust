//! All models related to [collections]
//!
//! [collections]: https://trakt.docs.apiary.io/#reference/users/collection
use crate::extended_info::{WithFull, WithNone};
use crate::models::{FullMovie, FullShow, Movie, Show};
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

impl WithFull for CollectionMovie {
    type Full = FullCollectionMovie;
}

/// A movie in a [collection] of a user with full [extended info].
///
/// [collection]: https://trakt.docs.apiary.io/#reference/users/collection/get-collection
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullCollectionMovie {
    pub collected_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub movie: FullMovie,
}

impl WithNone for FullCollectionMovie {
    type None = CollectionMovie;
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

impl WithFull for CollectionShow {
    type Full = FullCollectionShow;
}
/// A show in a [collection] of a user with full [extended info].
///
/// [collection]: https://trakt.docs.apiary.io/#reference/users/collection/get-collection
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullCollectionShow {
    pub last_collected_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub show: FullShow,
    pub seasons: Vec<CollectionSeason>,
}

impl WithNone for FullCollectionShow {
    type None = CollectionShow;
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
