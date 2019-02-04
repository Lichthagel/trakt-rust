use crate::models::ids::Ids;
use chrono::{DateTime, Utc};
use std::ops::AddAssign;

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionShow {
    pub title: Option<String>,
    pub year: Option<u16>,
    pub ids: Option<Ids>,
}

impl OptionShow {
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

impl From<Show> for OptionShow {
    fn from(show: Show) -> Self {
        Self {
            title: Some(show.title),
            year: show.year,
            ids: Some(show.ids),
        }
    }
}

impl Default for OptionShow {
    fn default() -> Self {
        Self {
            title: None,
            year: None,
            ids: None
        }
    }
}

impl AddAssign for OptionShow {
    fn add_assign(&mut self, rhs: Self) {
        if let Some(title) = rhs.title {
            self.title = Some(title);
        }
        if let Some(year) = rhs.year {
            self.year = Some(year);
        }
        if let Some(ids) = rhs.ids {
            match &mut self.ids {
                Some(lids) => {
                    lids.trakt = lids.trakt.clone().or(ids.trakt);
                    lids.slug = lids.slug.clone().or(ids.slug);
                    lids.tvdb = lids.tvdb.clone().or(ids.tvdb);
                    lids.imdb = lids.imdb.clone().or(ids.imdb);
                    lids.tmdb = lids.tmdb.clone().or(ids.tmdb);
                    lids.tvrage = lids.tvrage.clone().or(ids.tvrage);
                }
                None => self.ids = Some(ids),
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowInfo {
    watchers: u32,
    show: Show,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedShow {
    watcher_count: u64,
    play_count: u64,
    collected_count: u64,
    collector_count: u64,
    show: Show,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnticipatedShow {
    list_count: u64,
    show: Show,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedShow {
    updated_at: DateTime<Utc>,
    show: Show,
}
