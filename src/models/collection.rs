use crate::models::{Movie, Show};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionMovie {
    collected_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    movie: Movie,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionShow {
    last_collected_at: DateTime<Utc>,
    last_updated_at: DateTime<Utc>,
    show: Show,
    seasons: Vec<CollectionSeason>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionSeason {
    number: u32,
    episodes: Vec<CollectionEpisode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionEpisode {
    number: u32,
    collected_at: DateTime<Utc>,
}
