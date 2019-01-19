#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    movies: UserMovieStats,
    shows: UserShowStats,
    seasons: UserSeasonStats,
    episodes: UserEpisodeStats,
    network: UserNetworkStats,
    ratings: UserRatingStats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMovieStats {
    plays: u64,
    watched: u64,
    minutes: u64,
    collected: u64,
    ratings: u64,
    comments: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserShowStats {
    watched: u64,
    collected: u64,
    ratings: u64,
    comments: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSeasonStats {
    ratings: u64,
    comments: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEpisodeStats {
    plays: u64,
    watched: u64,
    minutes: u64,
    collected: u64,
    ratings: u64,
    comments: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNetworkStats {
    friends: u64,
    followers: u64,
    following: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRatingStats {
    total: u64,
    distribution: [u64; 10],
}
