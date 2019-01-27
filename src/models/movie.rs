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
    pub watchers: u32,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedMovie {
    pub watcher_count: u64,
    pub play_count: u64,
    pub collected_count: u64,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnticipatedMovie {
    pub list_count: u64,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedMovie {
    pub updated_at: DateTime<Utc>,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionMovie {
    pub title: Option<String>,
    pub year: Option<u16>,
    pub ids: Option<Ids>,
}

impl OptionMovie {
    pub fn new(trakt_slug: String) -> Self {
        Self {
            title: None,
            year: None,
            ids: Some(Ids {
                trakt: None,
                slug: Some(trakt_slug),
                tvdb: None,
                imdb: None,
                tmdb: None,
                tvrage: None,
            }),
        }
    }
}

impl From<Movie> for OptionMovie {
    fn from(movie: Movie) -> Self {
        Self {
            title: Some(movie.title),
            year: movie.year,
            ids: Some(movie.ids),
        }
    }
}
