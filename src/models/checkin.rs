use crate::models::{Episode, Ids, Movie, Show};
use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkin {
    pub movie: Option<CheckinMovie>,
    pub show: Option<CheckinShow>,
    pub episode: Option<CheckinEpisode>,
    pub sharing: CheckinSharing,
    pub message: String,
    pub app_version: String,
    pub app_date: NaiveDate,
}

impl Checkin {
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinMovie {
    title: Option<String>,
    year: Option<u16>,
    ids: Option<Ids>,
}

impl CheckinMovie {
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

impl From<Movie> for CheckinMovie {
    fn from(movie: Movie) -> Self {
        Self {
            title: Some(movie.title),
            year: movie.year,
            ids: Some(movie.ids),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinShow {
    title: Option<String>,
    year: Option<u16>,
    ids: Option<Ids>,
}

impl CheckinShow {
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

impl From<Show> for CheckinShow {
    fn from(show: Show) -> Self {
        Self {
            title: Some(show.title),
            year: show.year,
            ids: Some(show.ids),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinEpisode {
    season: Option<u32>,
    number: Option<u32>,
    title: Option<String>,
    ids: Option<Ids>,
}

impl CheckinEpisode {
    pub fn new(trakt_id: u64) -> Self {
        Self {
            season: None,
            number: None,
            title: None,
            ids: Some(Ids {
                trakt: Some(trakt_id),
                slug: None,
                tvdb: None,
                imdb: None,
                tmdb: None,
                tvrage: None,
            }),
        }
    }

    pub fn by_season(season_number: u32, episode_number: u32) -> Self {
        Self {
            season: Some(season_number),
            number: Some(episode_number),
            title: None,
            ids: None,
        }
    }
}

impl From<Episode> for CheckinEpisode {
    fn from(ep: Episode) -> Self {
        Self {
            season: Some(ep.season),
            number: Some(ep.number),
            title: ep.title,
            ids: Some(ep.ids),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinSharing {
    twitter: bool,
    tumblr: bool,
    facebook: bool,
}

impl CheckinSharing {
    pub fn new(twitter: bool, tumblr: bool, facebook: bool) -> CheckinSharing {
        CheckinSharing {
            twitter,
            tumblr,
            facebook,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckinResponse {
    id: u64,
    watched_at: DateTime<Utc>,
    sharing: CheckinSharing,
    movie: Option<Movie>,
    episode: Option<Episode>,
    show: Option<Show>,
}
