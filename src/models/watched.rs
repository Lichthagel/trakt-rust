use crate::extended_info::{WithFull, WithNone};
use crate::models::{FullMovie, FullShow, Movie, Show};
use chrono::{DateTime, Utc};

/// A [watched entry] of an user
///
/// [watched entry]: https://trakt.docs.apiary.io/#reference/users/watched/get-watched
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedEntry {
    pub plays: u32,
    pub last_watched_at: DateTime<Utc>,
    pub last_updated_at: Option<DateTime<Utc>>,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
    pub seasons: Option<Vec<WatchedSeason>>,
}

impl WithFull for WatchedEntry {
    type Full = FullWatchedEntry;
}

/// A [watched entry] of an user with full [extended info]
///
/// [watched entry]: https://trakt.docs.apiary.io/#reference/users/watched/get-watched
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullWatchedEntry {
    pub plays: u32,
    pub last_watched_at: DateTime<Utc>,
    pub last_updated_at: Option<DateTime<Utc>>,
    pub movie: Option<FullMovie>,
    pub show: Option<FullShow>,
    pub seasons: Option<Vec<WatchedSeason>>,
}

impl WithNone for FullWatchedEntry {
    type None = WatchedEntry;
}

/// A watched season in [WatchedEntry]
///
/// [WatchedEntry]: struct.WatchedEntry.html
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedSeason {
    pub number: u32,
    pub episodes: Vec<WatchedEpisode>,
}

/// A watched episode in [WatchedSeason]
///
/// [WatchedSeason]: struct.WatchedSeason.html
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedEpisode {
    pub number: u32,
    pub plays: u32,
    pub last_watched_at: DateTime<Utc>,
}
