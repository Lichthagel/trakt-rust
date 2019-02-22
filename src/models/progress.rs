use chrono::{DateTime, Utc};

/// Progress of how many items a user collected from a show.
/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/shows/collection-progress/get-show-collection-progress
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgress {
    pub aired: u32,
    pub completed: u32,
    pub last_collected_at: Option<DateTime<Utc>>,
    pub seasons: Vec<CollectionProgressSeason>,
}

/// Collection progress of a season in [CollectionProgress]
///
/// [CollectionProgress]: struct.CollectionProgress.html
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgressSeason {
    pub number: u32,
    pub aired: u32,
    pub completed: u32,
    pub episodes: Vec<CollectionProgressEpisode>,
}

/// Collection progress of a episode in [CollectionProgressSeason]
///
/// [CollectionProgressSeason] struct.CollectionProgressSeason.html
#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgressEpisode {
    pub number: u32,
    pub completed: bool,
    pub collected_at: Option<DateTime<Utc>>,
}

/// Progress of how many items a user watched from a show.
/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/shows/watched-progress/get-show-watched-progress
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgress {
    pub aired: u32,
    pub completed: u32,
    pub last_watched_at: Option<DateTime<Utc>>,
    pub reset_at: Option<DateTime<Utc>>,
    pub seasons: Vec<WatchedProgressSeason>,
}

/// Watched progress of a season in [WatchedProgress]
///
/// [WatchedProgress]: struct.WatchedProgress.html
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgressSeason {
    pub number: u32,
    pub aired: u32,
    pub completed: u32,
    pub episodes: Vec<WatchedProgressEpisode>,
}

/// Watched progress of a episode in [WatchedProgressSeason]
///
/// [WatchedProgressSeason]: struct.WatchedProgressSeason.html
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgressEpisode {
    pub number: u32,
    pub completed: bool,
    pub last_watched_at: Option<DateTime<Utc>>,
}
