use crate::models::{Movie, Show};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionMovie {
    pub collected_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionShow {
    pub last_collected_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub show: Show,
    pub seasons: Vec<CollectionSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionSeason {
    pub number: u32,
    pub episodes: Vec<CollectionEpisode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionEpisode {
    pub number: u32,
    pub collected_at: DateTime<Utc>,
}
