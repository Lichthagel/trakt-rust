#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    movies: MovieStats,
    shows: ShowStats,
    seasons: SeasonStats,
    episodes: EpisodeStats,
    network: NetworkStats,
    ratings: RatingStats
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieStats {
    plays: u64,
    watched: u64,
    minutes: u64,
    collected: u64,
    ratings: u64,
    comments: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowStats {
    watched: u64,
    collected: u64,
    ratings: u64,
    comments: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonStats {
    ratings: u64,
    comments: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodeStats {
    plays: u64,
    watched: u64,
    minutes: u64,
    collected: u64,
    ratings: u64,
    comments: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    friends: u64,
    followers: u64,
    following: u64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatingStats {
    total: u64,
    distribution: [u64; 10]
}