//! All models related to [episodes]
//!
//! [episodes]: https://trakt.docs.apiary.io/#reference/episodes
use crate::{
    extended_info::{WithFull, WithNone},
    models::ids::Ids,
};
use chrono::{DateTime, Utc};

/// An [episode]
///
/// [episode]: https://trakt.docs.apiary.io/#reference/episodes
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Episode {
    pub season: u32,
    pub number: u32,
    pub title: Option<String>,
    pub ids: Ids,
}

/// An [episode] with full [extended info]
///
/// [episode]: https://trakt.docs.apiary.io/#reference/episodes
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullEpisode {
    pub season: u32,
    pub number: u32,
    pub title: Option<String>,
    pub ids: Ids,
    pub number_abs: Option<u32>,
    pub overview: Option<String>,
    pub rating: f32,
    pub votes: u32,
    pub comment_count: u32,
    pub first_aired: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub available_translations: Vec<String>,
    pub runtime: u32,
}

impl WithFull for Episode {
    type Full = FullEpisode;
}

impl WithNone for FullEpisode {
    type None = Episode;
}

impl PartialEq for FullEpisode {
    fn eq(&self, other: &FullEpisode) -> bool {
        self.season == other.season
            && self.number == other.number
            && self.title == other.title
            && self.ids == other.ids
            && self.number_abs == other.number_abs
            && self.overview == other.overview
            && self.first_aired == other.first_aired
            && self.available_translations == other.available_translations
            && self.runtime == other.runtime
    }
}

/// An [Episode] with all fields optional
///
/// [Episode]: struct.Episode.html
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionEpisode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

impl OptionEpisode {
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

impl From<Episode> for OptionEpisode {
    fn from(ep: Episode) -> Self {
        Self {
            season: Some(ep.season),
            number: Some(ep.number),
            title: ep.title,
            ids: Some(ep.ids),
        }
    }
}

impl Default for OptionEpisode {
    fn default() -> Self {
        Self {
            season: None,
            number: None,
            title: None,
            ids: None,
        }
    }
}
