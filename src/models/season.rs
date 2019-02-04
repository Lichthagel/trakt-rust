use crate::models::ids::Ids;

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub number: u32,
    pub ids: Ids,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionSeason {
    pub number: Option<u32>,
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
            ids: None
        }
    }
}