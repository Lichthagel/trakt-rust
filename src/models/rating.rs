use crate::models::{ItemType, Movie, Season, Show};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    rated_at: DateTime<Utc>,
    rating: u8,
    #[serde(rename = "type")]
    item_type: ItemType,
    season: Option<Season>,
    show: Option<Show>,
    movie: Option<Movie>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ratings {
    rating: f32,
    votes: u32,
    distribution: RatingDistribution,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatingDistribution {
    #[serde(rename = "1")]
    one: u32,
    #[serde(rename = "2")]
    two: u32,
    #[serde(rename = "3")]
    three: u32,
    #[serde(rename = "4")]
    four: u32,
    #[serde(rename = "5")]
    five: u32,
    #[serde(rename = "6")]
    six: u32,
    #[serde(rename = "7")]
    seven: u32,
    #[serde(rename = "8")]
    eight: u32,
    #[serde(rename = "9")]
    nine: u32,
    #[serde(rename = "10")]
    ten: u32,
}
