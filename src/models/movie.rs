use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    title: String,
    year: u16,
    ids: Ids
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieInfo {
    watchers: u32,
    movie: Movie
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedMovie {
    watcher_count: u64,
    play_count: u64,
    collected_count: u64,
    movie: Movie
}