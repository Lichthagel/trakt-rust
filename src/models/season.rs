use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub number: u32,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionSeason {
    pub number: Option<u32>,
    pub ids: Ids,
}

impl OptionSeason {
    pub fn new(trakt_id: u64) -> Self {
        Self {
            number: None,
            ids: Ids {
                trakt: Some(trakt_id),
                slug: None,
                tvdb: None,
                imdb: None,
                tmdb: None,
                tvrage: None,
            },
        }
    }
}

impl From<Season> for OptionSeason {
    fn from(season: Season) -> Self {
        Self {
            number: Some(season.number),
            ids: season.ids,
        }
    }
}
