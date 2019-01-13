use chrono::{
    DateTime,
    Utc
};
use crate::models::{
    Movie,
    Show,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedEntry {
    plays: u32,
    last_watched_at: DateTime<Utc>,
    last_updated_at: Option<DateTime<Utc>>,
    movie: Option<Movie>,
    show: Option<Show>,
    seasons: Option<Vec<WatchedSeason>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedSeason {
    number: u32,
    episodes: Vec<WatchedEpisode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedEpisode {
    number: u32,
    plays: u32,
    last_watched_at: DateTime<Utc>,
}