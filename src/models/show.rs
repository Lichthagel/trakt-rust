use crate::{
    error::Error,
    extended_info::{WithFull, WithNone},
    models::ids::Ids,
};
use chrono::{DateTime, Utc};
use serde::{de::Visitor, Deserializer};
use std::{fmt, ops::AddAssign, str::FromStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: Option<u16>,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Airing {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
}

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
