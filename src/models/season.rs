use crate::{
    extended_info::{WithFull, WithNone},
    models::ids::Ids,
};
use chrono::{DateTime, Utc};

/// A [season]
///
/// [season]: https://trakt.docs.apiary.io/#reference/seasons
#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub number: u32,
    pub ids: Ids,
}

/// A [season] with full [extended info]
///
/// [season]: https://trakt.docs.apiary.io/#reference/seasons
/// [extended info]: https://trakt.docs.apiary.io/#introduction/extended-info
#[derive(Debug, Serialize, Deserialize)]
pub struct FullSeason {
    pub number: u32,
    pub ids: Ids,
    pub rating: f64,
    pub votes: u32,
    pub episode_count: u32,
    pub aired_episodes: u32,
    pub title: String,
    pub overview: Option<String>,
    pub first_aired: DateTime<Utc>,
    pub network: String,
}

impl WithFull for Season {
    type Full = FullSeason;
}

impl WithNone for FullSeason {
    type None = Season;
}

/// A [Season] with only optional fields
///
/// [Season]: struct.Season.html
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionSeason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Ids>,
}

impl OptionSeason {
    pub fn new(season_number: u32) -> Self {
        Self {
            number: Some(season_number),
            ids: None,
        }
    }
}

impl From<Season> for OptionSeason {
    fn from(season: Season) -> Self {
        Self {
            number: Some(season.number),
            ids: Some(season.ids),
        }
    }
}

impl Default for OptionSeason {
    fn default() -> Self {
        Self {
            number: None,
            ids: None,
        }
    }
}
