use crate::models::ids::Ids;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionShow {
    title: Option<String>,
    year: Option<u16>,
    ids: Option<Ids>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ShowStats {
    watchers: u64,
    plays: u64,
    collectors: u64,
    collected_episodes: u64,
    comments: u64,
    lists: u64,
    votes: u64,
}
