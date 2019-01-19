use crate::models::ids::Ids;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieInfo {
    watchers: u32,
    movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedMovie {
    watcher_count: u64,
    play_count: u64,
    collected_count: u64,
    movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnticipatedMovie {
    list_count: u64,
    movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedMovie {
    updated_at: DateTime<Utc>,
    movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieStats {
    watchers: u64,
    plays: u64,
    collectors: u64,
    comments: u64,
    lists: u64,
    votes: u64,
}
