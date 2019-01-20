use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgress {
    aired: u32,
    completed: u32,
    last_collected_at: Option<DateTime<Utc>>,
    seasons: Vec<CollectionProgressSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgressSeason {
    number: u32,
    aired: u32,
    completed: u32,
    episodes: Vec<CollectionProgressEpisode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgressEpisode {
    number: u32,
    completed: bool,
    collected_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgress {
    aired: u32,
    completed: u32,
    last_watched_at: Option<DateTime<Utc>>,
    reset_at: Option<DateTime<Utc>>,
    seasons: Vec<WatchedProgressSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgressSeason {
    number: u32,
    aired: u32,
    completed: u32,
    episodes: Vec<WatchedProgressEpisode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgressEpisode {
    number: u32,
    completed: bool,
    last_watched_at: Option<DateTime<Utc>>,
}
