pub mod rating_request;
pub mod sync_request;

pub use crate::models::sync::rating_request::{
    RatingFactory, RatingSeasonFactory, RatingShowFactory,
};
pub use crate::models::sync::sync_request::{
    SyncFactory, SyncSeasonFactory, SyncShowFactory, SyncType,
};

use crate::models::{
    Episode, Movie, OptionEpisode, OptionMovie, OptionSeason, OptionShow, Show, WatchableType,
};
use chrono::{DateTime, Utc};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncAddResponse {
    added: SyncResponseNumbers,
    updated: Option<SyncResponseNumbers>,
    existing: Option<SyncResponseNumbers>,
    not_found: SyncResponseNotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncRemoveResponse {
    deleted: SyncResponseNumbers,
    not_found: SyncResponseNotFound,
}

/// shows and seasons only get returned when adding ratings
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponseNumbers {
    movies: u32,
    episodes: u32,
    shows: Option<u32>,
    seasons: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncResponseNotFound {
    movies: Vec<OptionMovie>,
    shows: Vec<OptionShow>,
    seasons: Vec<OptionSeason>,
    episodes: Vec<OptionEpisode>,
}
