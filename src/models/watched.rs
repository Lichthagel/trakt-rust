pub struct WatchedEntry {
    plays: u32,
    last_watched_at: DateTime,
    movie: Option<Movie>,
    show: Option<Show>,
    seasons: Option<Vector<WatchedSeason>>
}

pub struct WatchedSeason {
    number: u32,
    episodes: Vector<WatchedEpisode>
}

pub struct WatchedEpisode {
    number: u32,
    plays: u32,
    last_watched_at: DateTime
}