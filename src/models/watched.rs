use crate::models::{Movie, Show};
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
