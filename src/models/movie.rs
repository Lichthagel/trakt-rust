use crate::{
    extended_info::{WithFull, WithNone},
    models::Ids,
};
use chrono::{DateTime, NaiveDate, Utc};
use std::ops::AddAssign;

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullMovie {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
    pub tagline: String,
    pub overview: String,
    pub released: NaiveDate,
    pub runtime: u32,
    pub country: Option<String>,
    pub trailer: Option<String>,
    pub homepage: Option<String>,
    pub rating: f64,
    pub votes: u32,
    pub comment_count: u32,
    pub updated_at: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub available_translations: Vec<String>,
    pub genres: Vec<String>,
    pub certification: Option<String>,
}

impl WithFull for Movie {
    type Full = FullMovie;
}

impl WithNone for FullMovie {
    type None = Movie;
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

impl Default for OptionMovie {
    fn default() -> Self {
        Self {
            title: None,
            year: None,
            ids: None,
        }
    }
}

impl AddAssign for OptionMovie {
    fn add_assign(&mut self, rhs: OptionMovie) {
        if let Some(title) = rhs.title {
            self.title = Some(title);
        }
        if let Some(year) = rhs.year {
            self.year = Some(year);
        }
        if let Some(ids) = rhs.ids {
            match &mut self.ids {
                Some(lids) => {
                    lids.trakt = lids.trakt.or(ids.trakt);
                    lids.slug = lids.slug.clone().or(ids.slug);
                    lids.tvdb = lids.tvdb.or(ids.tvdb);
                    lids.imdb = lids.imdb.clone().or(ids.imdb);
                    lids.tmdb = lids.tmdb.or(ids.tmdb);
                    lids.tvrage = lids.tvrage.or(ids.tvrage);
                }
                None => self.ids = Some(ids),
            }
        }
    }
}
