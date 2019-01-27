use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgress {
    pub aired: u32,
    pub completed: u32,
    pub last_collected_at: Option<DateTime<Utc>>,
    pub seasons: Vec<CollectionProgressSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgressSeason {
    pub number: u32,
    pub aired: u32,
    pub completed: u32,
    pub episodes: Vec<CollectionProgressEpisode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionProgressEpisode {
    pub number: u32,
    pub completed: bool,
    pub collected_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgress {
    pub aired: u32,
    pub completed: u32,
    pub last_watched_at: Option<DateTime<Utc>>,
    pub reset_at: Option<DateTime<Utc>>,
    pub seasons: Vec<WatchedProgressSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgressSeason {
    pub number: u32,
    pub aired: u32,
    pub completed: u32,
    pub episodes: Vec<WatchedProgressEpisode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedProgressEpisode {
    pub number: u32,
    pub completed: bool,
    pub last_watched_at: Option<DateTime<Utc>>,
}
