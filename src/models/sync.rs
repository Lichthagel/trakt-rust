use crate::models::{Episode, Movie, Show, WatchableType};
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
