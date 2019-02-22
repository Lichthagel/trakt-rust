/// [Stats] of a user
///
/// [Stats]: https://trakt.docs.apiary.io/#reference/users/stats/get-stats
#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub movies: UserMovieStats,
    pub shows: UserShowStats,
    pub seasons: UserSeasonStats,
    pub episodes: UserEpisodeStats,
    pub network: UserNetworkStats,
    pub ratings: UserRatingStats,
}

/// See [UserStats]
///
/// [UserStats]: struct.UserStats.html
#[derive(Debug, Serialize, Deserialize)]
pub struct UserMovieStats {
    pub plays: u64,
    pub watched: u64,
    pub minutes: u64,
    pub collected: u64,
    pub ratings: u64,
    pub comments: u64,
}

/// See [UserStats]
///
/// [UserStats]: struct.UserStats.html
#[derive(Debug, Serialize, Deserialize)]
pub struct UserShowStats {
    pub watched: u64,
    pub collected: u64,
    pub ratings: u64,
    pub comments: u64,
}

/// See [UserStats]
///
/// [UserStats]: struct.UserStats.html
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSeasonStats {
    pub ratings: u64,
    pub comments: u64,
}

/// See [UserStats]
///
/// [UserStats]: struct.UserStats.html
#[derive(Debug, Serialize, Deserialize)]
pub struct UserEpisodeStats {
    pub plays: u64,
    pub watched: u64,
    pub minutes: u64,
    pub collected: u64,
    pub ratings: u64,
    pub comments: u64,
}

/// See [UserStats]
///
/// [UserStats]: struct.UserStats.html
#[derive(Debug, Serialize, Deserialize)]
pub struct UserNetworkStats {
    pub friends: u64,
    pub followers: u64,
    pub following: u64,
}

/// See [UserStats]
///
/// [UserStats]: struct.UserStats.html
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRatingStats {
    pub total: u64,
    pub distribution: [u64; 10],
}

/// Stats of a [movie] or [show]
///
/// [movie]: https://trakt.docs.apiary.io/#reference/movies/stats/get-movie-stats
/// [show]: https://trakt.docs.apiary.io/#reference/shows/stats/get-show-stats
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaStats {
    pub watchers: u64,
    pub plays: u64,
    pub collectors: u64,
    pub collected_episodes: Option<u64>,
    pub comments: u64,
    pub lists: u64,
    pub votes: u64,
}
