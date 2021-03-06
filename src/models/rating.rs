use crate::models::{ItemType, Movie, Season, Show};
use chrono::{DateTime, Utc};

/// A [rating]
///
/// [rating]: https://trakt.docs.apiary.io/#reference/users/ratings/get-ratings
#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    pub rated_at: DateTime<Utc>,
    pub rating: u8,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub season: Option<Season>,
    pub show: Option<Show>,
    pub movie: Option<Movie>,
}

/// [Ratings] for a movie/show/season/episode
///
/// [Ratings]: https://trakt.docs.apiary.io/#reference/movies/ratings/get-movie-ratings
#[derive(Debug, Serialize, Deserialize)]
pub struct Ratings {
    pub rating: f32,
    pub votes: u32,
    pub distribution: RatingDistribution,
}

/// The rating distribution in [Ratings]
///
/// [Ratings]: struct.Ratings.html
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
