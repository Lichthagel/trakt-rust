//! All models related to [shows]
//!
//! [shows]: https://trakt.docs.apiary.io/#reference/shows
use crate::{
    error::Error,
    extended_info::{WithFull, WithNone},
    models::ids::Ids,
};
use chrono::{DateTime, Utc};
use serde::{de::Visitor, Deserializer};
use std::{fmt, ops::AddAssign, str::FromStr};

/// A [show]
///
/// [show]: https://trakt.docs.apiary.io/#reference/shows
#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
}

/// Airing information of a [show]. Used in [FullShow]
///
/// [show]: https://trakt.docs.apiary.io/#reference/shows
/// [FullShow]: struct.FullShow.html
#[derive(Debug, Serialize, Deserialize)]
pub struct Airing {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
}

/// Status of a [show]. Used in [FullShow]
///
/// [show]: https://trakt.docs.apiary.io/#reference/shows
/// [FullShow]: struct.FullShow.html
#[derive(Debug, Serialize, Deserialize)]
pub enum ShowStatus {
    #[serde(rename = "returning series")]
    Returning,
    #[serde(rename = "in production")]
    InProduction,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "ended")]
    Ended,
}

impl fmt::Display for ShowStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            ShowStatus::Returning => "returning series",
            ShowStatus::InProduction => "in production",
            ShowStatus::Planned => "planned",
            ShowStatus::Cancelled => "cancelled",
            ShowStatus::Ended => "ended",
        })
    }
}

impl FromStr for ShowStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "returning series" => Ok(ShowStatus::Returning),
            "in production" => Ok(ShowStatus::InProduction),
            "planned" => Ok(ShowStatus::Planned),
            "cancelled" => Ok(ShowStatus::Cancelled),
            "ended" => Ok(ShowStatus::Ended),
            _ => Err(Error::NoneError),
        }
    }
}

fn deserialize_status<'de, D>(deserializer: D) -> Result<Option<ShowStatus>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ShowStatusVisitor;

    impl<'de> Visitor<'de> for ShowStatusVisitor {
        type Value = Option<ShowStatus>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Option<ShowStatus>, E>
        where
            E: serde::de::Error,
        {
            match ShowStatus::from_str(v) {
                Ok(status) => Ok(Some(status)),
                Err(_) => Ok(None),
            }
        }
    }

    deserializer.deserialize_any(ShowStatusVisitor)
}

/// A [show] with full [extended info]
///
/// [show]: https://trakt.docs.apiary.io/#reference/shows
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullShow {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
    pub overview: Option<String>,
    pub first_aired: Option<DateTime<Utc>>,
    pub airs: Airing,
    pub runtime: u32,
    pub certification: Option<String>,
    pub network: Option<String>,
    pub country: Option<String>,
    pub trailer: Option<String>,
    pub homepage: Option<String>,
    #[serde(deserialize_with = "deserialize_status")]
    pub status: Option<ShowStatus>,
    pub rating: f64,
    pub votes: u32,
    pub comment_count: u32,
    pub updated_at: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub available_translations: Vec<String>,
    pub genres: Vec<String>,
    pub aired_episodes: u32,
}

impl WithFull for Show {
    type Full = FullShow;
}

impl WithNone for FullShow {
    type None = Show;
}

/// A [Show] with only optional fields
///
/// [Show]: struct.Show.html
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionShow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
            ids: None,
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

/// Info about a [trending show]
///
/// [trending show]: https://trakt.docs.apiary.io/#reference/shows/trending/get-trending-shows
#[derive(Debug, Serialize, Deserialize)]
pub struct ShowInfo {
    watchers: u32,
    show: Show,
}

/// Stats of a [show]
///
/// [show]: https://trakt.docs.apiary.io/#reference/shows/played/get-the-most-played-shows
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchedShow {
    watcher_count: u64,
    play_count: u64,
    collected_count: u64,
    collector_count: u64,
    show: Show,
}

/// An [anticipated show] that is not yet released but in some lists
///
/// [anticipated show]: https://trakt.docs.apiary.io/#reference/shows/anticipated/get-the-most-anticipated-shows
#[derive(Debug, Serialize, Deserialize)]
pub struct AnticipatedShow {
    list_count: u64,
    show: Show,
}

/// A [show] that got recently updated
///
/// [show]: https://trakt.docs.apiary.io/#reference/shows
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedShow {
    updated_at: DateTime<Utc>,
    show: Show,
}
