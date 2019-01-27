use crate::models::{Movie, Show};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedEntry {
    pub plays: u32,
    pub last_watched_at: DateTime<Utc>,
    pub last_updated_at: Option<DateTime<Utc>>,
    pub movie: Option<Movie>,
    pub show: Option<Show>,
    pub seasons: Option<Vec<WatchedSeason>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedSeason {
    pub number: u32,
    pub episodes: Vec<WatchedEpisode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedEpisode {
    pub number: u32,
    pub plays: u32,
    pub last_watched_at: DateTime<Utc>,
}
