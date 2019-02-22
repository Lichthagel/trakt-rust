//! Models used in sync requests and responses.
//! [API docs]
//!
//! [API docs]: https://trakt.docs.apiary.io/#reference/sync
use crate::models::{
    Episode, Movie, OptionEpisode, OptionMovie, OptionSeason, OptionShow, Show, WatchableType,
};
use chrono::{DateTime, Utc};

/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/sync/last-activities/get-last-activity
#[derive(Debug, Serialize, Deserialize)]
pub struct LastActivities {
    all: DateTime<Utc>,
    movies: LastActivitiesElement,
    episodes: LastActivitiesElement,
    shows: LastActivitiesElement,
    seasons: LastActivitiesElement,
    comments: LastActivitiesElement,
    lists: LastActivitiesElement,
}

/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/sync/last-activities/get-last-activity
#[derive(Debug, Serialize, Deserialize)]
pub struct LastActivitiesElement {
    watched_at: Option<DateTime<Utc>>,
    collected_at: Option<DateTime<Utc>>,
    rated_at: Option<DateTime<Utc>>,
    watchlisted_at: Option<DateTime<Utc>>,
    commented_at: Option<DateTime<Utc>>,
    paused_at: Option<DateTime<Utc>>,
    hidden_at: Option<DateTime<Utc>>,
    liked_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/sync/playback/get-playback-progress
#[derive(Debug, Serialize, Deserialize)]
pub struct Playback {
    progress: f64,
    paused_at: Option<DateTime<Utc>>,
    id: u64,
    #[serde(rename = "type")]
    item_type: WatchableType,
    movie: Option<Movie>,
    episode: Option<Episode>,
    show: Option<Show>,
}

/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/sync/add-to-collection/add-items-to-collection
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncAddResponse {
    added: SyncResponseNumbers,
    updated: Option<SyncResponseNumbers>,
    existing: Option<SyncResponseNumbers>,
    not_found: SyncResponseNotFound,
}

/// [API docs]
///
/// [API docs]: https://trakt.docs.apiary.io/#reference/sync/remove-from-collection/remove-items-from-collection
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRemoveResponse {
    deleted: SyncResponseNumbers,
    not_found: SyncResponseNotFound,
}

/// Number of affected entries in [SyncAddResponse] or [SyncRemoveResponse]
///
/// **NOTE**: shows and seasons only get returned when adding ratings
///
/// [SyncAddResponse]: struct.SyncAddResponse.html
/// [SyncRemoveResponse]: struct.SyncRemoveResponse.html
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponseNumbers {
    movies: u32,
    episodes: u32,
    shows: Option<u32>,
    seasons: Option<u32>,
}

/// Not found entries in [SyncAddResponse] or [SyncRemoveResponse]
///
/// [SyncAddResponse]: struct.SyncAddResponse.html
/// [SyncRemoveResponse]: struct.SyncRemoveResponse.html
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponseNotFound {
    movies: Vec<OptionMovie>,
    shows: Vec<OptionShow>,
    seasons: Vec<OptionSeason>,
    episodes: Vec<OptionEpisode>,
}
